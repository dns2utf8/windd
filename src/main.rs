//! This program creates a empty file and consumes all available space.
//! After the new file consumed all available space the file will be removed.

extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

use std::io::prelude::*;
use std::fs::{File, remove_file};
use std::env::args as arguments;

/// 1mB buffer
const BUFFER_LENGTH: usize = 1024 * 1024;
static RUNNING: AtomicBool = ATOMIC_BOOL_INIT;

fn main() {
  RUNNING.store(true, Ordering::Relaxed);
  let file_name = arguments().nth(1).unwrap_or("empty.bin".into());
  match &*file_name {
    "-h" | "--help" | "/h" | "/help" => {
	  println!("{} [file_name]\n  Fill the disk with a zeroed file.", arguments().next().unwrap());
	  return;
	}
	_ => {}
  }
  println!("filling: {}", file_name);
  
  ctrlc::set_handler(move || {
    RUNNING.store(false, Ordering::SeqCst);
	println!("Shutting down ...");
  });
  
  create_empty_file(&file_name).is_ok();
  remove_file(&file_name).unwrap();
}

fn create_empty_file(name: &String) -> std::io::Result<()> {
  let mut f = try!(File::create(name));

  let arr: [u8; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
  
  while RUNNING.load(Ordering::Relaxed) {
    try!(f.write_all(&arr));
  }
  
  Ok( () )
}
