//! This program creates a empty file and consumes all available space.
//! After the new file consumed all available space the file will be removed.

extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};

use std::env::args as arguments;
use std::fs::{remove_file, File};
use std::io::prelude::*;

/// 1mB buffer
const BUFFER_LENGTH: usize = 1024 * 1024;
static RUNNING: AtomicBool = AtomicBool::new(false);

fn main() {
    RUNNING.store(true, Ordering::Relaxed);
    let file_name = arguments().nth(1).unwrap_or("empty.bin".into());
    match &*file_name {
        "-h" | "--help" | "/h" | "/help" => {
            println!(
                "{} [file_name]\n  Fill the disk with a zeroed file.",
                arguments().next().expect("first argument must be available always")
            );
            return;
        }
        _ => {}
    }
    println!("filling: {}", file_name);

    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::SeqCst);
        println!("Shutting down ...");
    })
    .expect("unable to setup abort handler");

    match create_empty_file(&file_name) {
        Ok(_) => {}
        Err(e) => eprintln!("a write error occured: {:?}", e),
    };
    remove_file(&file_name).expect(&*format!("unable to remove {:?}", file_name));
}

fn create_empty_file(name: &String) -> std::io::Result<()> {
    let mut f = File::create(name)?;

    let arr: [u8; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

    while RUNNING.load(Ordering::Relaxed) {
        f.write_all(&arr)?;
    }

    Ok(())
}
