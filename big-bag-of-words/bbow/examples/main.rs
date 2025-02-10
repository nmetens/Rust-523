use bbow::Bbow;

fn main() {
    // The target text with words:
    let target: &str = "world Hello! there wOrLd, hello hoW hip hellO you? are HELLO THERE HOW hoi WORLD aren't y'all you are?? jonie? hoi";
    // let target = ""; // Empty bag of words
    println!("Target: '{}'", target);

    // Call the bbow passing target to filter out bad words (words with appostrophes) and count repeat words:
    let bbow = Bbow::new().extend_from_text(target);

    println!("The bbow: {:?}", bbow); // Show the contents of the bbow

    let keyword: &str = "hello";
    println!(
        "The word '{}' appears {} times in the target.",
        keyword,
        bbow.match_count(keyword)
    );

    println!("There are {} total words in the bbow.", bbow.count());

    println!(
        "There are a total of {} unique words in the bbow.",
        bbow.len()
    );

    if bbow.is_empty() {
        println!("The bag of words is empty!");
    } else {
        println!("The bag of words is NOT empty!");
    }
}
