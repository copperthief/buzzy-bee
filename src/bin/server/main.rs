use std::collections::HashMap;
use std::fmt::Display;
use rand::Rng;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufRead, BufReader};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
enum Word {
	Start,
	Word(String),
	End,
}

#[derive(Serialize, Deserialize)]
struct Lang {
    lang : HashMap<Word, Vec<Word>>,
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Word::Start => "[start of sentence]",
            Word::Word(word) => word,
            Word::End => "[end of sentence]",
        })
    }
}

fn main() {	
    let SAVE_ONCE_EVERY = 20;
    let mut should_save = 0;
    let mut lang = load_lang();

    let listener = TcpListener::bind("[listening address]").unwrap();
    for result in listener.incoming() {
        match result {
            Result::Ok(mut stream) => {
                println!("Recieved input stream from bot...");
                let mut request = String::new();

                let mut reader = BufReader::new(&stream);
                match reader.read_line(&mut request) {
                    Result::Ok(_) => {
                        println!("Recieved request: {}", request);
                        if request == "speak\n".to_owned() {
                            let statement = to_string(speak(&lang.lang));
                            println!("Statement: {}", statement);
                            match stream.write((statement + "\n").as_bytes()) {
                                Result::Ok(_) => println!("Statement sent to bot."),
                                Result::Err(e) => println!("Error sending data to bot: {}", e),
                            }
                        } else { 
                            listen(&mut lang.lang, parse(request));
                            should_save += 1;
                            if should_save == SAVE_ONCE_EVERY {
                                should_save = 0;
                                save_lang(&lang);
                            }
                        }
                    },
                    Result::Err(e) => println!("Error reading data from bot: {}", e),
                }
            },
            Result::Err(e) => println!("Error listening for request from bot: {}", e),
        }
    }
}

fn parse(sentence : String) -> Vec<Word> {
    let mut strings : Vec<String> = sentence.split_whitespace().map(str::to_string).collect();
    if strings[0] == "" || strings[0] == "buzzy" || strings[0].chars().last().unwrap() == ':' {
        strings.remove(0);
    }
    let mut words : Vec<Word> = Vec::new();
    for string in strings {
        words.push(Word::Word(string));
    }
    words
}

fn speak<'a>(language : &HashMap<Word, Vec<Word>>) -> Vec<Word> {
	let mut current_word = &Word::Start;
    let mut statement = vec![Word::Start];
    println!("");
    println!("Speaking...");
	while *current_word != Word::End {
        println!("Current word: {}", current_word);
		let choices = language.get(&current_word).unwrap();

        let random = rand::thread_rng().gen_range(0..choices.len());
        
        statement.push(choices[random].clone());

        current_word = &choices[random];
    }
    println!("Finished speaking.");
    println!("");
    statement
}

fn to_string(statement : Vec<Word>) -> String {
    let mut s = String::from("");
    for word in statement {
        match word {
            Word::Word(w) => s.push_str(&(w + " ")),
            _ => (),
        } 
    }
    s
}

fn listen<'a>(language : &mut HashMap<Word, Vec<Word>>, statement : Vec<Word>) {
    let mut last_word = Word::Start;
    for word in statement { 
        if !(word == last_word) {
            add_succ(language, last_word, word.clone());
        }
        last_word = word;
    }
    add_succ(language, last_word, Word::End);
    /*
    match language.get_mut(&last_word) {
        Option::None => {
            language.insert(last_word, vec![Word::End]);
            ()
        }
        Option::Some(w) => w.push(Word::End),
    }*/
}

fn add_succ(language : &mut HashMap<Word, Vec<Word>>, predecessor : Word, successor : Word) {
    println!("Buzzy learned that '{}' can be followed by '{}'.", &predecessor, &successor);
    match language.get_mut(&predecessor) {
        Option::None => {
            language.insert(predecessor, vec![successor]);
            ()
        }   
        Option::Some(succ_list) => succ_list.push(successor),
    }
}

fn save_lang(lang : &Lang) {
    let string = ron::to_string(lang).unwrap();
    fs::write("buzzy-lang.ron", string).expect("Error saving lang to file");
}

fn load_lang() -> Lang {
    if Path::new("buzzy-lang.ron").exists() {
        let string = fs::read_to_string("buzzy-lang.ron").expect("Unable to read file");
        ron::from_str(&string).unwrap()
    } else {
        Lang { lang : HashMap::new(), }
    }
}
