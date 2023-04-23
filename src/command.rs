use std::process::{Command, Stdio};
use std::io::Write;

pub struct PreparedCommand {
    pub command_prefix: Vec<String>,
    pub command_postfix: Vec<String>,
    pub stdin: bool,
}

impl PreparedCommand {
    pub fn new(
        program_path: &String,
        method: &String,
        iterations: u32,
        stdin: bool,
        ) -> PreparedCommand {
        let command_prefix = format!("perf stat -r {iterations} -x, \
                                     -e {method}:u {program_path}");
        let command_prefix = command_prefix.split(' ').map(|s| {
            s.to_string()
        }).collect();
        let command_postfix = "> /dev/null".to_string();
        let command_postfix = command_postfix.split(' ').map(|s| {
            s.to_string()
        }).collect();
        PreparedCommand {
            command_prefix,
            command_postfix,
            stdin,
        }
    }

    fn get_cmd_split(&self, input: &String) -> Vec<String> {
        let mut split: Vec<String> = Vec::new();
        self.command_prefix.iter().for_each(|s| split.push(s.to_string()));
        if self.stdin {
            split.push(input.clone());
        }
        self.command_postfix.iter().for_each(|s| split.push(s.to_string()));
        split
    }

    pub fn run(self, input: &String) -> String {
        let split = self.get_cmd_split(input);

        let mut perf_command;
        if self.stdin {
            perf_command = Command::new(&split[0])
                .args(&split[1..])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to run perf command");
            let mut stdin = perf_command.stdin.take().expect("Failed to open stdin");
            write!(stdin, "{}", input).expect("Failed to write to stdin");
        } else {
            perf_command = Command::new(&split[0])
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to run perf command");
        }

        let output = perf_command
            .wait_with_output()
            .expect("Failed to read the output");
        String::from_utf8(output.stdout).expect("Failed to convert to utf8")
    }
}
