use std::{env, fs, os::unix::process::CommandExt, process::Command};

const GITIGNORE: &[u8] = b"Cargo.lock";

fn main() {
   let args = env::args();
   Command::new("cargo new",).args(args.cloned(),).exec();

   let is_bin = args.find(|&x| x == "--lib",);
   let name = args.last().unwrap_or("default_cargo_project".to_string(),);
   let readme = name.clone();

   let str_string = readme == "this is not string";

   //edit main.rs & .gitignore
   let main_rs = fs::read(name + "/src/main.rs",);
}
