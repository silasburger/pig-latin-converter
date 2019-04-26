use std::io;
use std::process;
use std::collections::HashSet;


fn convert_string(input: String) -> String {
    let mut words: Vec<String> = input.split_whitespace().map(|w| String::from(w)).collect::<Vec<String>>();

    let vowel_set: HashSet<&'static str> = [ "a", "e", "i", "o", "u", "A", "E", "I", "O", "U", "1", "2", "3", "4", "5", "6", "7", "8", "9" ].iter().cloned().collect();

    for word in &mut words {
        let (letter, rest_of_word) = word.split_at(1);

        if vowel_set.contains(letter) {
            word.push_str("hay");
        } else {
            *word = format!("{}{}ay", rest_of_word, letter);
        };
    }
    
    words.join(" ")
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
