use bbow::*;

fn main() {

    // The target text with words:
    let target: &str = "Hello! there wOrLd, hello hoW are12 you? HELLO THERE HOW WORLDaren't y'all you are???";
    
    // Call the bbow passing target to filter out bad words and count repeat words:
    let bbow = Bbow::new().extend_from_text(target);
    
    println!("The bbow: {:?}", bbow);
}
