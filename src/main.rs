use clap::Parser;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};
use std::path::Path;
use std::error::Error;
use which;
use side_channel_crackme_solver::workers;
use side_channel_crackme_solver::workers::ThreadsData;
use side_channel_crackme_solver::args::Args;
use side_channel_crackme_solver::command::{PreparedCommand, InputPreparer};
use log::info;
use env_logger;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    if !Path::new(&args.exe_path).is_file() {
        return Err(format!("File does not exist: {}", args.exe_path).into());
    }

    if !which::which("perf").is_ok() {
        return Err("Can't find perf binary in your $PATH".into());
    }

    // Logging turned on by default cuz usually
    // I want to actually see what the program is doing
    if !args.quiet {
        let default = env_logger::Env::default()
            .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
        env_logger::init_from_env(default);
    }

    if args.alphabet == "" {
        for i in 1..0x80 {
            args.alphabet.push(i as u8 as char);
        }
        info!("No alphabet given. Using the default one: ascii values from 0x01 to 0x7f.");
    }
    
    if args.threads == 0 {
        args.threads = thread::available_parallelism().unwrap().get();
        info!("No number of threads given. Using the detected recommended number instead: {}",
              args.threads);
    }

    info!("Starting solver...");
    main_loop(args);

    Ok(())
}

pub fn main_loop(args: Args) {
    let mut thread_workers = vec![];
    let data = Arc::new(Mutex::new(ThreadsData::new()));
    {
        let mut data = data.lock().unwrap();
        data.chars_to_process = args.alphabet.chars().collect();
    }
    let input_preparer = InputPreparer::new(
        args.input_beg.clone(),
        args.input_end.clone(),
        args.length,
        args.padding,
    );
    let prepared_command = PreparedCommand::new(
        &args.exe_path,
        &args.event,
        args.iterations,
        args.stdin
    );
    for _ in 0..args.threads {
        let data = Arc::clone(&data);
        let prepared_command = prepared_command.clone();
        let input_preparer = input_preparer.clone();
        thread_workers.push(thread::spawn(
                move || workers::thread_worker(data, prepared_command, input_preparer)
        ));
    }

    loop {
        // Wait till there are no chars left to process
        loop {
            let chars_left;
            {
                let data = data.lock().unwrap();
                chars_left = data.chars_to_process.len();
            }

            if chars_left > 0 {
                info!("Number of chars left in the queue: {}", chars_left);
                thread::sleep(time::Duration::from_millis(250));
                continue;
            } else {
                break;
            }
        }

        // Process the found chars
        {
            let mut data = data.lock().unwrap();
            data.processed_chars.sort();
            let &(_, char) = data.processed_chars.last().unwrap();
            data.found_password_prefix.push(char);

            // Confirm starts_with and ends_with
            if args.starts_with != "" {
                let compare_to = std::cmp::min(
                    data.found_password_prefix.len(),
                    args.starts_with.len()
                    );
                if &data.found_password_prefix[..compare_to] != &args.starts_with[..compare_to] {
                    if !args.quiet {
                        println!("Found password and starts_with argument don't match-up");
                        println!("Found password: {}", data.found_password_prefix);
                        println!("starts_with: {}", args.starts_with);
                        println!("Ending execution...");
                        return;
                    }
                }
            }

            if args.ends_with != "" {
                let end_start_idx = args.length - args.ends_with.len();
                if data.found_password_prefix.len() > end_start_idx {
                    let postfix = &data.found_password_prefix[end_start_idx..];
                    let ends_with = &args.ends_with[..postfix.len()];
                    if postfix != ends_with {
                        if !args.quiet {
                            println!("Found password and ends_with argument don't match-up");
                            println!("Found password: {}", data.found_password_prefix);
                            println!("ends_with: {}", args.ends_with);
                            println!("Ending execution...");
                            return;
                        }
                    }
                }
            }

            // If password length is satisfied then quit.
            if data.found_password_prefix.len() == input_preparer.length {
                break;
            }
            data.processed_chars = Vec::new();
            data.chars_to_process = args.alphabet.chars().collect();

            if !args.quiet {
                println!("Currently found password: {}", data.found_password_prefix);
            }
        }
    }

    // Final results
    {
        let data = data.lock().unwrap();
        if !args.quiet {
            println!("Found: {}", data.found_password_prefix);
        } else {
            print!("{}", data.found_password_prefix);
        }
    }
    for thread in thread_workers {
        thread.join().unwrap();
    }
}
