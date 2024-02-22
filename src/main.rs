mod fizzbuzz;
mod narcissistic_number_check;
mod rustlatin;
mod calculator;
mod serde_lifetimes;
mod tcp_server;
mod shapes;
mod async_await;

fn main() {
    // fizzbuzz::main();
    // rustlatin::main();
    // narcissistic_number_check::main();
    // calculator::main();
    // tcp_server::main().unwrap();
    // shapes::main();
    // serde_lifetimes::main().unwrap();
    async_await::main().unwrap();
}
