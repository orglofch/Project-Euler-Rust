// TODO:
//
// There are basically 4 different conditions here:
// 1) Origin is the 90 degree angle.
// 2) Point P is the 90 dgree angle.
// 3) Point Q is the 90 degree angle.
// 4) When it forms a weird right angle triangle on a diagonal.
// 
// For a square of unit width n:
// 1) The number of triangles with origin as the 90 degree angle
//    is the number of ways to choose p and q where p is along
//    the y-axis (n) ways and q is along the x-axis (n) ways.
//    This is equal to n^2.
// 2) The number of triangles with P as the origin is also
//    n^2 using similar logic.
// 3) The number of triangles with Q as the origin is also
//    n^2 using similar logic.
// 3) The number of weird right angle triangle is 2 times
//    (one for each orientation) the number of ways to
//    form a triangle of ways to have either p or q touching
//    a point along the diagonal which is == n
//    except in the case where n == 1.
//
// Therefore, the final solution should be:
// 3 * n^2 + n
