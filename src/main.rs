//!Overrides cargo new
use mylibrary::cli::CliParser;
use mylibrary::sh_cmd;
use std::fs;
use std::io;

const GITIGNORE: &[u8] = b"/target
Cargo.lock";
const README: &[u8] = b"This repository is";
const CARGO_TOML: &[u8] = b"mylibrary={git=\"https://github.com/sugiura-hiromichi/mylibrary\"}";
const MAKEFILE: &[u8] = b"run :
\tcargo run

test :
\tcargo test

build :
\tcargo build

.PHONY : run test build";
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

fn main() -> io::Result<(),> {
   let args = std::env::args().to_string();
   sh_cmd!("cargo", format!("new {args}").split_whitespace());

   let name = args.split_whitespace().last().unwrap().to_string();
   if args.contains("--lib",) {
      //When to lib package
      fs::write(name.clone() + "/src/lib.rs", LIB_RS,)?;
   } else {
      //When to bin package
      fs::write(name.clone() + "/src/main.rs", MAIN_RS,)?;
      fs::write(name.clone() + "/.gitignore", GITIGNORE,)?;
   }

   fs::write(name.clone() + "/Makefile", MAKEFILE,)?;
   fs::write(name.clone() + "/README.md", README,)?;
   append_path(name.clone() + "/Cargo.toml", CARGO_TOML,)?;

   Ok((),)
}
