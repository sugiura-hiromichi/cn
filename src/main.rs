use std::{env, fs, os::unix::process::CommandExt, process::Command};

const GITIGNORE: &[u8] = b"Cargo.lock";

fn main() {
   let args: Vec<String,> = env::args().collect();
   let is_bin = args.iter().find(|x| x == "--lib",).unwrap_or(&"dflt_cargo_prj".to_string(),);

   Command::new("cargo",).arg("new",).args(args,).exec();

   //edit main.rs & .gitignore
}
