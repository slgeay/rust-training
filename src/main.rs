mod fizzbuzz;
mod narcissistic_number_check;
mod rustlatin;
mod calculator;
mod serde_lifetimes;

fn main() {
    fizzbuzz::main();
    rustlatin::main();
    narcissistic_number_check::main();
    calculator::main();
    serde_lifetimes::main().unwrap();
}
