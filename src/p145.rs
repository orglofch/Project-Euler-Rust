// TODO
//
// We can skip all numbers which have all zero suffixes since those
// would result in leading zeros.
//
// Since numbers always have to be odd, we know that either the
// first number or the second number need to be odd. This concept
// can be carried into the inner numbers as well, building inwards.

fn reversible_numbers() -> u64 {

}

fn main() {
    println!("There are {} reversible numbers", reversible_numbers());
}
