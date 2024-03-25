use rand::Rng;
use std::fs;

pub fn get_random_word(filename: &str) -> Result<String, &'static str> {
    if filename.len() == 0 {
        return Err("Filename cannot be blank");
    }

    let mut file_words: Vec<String> = Vec::new();
        
    match fs::read_to_string(filename) {
        Err(err) => return Err("Unable to read word file"),    
        Ok(words) => {
            for line in words.lines() {
                file_words.push(line.to_string());
            };
        },
    };

    if file_words.len() == 0 {
        return Err("No words in file");
    }


    // select random number
    let rand_num: usize = rand::thread_rng().gen::<usize>() % (file_words.len() - 1);
    let chosen_word = String::from(&file_words[rand_num]);

    return Ok(chosen_word); 
}
