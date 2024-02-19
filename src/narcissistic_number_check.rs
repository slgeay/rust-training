// https://ferrous-systems.github.io/teaching-material/assignments/narcissistic-number-check.html

fn get_digits(num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits
}

fn is_narcissistic(num: u32) -> bool {
    let digits = get_digits(num);
    let num_digits = digits.len() as u32;
    let sum: u32 = digits.iter().map(|&x| x.pow(num_digits)).sum();
    sum == num
}

pub fn main() {
    println!("<<< Narcissistic Number Check >>>");
    for i in 1..=1000 {
        if is_narcissistic(i) {
            println!("{}", i);
        }
    }
}

#[test]
fn test_get_digits() {
    assert_eq!([2, 3, 1, 4, 1, 5, 9, 2, 6].to_vec(), get_digits(629514132))
}

#[test]
fn test_is_narcissistic() {
    assert_eq!(true, is_narcissistic(407));
}

#[test]
fn test_is_not_narcissistic() {
    assert_eq!(false, is_narcissistic(408));
}
