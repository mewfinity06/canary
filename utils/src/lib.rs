#![allow(dead_code)]

pub const INFO: &str = "\x1b[38;5;40m";
pub const WARNING: &str = "\x1b[38;5;226m";
pub const ERROR: &str = "\x1b[38;5;196m";
pub const RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! info {
    ($( $args:expr ),+) => {
        print!("   [{}INFO{}] ", utils::INFO, utils::RESET);
        println!($( $args ),+);
    };
}

#[macro_export]
macro_rules! warning {
    ($( $arg:expr ),+) => {
        use crate::*;
        print!(" [{}WARNING{}] ", utils::WARNING, utils::RESET);
        println!($( $arg ),+);
    };
}

#[macro_export]
macro_rules! error {
    ($( $arg:expr ),+) => {
        use crate::*;
        print!("  [{}ERROR{}] ", utils::ERROR, utils::RESET);
        println!($( $arg ),+);
    };
}

#[macro_export]
macro_rules! context {
    ($( $arg:expr ),+) => {
        use crate::*;
        print!("[{}CONTEXT{}] ", utils::INFO, utils::RESET);
        println!($( $arg ),+);
    };
}

#[macro_export]
macro_rules! debug {
    ($( $arg:expr ),+) => {
        use crate::*;
        print!(" [{}DEBUG{}] ", utils::INFO, utils::RESET);
        println!($( $arg ),+);
    };
}
