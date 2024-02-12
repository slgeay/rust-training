fn fizzbuzz(i: u32) -> String {
    match (i % 3, i % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => i.to_string()
    }
}

pub(crate) fn main() {
    println!("<<< FizzBuzzMatch >>>");
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}