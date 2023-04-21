use std::process::{Command, Stdio};
use std::io::Write;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};

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

pub fn run_program(cmd: &String, input: &String) -> String {
    let split: Vec<String> = cmd.split(' ').map(|s| { s.to_string() }).collect();

    let mut perf_command = Command::new(&split[0])
        .args(&split[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run perf command");
    let mut stdin = perf_command.stdin.take().expect("Failed to open stdin");
    write!(stdin, "{}", input).expect("Failed to write to stdin");

    let output = perf_command
        .wait_with_output()
        .expect("Failed to read the output");
    String::from_utf8(output.stdout).expect("Failed to convert to utf8")
}

pub fn prepare_command(
    args: String,
    program_path: String,
    method: String,
    iterations: u32,
    ) -> String {
    format!(
        "perf stat -r {iterations} -x, -e {method}:u \
        {program_path} {args} > /dev/null"
    )
}

pub fn thread_worker(data: Arc<Mutex<ThreadsData>>) {
    loop {
        let popped_char;
        {
            let mut data = data.lock().unwrap();
            popped_char = data.chars_to_process.pop();
        }

        let char = match popped_char {
            Some(char) => char,
            _ => {
                thread::sleep(time::Duration::from_millis(10));
                continue;
            }
        };

        // Process the found char
    }
}
