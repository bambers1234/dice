//! Allow the user to interact with this app via the command line as a shell.
use std::io::stdin;
use crate::{INPUT, compile};
/// Create an interactive shell for the user.
fn create_shell_from_stdin() {
    INPUT.get_or_init(|| {
        let mut input = String::new();
        loop {
            stdin().read_line(&mut input).expect("Broken input pipeline- restart the shell by running the command again.");
            if input.ends_with(';') {
                break;
            }
        // do not clear input, we want to keep line breaks
        }
        input
    });
    compile(INPUT.get().unwrap());
    // TODO: run bytecode and output
}