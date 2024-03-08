#include "units.h"

Node::Node(uint8_t k, 
           NodePtr a, NodePtr b, NodePtr c, NodePtr d,
           int64_t n,
           uint64_t hash) : 
           k(k), a(a), b(b), c(c), d(d), n(n), hash(hash) {};

std::size_t NodePtrHash::operator()(const NodePtr& t) const {
    return t->hash;
}

std::size_t NodePtrAndStepHash::operator()(const NodePtrAndStep& t) const {
    return t.first->hash * 202481 + t.second;
}

std::size_t FourNodePtrHash::operator()(const FourNodePtr& t) const {
    const auto& [a, b, c, d] = t;
    return a->hash * 345547 + b->hash * 355573 + c->hash * 279329 + d->hash * 357263;
}