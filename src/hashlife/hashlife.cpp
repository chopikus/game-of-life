#include "hashlife.h"

Hashlife::Hashlife(const std::vector<Cell>& active_cells) {
    _off = _allocator.new_node(0, nullptr, nullptr, nullptr, nullptr, 0, 0);
    _on = _allocator.new_node(0, nullptr, nullptr, nullptr, nullptr, 1, 1);

    if (active_cells.empty())
    {
        _root = _off;
        return;
    }

    int64_t min_x = active_cells[0].x;
    int64_t min_y = active_cells[0].y;
    for (auto c : active_cells) {
        if (c.x < min_x)
            min_x = c.x;
        if (c.y < min_y)
            min_y = c.y;    
    }

    _construct_root(min_x, min_y, active_cells);

    _fix_x = min_x;
    _fix_y = min_y;

    /* The size of the world is 2^29, 
       computing up to 2^27 generations in one go.
       Last wrapping is used to compute successors. */
    while (_root->k < 29) {
        _fix_x += (1 << (_root->k - 1));
        _fix_y += (1 << (_root->k - 1));

        _root = _centre(_root);
    }
}

void Hashlife::_construct_root(int64_t min_x, int64_t min_y, const std::vector<Cell>& active_cells) {
    std::map<Cell, NodePtr> cur;
    for (auto c : active_cells) {
        /* changing coordinates so that all coordinates are >=0*/
        cur.insert({Cell{c.x-min_x, c.y-min_y}, _on});
    }

    size_t k = 0;
    while (cur.size() > 1) {
        auto z = _get_zero(k);
        auto pop = [&](int64_t x, int64_t y) -> NodePtr {
            auto it = cur.find({x, y});
            if (it == cur.end())
                return z;
            else {
                auto result = it->second;
                cur.erase({x, y});
                return result;
            }
        };

        std::map<Cell, NodePtr> next_level;
        while (cur.size() > 0) {
            auto [x, y] = cur.begin()->first;
            
            x -= x & 1;
            y -= y & 1;
            NodePtr a = pop(x, y);
            NodePtr b = pop(x+1, y);
            NodePtr c = pop(x, y+1);
            NodePtr d = pop(x+1, y+1);
            NodePtr r = _join(a, b, c, d);
            next_level.insert({{x/2, y/2}, _join(a,b,c,d)});
        }
        cur.clear();
        cur = std::move(next_level);
        k+=1;
    }

    _root = cur.begin()->second;
}

NodePtr Hashlife::_get_zero(uint8_t k) {
    if (k == 0) {
        return _off;
    }
    
    if (_zero_cache.exists(k))
        return _zero_cache.get(k);
    
    auto result = _join(
                    _get_zero(k-1), _get_zero(k-1), 
                    _get_zero(k-1), _get_zero(k-1));
    _zero_cache.put(k, result);
    return result;
}

NodePtr Hashlife::_join(NodePtr a, NodePtr b, 
                        NodePtr c, NodePtr d) {
    FourNodePtr t{a, b, c, d};
    if (_join_cache.exists(t)) {
        return _join_cache.get(t);
    } else {
        uint64_t hash = a->k * 784753ull;
        hash += a->n + b->n + c->n + d->n;
        hash += 616207 * a->hash;
        hash += 990037 * b->hash;
        hash += 599383 * c->hash;
        hash += 482263 * d->hash;
        auto node = _allocator.new_node(
                        static_cast<uint8_t>(a->k + 1),
                        a,
                        b,
                        c,
                        d,
                        a->n + b->n + c->n + d->n,
                        hash
                    );
        _join_cache.put(t, node);
        return node;
    }
}

NodePtr Hashlife::_centre(NodePtr m) {
    auto z = _get_zero(m->k - 1);
    return _join(_join(z, z, z, m->a),
                 _join(z, z, m->b, z),
                 _join(z, m->c, z, z),
                 _join(m->d, z, z, z));
}

NodePtr Hashlife::_life_3x3(
    NodePtr a, NodePtr b, NodePtr c,
    NodePtr d, NodePtr E, NodePtr f,
    NodePtr g, NodePtr h, NodePtr i) {

    int64_t outer_on_cells = a->n + b->n + c->n +
                             d->n + f->n +
                             g->n + h->n + i->n ;

    if ((outer_on_cells == 2 && E->n) || outer_on_cells == 3)
        return _on;
    else
        return _off;
};

NodePtr Hashlife::_life_4x4(NodePtr m) {
    
    if (_life_4x4_cache.exists(m))
        return _life_4x4_cache.get(m);
    NodePtr ab = _life_3x3(m->a->a, m->a->b, m->b->a, 
                          m->a->c, m->a->d, m->b->c,
                          m->c->a, m->c->b, m->d->a);
    
    NodePtr bc = _life_3x3(m->a->b, m->b->a, m->b->b, 
                          m->a->d, m->b->c, m->b->d,
                          m->c->b, m->d->a, m->d->b);

    NodePtr cb = _life_3x3(m->a->c, m->a->d, m->b->c,
                          m->c->a, m->c->b, m->d->a,
                          m->c->c, m->c->d, m->d->c);
    
    NodePtr da = _life_3x3(m->a->d, m->b->c, m->b->d,
                          m->c->b, m->d->a, m->d->b,
                          m->c->d, m->d->c, m->d->d);

    auto result = _join(ab, bc, cb, da);
    _life_4x4_cache.put(m, result);

    return result;
}

void Hashlife::successor(uint8_t j) {
    _root = _centre(_successor(_root, j));
}

NodePtr Hashlife::_successor(NodePtr m, uint8_t j) {
    if (m->n == 0) {
        return m->a;
    }

    if (m->k == 1)
        return m;
    
    if (m->k == 2) {
        auto result = _life_4x4(m);
        return result;
    }

    if (j > m->k - 2)
        j = m->k - 2;

    if (_successor_cache.exists({m, j}))
        return _successor_cache.get({m, j});
    
    auto c1 = _successor(m->a, j);
    auto c2 = _successor(_join(m->a->b, m->b->a, m->a->d, m->b->c), j);
    auto c3 = _successor(m->b, j);
    auto c4 = _successor(_join(m->a->c, m->a->d, m->c->a, m->c->b), j);
    auto c5 = _successor(_join(m->a->d, m->b->c, m->c->b, m->d->a), j);
    auto c6 = _successor(_join(m->b->c, m->b->d, m->d->a, m->d->b), j);
    auto c7 = _successor(m->c, j);
    auto c8 = _successor(_join(m->c->b, m->d->a, m->c->d, m->d->c), j);
    auto c9 = _successor(m->d, j);

    if (j < m->k - 2) {
        auto result = _join(
                           _join(c1->d, c2->c, c4->b, c5->a),
                           _join(c2->d, c3->c, c5->b, c6->a),
                           _join(c4->d, c5->c, c7->b, c8->a),
                           _join(c5->d, c6->c, c8->b, c9->a)
                       );
        _successor_cache.put({m, j}, result);
        return result;
    } else {
        auto result = _join(
                           _successor(_join(c1, c2, c4, c5), j),
                           _successor(_join(c2, c3, c5, c6), j),
                           _successor(_join(c4, c5, c7, c8), j),
                           _successor(_join(c5, c6, c8, c9), j)
                       );
        _successor_cache.put({m, j}, result);
        return result;
    };
}

void Hashlife::_append_alive_cells(NodePtr node, std::vector<Cell>& output,
                                  uint8_t level, 
                                  int64_t x, int64_t y,
                                  int64_t min_x, int64_t min_y,
                                  int64_t max_x, int64_t max_y) {

    /* return alive cells from (min_x, min_y) up to (max_x, max_y) included.*/
    /* (x, y) is the top-left point of node */
    if (node->n == 0) {
        return;
    }

    int64_t size = 1 << (node->k);
    if (x + size <= min_x || x > max_x)
        return;
    if (y + size <= min_y || y > max_y)
        return;
    
    if (node->k == level) {
        output.push_back({x-_fix_x, y-_fix_y});
        return;
    }

    int64_t offset = size >> 1;
    _append_alive_cells(node->a, output, level, x, y, min_x, min_y, max_x, max_y);
    _append_alive_cells(node->b, output, level, x + offset, y, min_x, min_y, max_x, max_y);
    _append_alive_cells(node->c, output, level, x, y + offset, min_x, min_y, max_x, max_y);
    _append_alive_cells(node->d, output, level, x + offset, y + offset, min_x, min_y, max_x, max_y);
}

std::vector<Cell> Hashlife::get_alive_cells(uint8_t level,
                                            int64_t min_x, int64_t min_y,
                                            int64_t max_x, int64_t max_y) {
    min_x += _fix_x;
    min_y += _fix_y;

    max_x += _fix_x;
    max_y += _fix_y;

    std::vector<Cell> output;
    _append_alive_cells(_root, output, level, 0, 0, min_x, min_y, max_x, max_y);

    return std::move(output);
}