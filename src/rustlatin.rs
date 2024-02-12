const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

fn rustlatin(sentence: &str) -> String {
    let mut new_words: Vec<String> = Vec::new();

    for word in sentence.split(" ") {
        if VOWELS.contains(&&word[0..1]) {
            new_words.push(format!("sr{}", word));
        } else {
            new_words.push(format!("{}rs", word));
        }
    }
    new_words.join(" ")
}

pub(crate) fn main() {
    println!("<<< Rustlatin >>>");
    println!(
        "{}",
        rustlatin("implement a function that splits a sentence into its words")
    );
}

#[test]
fn test_rustlatin() {
    assert_eq!(
        "srimplement sra functionrs thatrs splitsrs sra sentencers srinto srits wordsrs",
        rustlatin("implement a function that splits a sentence into its words")
    )
}
