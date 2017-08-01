// This problem can be solved somewhat trivially by a greedy algorithm
// which calculates the minimum spanning tree of the network.
//
// The more tricky parts are turning the matrix into a better sparse
// form to improve performance.
//
// The first idea is to maintain a list of strongly connected
// components for each node in the graph:
// E.g.
// A -> [A]
// ...
// G -> [G]
//
// and a sparse list of all edges, indexed by the nodes id.
// A -> [B, C, D]
// ,,,
// G -> [E, D, F]
//
// Each iteration:
// 1) We walk the set of edges and select the edge with least cost [S <-> T]
// 2) Remove each edge in S connected to an element in T's component.
// 2) Update the strongly connected components for S and T.
//
// We can actually store and edge as either in S or T which
// removes the ability to step over an edge twice in step (1) at the cost
// of needing to remove all edges [S -> T] and all edges [T -> S] in (2).


