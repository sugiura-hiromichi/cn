//!Overrides cargo new
use mylibrary::sh_cmd;
use std::fs;
use std::io;

const GITIGNORE: &[u8] = b"Cargo.lock";
const README: &[u8] = b"This repository is";
const CARGO_TOML: &[u8] = b"mylibrary={git=\"https://github.com/sugiura-hiromichi/mylibrary\"}";
const MAIN_RS: &[u8] = b"#![allow(unused)]

fn main(){

}";
const LIB_RS: &[u8] = b"//! `crate_name`

#[cfg(test)]
mod tests {
   use super::*;

      #[test]
      fn give_test_name() {}
}";

///If `path` exist, append it's content
///`content` start with newline
fn append_path(path: String, content: &[u8],) -> io::Result<(),> {
   let mut cntnt = fs::read_to_string(path.clone(),)?;
   cntnt.push_str(std::str::from_utf8(content,).unwrap(),);
   fs::write(path, cntnt,)
}

///  todo!("==============================================================
///                [CommandLineArgument]
///                        is there any way to handle options & path as
/// commandline argument?
/// ==============================================================");
fn main() -> io::Result<(),> {
   let mut buf = String::new();
   println!("input options & name");
   std::io::stdin().read_line(&mut buf,)?;
   let args = format!("new {buf}");

   sh_cmd!("cargo", args.split_whitespace());

   let name = buf.split_whitespace().last().unwrap().to_string();
   if buf.contains("--lib",) {
      //When to lib package
      fs::write(name.clone() + "/src/lib.rs", LIB_RS,)?;
   } else {
      //When to bin package
      fs::write(name.clone() + "/src/main.rs", MAIN_RS,)?;
      append_path(name.clone() + "/.gitignore", GITIGNORE,)?;
   }

   append_path(name.clone() + "/Cargo.toml", CARGO_TOML,)?;
   fs::write(name.clone() + "/README.md", README,)?;

   Ok((),)
}
