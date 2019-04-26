use std::io;
use std::process;
use std::collections::HashSet;


fn convert_string(input: String) -> String {
    let words: Vec<&str> = input.split(' ').collect();

    let mut result_vec: Vec<&str> = Vec::new();

    let vowel_set: HashSet<&'static str> = [ "a", "e", "i", "o", "u", "A", "E", "I", "O", "U", "1", "2", "3", "4", "5", "6", "7", "8", "9" ].iter().cloned().collect();

    let mut result_word = String::new();

    for word in words {
        let (letter, rest_of_word) = word.split_at(1);

        if vowel_set.contains(letter) {
            result_word.push_str(word);
            result_word.push_str("hay");
        } else {
            result_word.push_str(rest_of_word);
            result_word.push_str(letter);
            result_word.push_str("ay");
        };
        
        result_vec.push(&result_word);

    }
    return result_vec.join(" ");
}

fn main() {
    println!("Press q to quit");
    loop {
        let mut input_text = String::new();

        println!("Enter a sentence to translate into pig latin");

        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");

        let trimmed = input_text.trim();

        if trimmed == "q" {
            process::exit(1);
        };

        let result_str = convert_string(trimmed.to_string());
        println!("{}", result_str);
    }
}
