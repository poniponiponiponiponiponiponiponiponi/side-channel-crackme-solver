use clap::Parser;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};
use side_channel_crackme_solver::workers;
use side_channel_crackme_solver::workers::ThreadsData;
use side_channel_crackme_solver::args::Args;
use side_channel_crackme_solver::command::{PreparedCommand, InputPreparer};
use log::info;
use env_logger;

fn main() {
    let mut args = Args::parse();

    // Logging turned on by default cuz usually
    // I want to actually see what the program is doing
    if !args.quiet {
        let default = env_logger::Env::default()
            .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
        env_logger::init_from_env(default);
    }
    info!("Starting solver...");

    if args.alphabet == "" {
        for i in 1..0x80 {
            args.alphabet.push(i as u8 as char);
        }
        info!("No alphabet given. Use the default one: ascii values from 0x01 to 0x7f.");
    }
    
    if args.threads == 0 {
        args.threads = thread::available_parallelism().unwrap().get();
        info!("No number of threads given. Use the detected recommended number instead: {}",
              args.threads);
    }

    main_loop(args);
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
        &"instructions".to_string(),
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

            // If password length is satisfied then quit.
            let prefix_len = input_preparer.input_prefix.len();
            let postfix_len = input_preparer.input_postfix.len();
            let input_len = prefix_len + data.found_password_prefix.len() +
                postfix_len;
            if input_len == input_preparer.length {
                break;
            }
            data.processed_chars = Vec::new();
            data.chars_to_process = args.alphabet.chars().collect();

            if !args.quiet {
                println!("Currently found password: {}", data.found_password_prefix);
            }
        }
    }

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
