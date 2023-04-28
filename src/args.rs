use clap::Parser;
/// Solver for simple crackmes using side channel attacks.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// String the input starts start with.
    #[arg(long, default_value = "")]
    pub input_beg: String,

    /// String the input ends start with.
    #[arg(long, default_value = "")]
    pub input_end: String,

    /// String the password should start with. If it doesn't then execution is aborted.
    #[arg(long, default_value = "")]
    pub starts_with: String,

    /// String the password should end with. If it doesn't then execution is aborted.
    #[arg(long, default_value = "")]
    pub ends_with: String,

    /// String that represents the alphabet used in the bruteforcing process.
    /// If none is set then every character in the range 0-0xff inclusive is used.
    #[arg(long, default_value = "")]
    pub alphabet: String,

    /// Use only printable characters from the alphabet.
    #[arg(short, long, default_value_t = false)]
    pub printable: bool,

    /// Number of worker threads (so the actual number of threads is threads+1).
    #[arg(short, long, default_value_t = 3)]
    pub threads: u32,

    /// Number of program runs per checked password.
    #[arg(short, long, default_value_t = 10)]
    pub iterations: u32,
    
    /// Pass input to stdin. If not use argv instead.
    #[arg(short, long, default_value_t = true)]
    pub stdin: bool,

    /// File path to the executable.
    #[arg(index=1)]
    pub exe_path: String,

    /// Length of the password. If set to zero the program will
    /// try to find it first instead.
    #[arg(short, long, default_value_t = 0)]
    pub length: usize,

    /// Character used for padding so the char has the correct length.
    #[arg(long, default_value_t = 'A')]
    pub padding: char,
}
