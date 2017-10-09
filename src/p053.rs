// TODO
//
// Choose is symetric about it's center and is usually (always?)
// greatest around the center so we can cut the work in half.
//
//
// We can start around the center and work from there outwards
// to decide whether we can find a value > 1_000_000.
// This works because (n - r)!r! <= (n - r + 1)!(r - 1)! // TODO(orglofch): Is this true?
//
// We also know that (n + 1)! / ((n + 1 - r)!r!) = (n + 1) * n! / ((n + 1 - r) * (n - r)!r!)
// when transistioning between values of n. So we could technically just use a table
// and work upwards. Or potentially use a table and work downwards.
