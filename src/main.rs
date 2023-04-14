use clap::Parser;

/// Solver for simple crackmes using side channel attacks.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String the password starts with.
    #[arg(long, default_value = "")]
    starts_with: String,

    /// String the input starts start with.
    #[arg(long, default_value = "")]
    input_beg: String,

    /// String the input ends start with.
    #[arg(long, default_value = "")]
    input_end: String,

    /// String the password ends with.
    #[arg(long, default_value = "")]
    ends_with: String,

    /// String that represents the alphabet used in the bruteforcing process.
    /// If none is set then every character in the range 0-0xff inclusive is used.
    #[arg(long, default_value = "")]
    alphabet: String,

    /// Use only printable characters from the alphabet.
    #[arg(short, long, default_value_t = false)]
    printable: bool,

    /// Number of worker threads (so the actual number of threads is threads+1).
    #[arg(short, long, default_value_t = 3)]
    threads: u32,

    /// Number of program runs per checked password.
    #[arg(short, long, default_value_t = 10)]
    iterations: u32,
    
    /// Pass input to stdin. If not use argv instead.
    #[arg(short, long, default_value_t = true)]
    stdin: bool,

    // File path to the executable.
    #[arg(index=1)]
    exe_path: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.exe_path);
}
