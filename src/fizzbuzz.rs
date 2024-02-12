fn fizzbuzz(i: u32) -> String {
    let mut res = String::from("");
    if i%3 == 0 {
        res.push_str("Fizz")
    }
    if i%5 == 0 {
        res.push_str("Buzz")
    }
    if res.is_empty() {
        return i.to_string()
    }
    res
}

pub(crate) fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}