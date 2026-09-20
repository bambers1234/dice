//! Allow the user to interact with this app via a command-line interface
use clap::{Args, Parser};
use std::io::{BufReader, Read};
use std::path::{PathBuf, Path};
use std::fs::File;
/// Automate complicated or frequent dice rolls for RPGs
/// 
/// Input some code and the program will translate it. You can then save it in a file and run whenever you want.
/// # Syntax
/// ## Dice 
/// XdN rolls X dice with N sides
/// ## Arithmetic
/// Basic arithmetic- +, -, *, /
/// Performing arithmetic on a dice roll such as XdN + 1 performs the arithmetic on the total of the roll.
#[derive(Parser, Debug, Clone)]
#[command(name = "Dice", version = "1.0")]
pub struct Cli {
    #[command(flatten)]
    input: Input,
    /// Optionally provide a file to store the compiled code in.
    #[arg(long, short)]
    out: Option<PathBuf>,
    /// Run the code now or save it for later. If no output file is given, defaults to true.
    #[arg(long, short = 'i')]
    run_immediately: bool
}
impl Cli {
    /// If no file is provided, the compiled bytecode must run immediately. Otherwise, the user can decide.
    fn infer_run_immediately(&mut self) {
        if self.out.is_none() { // no out file given
            self.run_immediately = true // must therefore run or be wasted
        }
        // otherwise, leave run_immediately untouched
    }
    /// Collate the command-line args, exit on error, and enforce that the code is either run or stored after compilation.
    pub fn parse() -> Self {
        let mut cli: Self = Parser::parse();
        cli.infer_run_immediately();
        cli
    }
    /// Return the code supplied as input, transformed to lowercase and trimmed
    pub fn input(&self) -> String {
        let (file, code) = (&self.input.file, &self.input.code);
        let input = match (file, code) {
            #[allow(clippy::unwrap_used)]
            // panic is desirable
            (Some(file), _) => &Self::read_from_file(file.as_path()).unwrap(),
            (_, Some(code)) => code,
            #[allow(clippy::panic)] // clap enforces at least one is present
            (None, None)  => panic!("No input was provided")
        };
        input.trim().to_lowercase()
    }
    fn read_from_file(file: &Path) -> Result<String, std::io::Error> {
        let file = File::open(file)?;
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
/// Where to read input from
#[derive(Args, Debug, Clone)]
#[group(required = true, multiple = false)]
struct Input {
    /// Read in code from a text file
    #[arg(long, short)]
    file: Option<PathBuf>,
    /// Supply your code as the argument
    #[arg(long, short)]
    code: Option<String>,
}
// TODO: optionally store code in output file.
// if file is given, allow RunImmediately::No
