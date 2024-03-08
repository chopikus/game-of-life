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
    using NodePtr = std::shared_ptr<Node>;
    uint8_t k; // level
    std::shared_ptr<Node> a,b,c,d;
    int64_t n; /* number of on nodes */
    uint64_t hash; // precomputed

    Node(uint8_t k, 
         NodePtr a, NodePtr b, NodePtr c, NodePtr d,
         int64_t n,
         uint64_t hash);

    auto operator<=>(const Node&) const = default;
};


using NodePtr = std::shared_ptr<Node>;
using NodePtrAndStep = std::pair<NodePtr, uint8_t>;
using FourNodePtr = std::tuple<NodePtr, NodePtr, NodePtr, NodePtr>;

struct NodePtrHash {
  std::size_t operator()(const NodePtr& t) const;
};

struct NodePtrAndStepHash {
  std::size_t operator()(const NodePtrAndStep& t) const;
};

struct FourNodePtrHash {
  std::size_t operator()(const FourNodePtr& t) const;
};