//! Allow the user to interact with this app via the command line as a shell.
use std::io::{stdin, stdout};
use crate::compile;
use crate::chars::EOF;
/// Create an interactive shell for the user.
fn create_shell_from_stdin() {
    let mut input = String::new();
    loop {
        stdin().read_line(&mut input).expect("Broken input pipeline- restart the shell by running the command again.");
        if input.ends_with(EOF) {
            break
        }
        // do not clear input, we want to keep line breaks
    }
    compile(input);
    // TODO: run bytecode and output
}
