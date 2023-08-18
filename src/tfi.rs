use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn main(file_path: String, print_file: bool, word_occur: bool) {
    let mut contents = String::new();

    // Opening the file and reading its contents
    match File::open(file_path) {
        Ok(mut file) => {
            if let Err(err) = file.read_to_string(&mut contents) { eprintln!("Error reading file: {}", err); return; }
        }
        Err(err) => { eprintln!("Error opening file: {}", err); return; }
    }
    
    // contents of the file
    if print_file {
        println!("    >-----------start of file----------<\n\n{}\n\n    >----------end of file----------<\n", contents)}
    
    // word occurrences
    if word_occur {
        let word_occurrences = get_word_occurrences(contents.clone());
        if !word_occurrences.is_empty() {
            println!("     >----------------------------------word occurrences------------------------------------------------<\n");        

            let max_word_length = 25;
            
            let mut iter = word_occurrences.iter();
            
            while let Some((word1, count1)) = iter.next() {
                let (word2, count2) = iter.next().map(|(w, c)| (w.clone(), *c)).unwrap_or((String::from(""), 0));
                let (word3, count3) = iter.next().map(|(w, c)| (w.clone(), *c)).unwrap_or((String::from(""), 0));

                // Truncate or pad the words to the specified length
                let truncated_word1 = if word1.len() > max_word_length {
                    format!("{}...", &word1[..max_word_length - 3])
                } else {
                    format!("{:<width$}", word1, width = max_word_length)
                };

                let truncated_word2 = if word2.len() > max_word_length {
                    format!("{}...", &word2[..max_word_length - 3])
                } else {
                    format!("{:<width$}", word2, width = max_word_length)
                };

                let truncated_word3 = if word3.len() > max_word_length {
                    format!("{}...", &word3[..max_word_length - 3])
                } else {
                    format!("{:<width$}", word3, width = max_word_length)
                };

                println!("{:<width$} : {}            {:<width$} : {}          {:<width$} : {}", truncated_word1, count1, truncated_word2, count2, truncated_word3, count3, width = max_word_length);
            }
            
            println!("\n    >------------------------------------------------------------------------------------------------<");
        }
    }


    // lines count
    println!("\nlines: {}", contents.lines().count());

    // words count
    println!("words: {}", contents.split_whitespace().count());
    
    // size
    let (size, test) = get_size(contents.clone());
    println!("size: {:.1} {}", size, test);
}

fn get_word_occurrences(contents: String) -> HashMap<String, usize> {

    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let words =contents.split_whitespace ();

    for word in words {
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    word_counts
}

fn get_size(contents: String) -> (f32, &'static str) {
    let mut size: f32 = 0.0;
    let mut bytes: &str = "B";

    if contents.len() < 1024 {
        size = contents.len() as f32;
    } else if contents.len() >= 1024 {
        size = contents.len() as f32 / 1024.0;
        bytes = "KB";
    } else if contents.len() as f32 > 1024.0_f32.powf(2.0) {
        size = contents.len() as f32 / 1024.0_f32.powf(2.0);
        bytes  = "MB";
    } else if contents.len() as f32 > 1024.0_f32.powf(3.0) {
        size = contents.len() as f32 / 1024.0_f32.powf(3.0);
        bytes = "GB";
    }

    (size, bytes)
}