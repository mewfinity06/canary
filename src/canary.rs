#![allow(dead_code)]

pub const INFO: &str = "\x1b[38;5;40m";
pub const WARNING: &str = "\x1b[38;5;226m";
pub const ERROR: &str = "\x1b[38;5;196m";
pub const RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! info {
    ($( $args:expr ),+) => {
        use crate::canary::*;
        print!("[{}INFO{}] ", INFO, RESET);
        println!($( $args ),+);
    };
}

#[macro_export]
macro_rules! warning {
    ($( $arg:expr ),+) => {
        use crate::canary::*;
        print!("[{}WARNING{}] ", WARNING, RESET);
        println!($( $arg ),+);
    };
}

#[macro_export]
macro_rules! error {
    ($( $arg:expr ),+) => {
        use crate::canary::*;
        print!("[{}ERROR{}] ", ERROR, RESET);
        println!($( $arg ),+);
    };
}
