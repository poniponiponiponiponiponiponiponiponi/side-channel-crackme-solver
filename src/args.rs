use clap::Parser;

/// Solver for simple crackmes using side channel attacks.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Event to use in perf. To list all possible perf events use `perf list`.
    #[arg(short, long, default_value = "instructions")]
    pub event: String,

    /// String the input starts with. It's length is not a part of the --length length.
    #[arg(long, default_value = "")]
    pub input_beg: String,

    /// String the input ends with. It's length is not a part of the --length length.
    #[arg(long, default_value = "")]
    pub input_end: String,

    /// String the password should start with. If it doesn't then execution is aborted.
    /// It checks the password found, it doesn't take into account what's inside of --input-beg.
    #[arg(long, default_value = "")]
    pub starts_with: String,

    /// String the password should end with. If it doesn't then execution is aborted.
    /// It checks the password found, it doesn't take into account what's inside of --input-beg.
    #[arg(long, default_value = "")]
    pub ends_with: String,

    /// String that represents the alphabet used in the bruteforcing process.
    /// If none is set then every printable character is used.
    #[arg(long, default_value = "")]
    pub alphabet: String,

    /// Number of worker threads (so the actual number of threads is threads+1).
    /// If set to zero the program will determinate the best number automatically.
    #[arg(short, long, default_value_t = 0)]
    pub threads: usize,

    /// Number of program runs per checked password.
    #[arg(short, long, default_value_t = 32)]
    pub iterations: u32,
    
    /// Pass input to stdin. If not use argv instead.
    #[arg(short, long, default_value_t = true)]
    pub stdin: bool,

    /// Quiet option for easier output parsing/piping.
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// File path to the executable.
    #[arg(index=1)]
    pub exe_path: String,

    /// Length of the password. If set to zero the program will
    /// try to find it first instead.
    #[arg(short, long, default_value_t = 0)]
    pub length: usize,

    /// Max length of searched password. Used while trying to determinate the length.
    #[arg(long, default_value_t = 50)]
    pub max_length: usize,

    /// Character used for padding so the char has the correct length.
    #[arg(long, default_value_t = '#')]
    pub padding: char,
}
