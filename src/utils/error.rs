use std::process;
use std::io::{self, Write};

/* TODO -> write error into log file */

/* Print error message into std err */
pub fn err_msg(msg: &str) {
    writeln!(io::stderr(), "{}", msg).unwrap();
}

/* Exiting program */
pub fn exit(cool: bool) -> ! {
    match cool {
        true => {
            println!("Exiting program ...");
            process::exit(0)
        },
        false => {
            err_msg("Aborting program !");
            process::abort()
        }
    }
}
