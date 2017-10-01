// DONE: 73162890
//
// This problem is solvable via a topological sort
// approach where dependencies are ordering dependencies and
// the minimum possible passcode is just the topological sorted order.
//
// Looking at the text file, there are so few tree's that we might as
// well just do it by hand, since it's relatively simple to express
// the text file as a graph and perform post-order DFS on it.
