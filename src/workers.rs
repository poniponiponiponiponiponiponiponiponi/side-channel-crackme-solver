use std::process::{Command, Stdio};
use std::io::Write;

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
