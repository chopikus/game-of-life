#include "units.h"

bool Node::n() const {
    return kn & (1 << 7);
}

uint8_t Node::k() const {
    return kn & ((1 << 7) - 1);
}

std::size_t NodePtrHash::operator()(NodePtr t) const {
    return t->hash;
}

std::size_t NodePtrAndStepHash::operator()(const NodePtrAndStep& t) const {
    return t.first->hash * 202481 + t.second;
}

std::size_t FourNodePtrHash::operator()(const FourNodePtr& t) const {
    const auto& [a, b, c, d] = t;
    return a->hash * 345547 + b->hash * 355573 + c->hash * 279329 + d->hash * 357263;
}