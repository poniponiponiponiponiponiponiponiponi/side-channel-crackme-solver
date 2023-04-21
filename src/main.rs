use clap::Parser;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};
use side_channel_crackme_solver::workers;
use side_channel_crackme_solver::workers::ThreadsData;
use side_channel_crackme_solver::args::Args;

fn main() {
    let args = Args::parse();
    main_loop(args);
}

pub fn main_loop(args: Args) {
    let mut thread_workers = vec![];
    let data = Arc::new(Mutex::new(ThreadsData::new()));
    for _ in 0..args.threads {
        let data = Arc::clone(&data);
        thread_workers.push(thread::spawn(|| workers::thread_worker(data)));
    }

    'outer: loop {
        // Wait till there are no chars left to process
        loop {
            let chars_left;
            {
                let data = data.lock().unwrap();
                // If password length is satisfied then quit.
                if data.found_password_prefix.len() == args.length {
                    break 'outer;
                }
                chars_left = data.chars_to_process.len();
            }

            if chars_left > 0 {
                thread::sleep(time::Duration::from_millis(100));
                continue;
            } else {
                break;
            }
        }

        // Process the found chars
    }

    for thread in thread_workers {
        thread.join().unwrap();
    }
}
