# Homework 3 Rust 523 (BBOW)

This homework implements the methods created in the impl of a struct called Bbow
that has one element: a BTreeMap. The BTreeMap has a key (copy on write (COW) that 
holds a reference to a string slice (a word)) and its count.

The methods implemented were:
    1. parse-word: Takes a &str and returns the String version of the word.
                   The method checks if the word ends with a non-alphabetic
                   character ('?', ',', '!', '.') and removes the punctuation.
                   The method also checks that the word doesn't have an 
                   apostrophy in it. Those aren't words allowed in the bbow.
    2. extend-from-text: Takes the Bbow struct and a target string slice with
                         the same lieftime as the Bbow and returns the Bbow. The
                         method create a vector of Copy on write words with all 
                         the parsed words in the target string slice. If the word
                         is a word (based on the bool is-word() method), we borrow 
                         the word otherwise, we own the String passed back from the
                         parsed-word() method described earlier. The method then 
                         iterates over the vector of words and makes them all 
                         lowercase and removes empty strings, then count each 
                         occurance of every word and add the counts to the usize
                         variable in the Bbow struct which containes the BTreeMap,
                         mapping each word to their count.
    3. match-count: Takes Bbow and a keyword. The method gets the count of that
                    keyword in the BTreeMap if it exists, otherwise return 0.
    4. count: Takes the Bbow (self) and adds all the counts together in the 
              BTreeMap, returning the total count of all occurances of each word.
    5. len: Takes the Bbow (self) and returns the count of unique words (how many
            unique words/keys are in the BTreeMap).
    6. is-empty: Returns true if there are 0 unique words in the BTreeMap, false 
                 otherwise.

The code for the Bbow implementation is in the src/lib.rs library.

## Run examples on the code

To run an examples main binary, run ```cargo run --example main```.
The main creates a string slice of words and non words. Then it calls all the
methods implemented above in the src/lib.rs library to create a Big bag of words
and dsiplay all the information.
