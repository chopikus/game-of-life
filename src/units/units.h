#pragma once
#include <memory>

struct Cell {
    int64_t x;
    int64_t y;

    auto operator<=>(const Cell&) const = default;
};

struct Node {
    /* ____
      /    \
      |A  B|
      |C  D|
      \___/
    */
    uint8_t kn; // first bit n, rest bits -- k
    std::shared_ptr<Node> a,b,c,d;
    uint64_t hash; // precomputed

    auto operator<=>(const Node&) const = default;
    
    uint8_t k() const;
    bool n() const;
};


using NodePtr = std::shared_ptr<Node>;
using NodePtrAndStep = std::pair<NodePtr, uint8_t>;
using FourNodePtr = std::tuple<NodePtr, NodePtr, NodePtr, NodePtr>;

struct NodePtrHash {
  std::size_t operator()(NodePtr t) const;
};

struct NodePtrAndStepHash {
  std::size_t operator()(const NodePtrAndStep& t) const;
};

struct FourNodePtrHash {
  std::size_t operator()(const FourNodePtr& t) const;
};