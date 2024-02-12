fn fizzbuzz(i: u32) -> String {
    if i % 15 == 0 {
        String::from("FizzBuzz")
    } else if i % 3 == 0 {
        String::from("Fizz")
    } else if i % 5 == 0 {
        String::from("Buzz")
    } else {
        i.to_string()
    }
}

pub(crate) fn main() {
    println!("<<< FizzBuzz >>>");
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(1), "1");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
}
