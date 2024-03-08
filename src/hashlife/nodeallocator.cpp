#include "nodeallocator.h"

NodeAllocator::NodeAllocator() {
    _nodes = std::make_unique<Node[]>(MAX_NODES);
}