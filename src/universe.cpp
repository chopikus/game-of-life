#include <algorithm>
#include <emscripten/bind.h>
#include "lib/json.hpp"
#include "universe.h"
#include <utility>

#include <iostream>

Universe::Universe(const std::vector<Cell>& cells) {
    this->alive_cells_row = {};
    for (const auto& [x, y] : cells) {
        auto& set = this->alive_cells_row[y];
        set.insert(x);
    }
}

Universe Universe::example() {
    return Universe({{1, 0}, {2, 1}, {0, 2}, {1, 2}, {2, 2}});
}

bool Universe::is_cell_alive(int64_t x, int64_t y) {
    auto it = alive_cells_row.find(y);
    if (it == alive_cells_row.end()) {
        return false;
    }
    return it->second.contains(x);
}

bool Universe::tick_one_cell(bool is_alive, int alive_neighbours) {
    return ((!is_alive && alive_neighbours == 3)
            ||
            (is_alive && alive_neighbours >= 2 && alive_neighbours <= 3));
}

void Universe::tick() {
    std::set<int64_t> rows_to_compute;
    int64_t min_column = 0;
    int64_t max_column = 0;
    for (const auto& [row, columns] : alive_cells_row) {
        rows_to_compute.insert(row - 1);
        rows_to_compute.insert(row);
        rows_to_compute.insert(row + 1);
        if (columns.empty())
            continue;
        min_column = std::min(min_column, *(columns.begin()));
        max_column = std::max(max_column, *(columns.rbegin()));
    }

    std::vector<std::pair<Cell, bool> > changes;
    for (int64_t y : rows_to_compute) {
        for (int64_t x = min_column - 1; x <= max_column + 1; x++) {
            bool was_alive = is_cell_alive(x, y);
            int alive_neighbours = 0;
            for (int dx = -1; dx <= 1; dx++) {
                for (int dy = -1; dy <= 1; dy++) {
                    if (dx == 0 && dy == 0) {
                        continue;
                    }
                    alive_neighbours += is_cell_alive(x+dx, y+dy);
                }
            }
            bool is_alive = tick_one_cell(was_alive, alive_neighbours);
            if (!was_alive && is_alive) {
                changes.push_back({{x, y}, true});
            } else if (was_alive && !is_alive) {
                changes.push_back({{x, y}, false});
            }
        }
    }
    for (auto [cell, alive] : changes) {
        if (alive)
            alive_cells_row[cell.y].insert(cell.x);
        else
            alive_cells_row[cell.y].erase(cell.x);
    }
}

nlohmann::json Universe::alive_cells() {
    auto result = nlohmann::json::array();
    for (const auto& [y, columns]: alive_cells_row) {
        for (const auto& x : columns) {
            result.push_back({{"x", x}, {"y", y}});
        }
    }
    return result;
}

std::string Universe::alive_cells_str() {
    return alive_cells().dump();
}

EMSCRIPTEN_BINDINGS(universe) {
    emscripten::class_<Universe>("Universe")
        .constructor<const std::vector<Cell>&>()
        .class_function("example", &Universe::example)
        .function("tick", &Universe::tick)
        .function("alive_cells_str", &Universe::alive_cells_str);
}

using NodePtr = Hashlife::NodePtr;

const NodePtr Hashlife::on = std::make_shared<Node>(0, nullptr, nullptr, 
                                                    nullptr, nullptr, 1, 1);
const NodePtr Hashlife::off = std::make_shared<Node>(0, nullptr, nullptr, 
                                                     nullptr, nullptr, 0, 0);

NodePtr Hashlife::get_zero(uint8_t k) {
    if (k == 0) {
        return off;
    }
    
    if (zero_cache.exists(k))
        return zero_cache.get(k);
    
    auto result = join(
                    get_zero(k-1), get_zero(k-1), 
                    get_zero(k-1), get_zero(k-1));
    zero_cache.put(k, result);
    return result;
}

NodePtr Hashlife::join(const NodePtr& a, const NodePtr& b, 
                    const NodePtr& c, const NodePtr& d) {
    int64_t n = a->n + b->n + c->n + d->n;
    uint64_t hash = a->k + 2;
    hash += 1001933ull * a->hash;
    hash += 2331061ull * b->hash;
    hash += 6281573ull * c->hash;
    hash += 4591507ull * d->hash;
    hash &= 4294967295ull;
    if (join_cache.exists(hash)) {
        return join_cache.get(hash);
    } else {
        auto node = std::make_shared<Node>(
                         static_cast<uint8_t>(a->k + 1),
                         a,
                         b,
                         c,
                         d,
                         a->n + b->n + c->n + d->n,
                         hash
                    );
        join_cache.put(hash, node);
        return node;
    }
}

NodePtr Hashlife::centre(const NodePtr& m) {
    auto z = get_zero(m->k - 1);
    return join(join(z, z, z, m->a),
                join(z, z, m->b, z),
                join(z, m->c, z, z),
                join(m->d, z, z, z));
}

NodePtr Hashlife::life_3x3(
    const NodePtr& a, const NodePtr& b, const NodePtr& c,
    const NodePtr& d, const NodePtr& E, const NodePtr& f,
    const NodePtr& g, const NodePtr& h, const NodePtr& i) {

    int64_t outer_on_cells = a->n + b->n + c->n +
                             d->n + f->n +
                             g->n + h->n + i->n ;

    if ((outer_on_cells == 2 && E->n) || outer_on_cells == 3)
        return on;
    else
        return off;
};

NodePtr Hashlife::life_4x4(const NodePtr& m) {
    NodePtr ab = life_3x3(m->a->a, m->a->b, m->b->a, 
                          m->a->c, m->a->d, m->b->c,
                          m->c->a, m->c->b, m->d->a);
    
    NodePtr bc = life_3x3(m->a->b, m->b->a, m->b->b, 
                          m->a->d, m->b->c, m->b->d,
                          m->c->b, m->d->a, m->d->b);

    NodePtr cb = life_3x3(m->a->c, m->a->d, m->b->c,
                          m->c->a, m->c->b, m->d->a,
                          m->c->c, m->c->d, m->d->c);
    
    NodePtr da = life_3x3(m->a->d, m->b->c, m->b->d,
                          m->c->b, m->d->a, m->d->b,
                          m->c->d, m->d->c, m->d->d);

    return join(ab, bc, cb, da);
}

NodePtr Hashlife::successor(const NodePtr& m, uint8_t j) {
    if (m->n == 0)
        return m->a;
    if (m->k == 1)
        return m;
    if (m->k == 2)
        return life_4x4(m);
    
    if (j >= m->k - 2)
        j = m->k - 2;

    auto c1 = successor(join(m->a->a, m->a->b, m->a->c, m->a->d), j);
    auto c2 = successor(join(m->a->b, m->b->a, m->a->d, m->b->c), j);
    auto c3 = successor(join(m->b->a, m->b->b, m->b->c, m->b->d), j);
    auto c4 = successor(join(m->a->c, m->a->d, m->c->a, m->c->b), j);
    auto c5 = successor(join(m->a->d, m->b->c, m->c->b, m->d->a), j);
    auto c6 = successor(join(m->b->c, m->b->d, m->d->a, m->d->b), j);
    auto c7 = successor(join(m->c->a, m->c->b, m->c->c, m->c->d), j);
    auto c8 = successor(join(m->c->b, m->d->a, m->c->d, m->d->c), j);
    auto c9 = successor(join(m->d->a, m->d->b, m->d->c, m->d->d), j);

    if (j < m->k - 2) {
        return 
            join(
                join(c1->d, c2->c, c4->b, c5->a),
                join(c2->d, c3->c, c5->b, c6->a),
                join(c4->d, c5->c, c7->b, c8->a),
                join(c5->d, c6->c, c8->b, c9->a)
            );
    } else {
        return
            join(
                successor(join(c1, c2, c4, c5), j),
                successor(join(c2, c3, c5, c6), j),
                successor(join(c4, c5, c7, c8), j),
                successor(join(c5, c6, c8, c9), j)
            );
    };
}
