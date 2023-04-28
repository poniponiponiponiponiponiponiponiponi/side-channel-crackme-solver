use crate::command::{PreparedCommand, InputPreparer};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

pub struct ThreadsData {
    pub chars_to_process: Vec<char>,
    pub processed_chars: Vec<(i32, char)>,
    pub found_password_prefix: String,
}

impl ThreadsData {
    pub fn new() -> ThreadsData {
        ThreadsData {
            chars_to_process: vec![],
            processed_chars: vec![],
            found_password_prefix: String::new(),
        }
    }
}

pub fn thread_worker(
        data: Arc<Mutex<ThreadsData>>,
        command: PreparedCommand,
        input_preparer: InputPreparer) {
    let mut prefix = String::new();
    loop {
        let popped_char;
        {
            let mut data = data.lock().unwrap();
            popped_char = data.chars_to_process.pop();
            if data.found_password_prefix.len() > prefix.len() {
                prefix = data.found_password_prefix.clone();
            }
        }

        let popped_char = match popped_char {
            Some(char) => char,
            _ => {
                thread::sleep(time::Duration::from_millis(10));
                continue;
            }
        };

        // Process the found char
        prefix.push(popped_char);
        command.run(&prefix);
        prefix.pop();
    }
}
