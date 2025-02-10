//! Big Bag Of Words
//!
//! The "Big Bag Of Words" is used in text analysis and
//! machine learning.  It reduces a text to a collection of
//! words, each with a count of the number of occurrences.
//!
//! This implementation uses zero-copy strings when
//! reasonably possible to improve performance and reduce
//! memory usage.
//!
//! Words are separated by whitespace, and consist of a
//! span of one or more consecutive letters (any Unicode
//! code point in the "letter" class) with no internal
//! punctuation: leading and trailing punctuation are
//! removed.
//!
//! For example, the text
//!
//! ```text
//! "It ain't over untïl it ain't, over."
//! ```
//!
//! contains the sequence of words `"It"`, `"over"`,
//! `"untïl"`, `"it"`, `"over"`.
//!
//! Words in the bag containing uppercase letters will be
//! represented by their lowercase equivalent.

use std::borrow::Cow;
use std::collections::BTreeMap;

/// Each key in this struct's map is a word in some
/// in-memory text document. The corresponding value is the
/// count of occurrences.
#[derive(Debug, Default, Clone)]
pub struct Bbow<'a>(BTreeMap<Cow<'a, str>, usize>);

fn is_word(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_alphabetic())
}

// Check if the work has punctaition and remove the punctuation...
// Example: "Hello, world!" --> "Hello", "world" (remove "," and "!").
// Also check of words have appostraphies and don't cound them:
// Example: "aren't" and "don't" and "ain't" are not words.
//fn parse_word(word: &str) -> &str {
fn parse_word(word: &str) -> String{
    if !word.ends_with(|c: char| c.is_alphabetic()) && !word.contains('\'') {
        return word.trim_end_matches(|c: char| !c.is_alphabetic()).to_string();
    }
    "".to_string()
}

fn has_uppercase(word: &str) -> bool {
    word.chars().any(char::is_uppercase)
}

impl<'a> Bbow<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this BBOW.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up a BBOW covering
    /// multiple texts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new().extend_from_text("Hello world.");
    /// assert_eq!(2, bbow.len());
    /// assert_eq!(1, bbow.match_count("hello"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        // Vector holding words (bag of words ==> bow):
        let mut bow: Vec<Cow<'a, str>> = vec![];

        for word in target.split_whitespace() { // Check words in the target sentence
            if is_word(word) { 
               bow.push(Cow::Borrowed(word));
            } else {
                bow.push(Cow::Owned(parse_word(word)));
            }
        }

        // Using functional programming, check if a word in the vector has
        // an uppercase letter and if so, change it to a lowercase:
        bow.iter_mut().for_each(|word| *word = Cow::Owned(word.to_lowercase()));

        // Remove all empty strings returned from the parse_words method:
        bow.retain(|word| !word.is_empty()); 

        // Loop through bow and add each word to Bbow:
        for i in 0..bow.len() {
            println!("{}", bow[i]);
            let count = self.0.entry(bow[i].clone()).or_insert(0);
            *count += 1;
        }

        self
    }

    /// Report the number of occurrences of the given
    /// `keyword` that are indexed by this BBOW. The keyword
    /// should be lowercase and not contain punctuation, as
    /// per the rules of BBOW: otherwise the keyword will
    /// not match and 0 will be returned.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("b b b-banana b");
    /// assert_eq!(3, bbow.match_count("b"));
    /// ```
    pub fn match_count(&self, keyword: &str) -> usize {
        // Self.0 accesses the only elem in the struct Bbow (the BTreeMap).
        // BTreeMap get() method (https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.get) returns an Option<>.
        // Cow::Borrowed(keyword) Holds the borrowed referenve of the &str keyword.
        // &Cow::Borrowed(keyword) means we are referencing a borrwed &str
        // self.0.get(&Cow::Borrowed(keyword)) is an Option<>, so we use
        // unwrap_or(&0) to return either the count of the keyword if it exists, or
        // 0 is it doesn't exist in the BTreeMap.
        // And we derefence the entire expression with the '*':
        *self.0.get(&Cow::Borrowed(keyword)).unwrap_or(&0)
    }

    pub fn words(&'a self) -> impl Iterator<Item = &'a str> {
        self.0.keys().map(|w| w.as_ref())
    }

    /// Count the overall number of words contained in this BBOW:
    /// multiple occurrences are considered separate.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(3, bbow.count());
    /// ```
    pub fn count(&self) -> usize {
        todo!()
    }

    /// Count the number of unique words contained in this BBOW,
    /// not considering number of occurrences.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(2, bbow.len());
    /// ```
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Is this BBOW empty?
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Bring all other scoped data into testing
}
