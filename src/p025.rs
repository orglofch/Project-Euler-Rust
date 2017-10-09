// DONE: 4782
//
// We can use the  closed form solution to generate fibonacci numbers instead
// of of iterating.
//
// Fib(n) = (Phi^n - phi^n) / 5^(1/2)
//
//
// Phi = (1 + sqrt(5)) / 2
// phi = (1 - sqrt(5)) / 2
//
// We can find the number of digit using log_10.
//
// log_10(Fib(n)) = log_10((Phi^n - phi^n) / sqrt(5))
//                = log_10(Phi^n - phi^n) - log_10(sqrt(5)
// We can't split out the larger log_10 to solve for n, but we can if we
// use the approximation:
//
// Fib(n) = round(Phi^n / sqrt(5))
//
// log_10(Fib(n)) ~= log_10(Phi^n / sqrt(5))
//                ~= n * log_10(Phi) - log_10(sqrt(5))
//                ~= n * log_10(Phi) - log_10(5)) / 2
//
// =>  999 < n * log_10(Phi) - log_10(sqrt(5))
// =>    n > (999 + log_10(5) / 2)) / log_10(Phi)

fn fib_index_1000() -> f64 {
    return ((999.0 + (5_f64).log(10.0) / 2.0) / ((1.0 + (5_f64).sqrt()) / 2.0).log(10.0)).ceil();
}

fn main() {
    println!("Fib index with 1000 digits is {}", fib_index_1000());
}
