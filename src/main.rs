//!Overrides cargo new
use mylibrary::cli::CliParser;
use mylibrary::sh_cmd;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

const GITIGNORE: &[u8] = b"/target
Cargo.lock";
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
fn append_path<P: AsRef<Path,>,>(path: P, content: &[u8],) -> io::Result<(),> {
	let mut cntnt = fs::read_to_string(&path,)?;
	cntnt.push_str(std::str::from_utf8(content,).unwrap(),);
	fs::write(path, cntnt,)
}

fn main() -> io::Result<(),> {
	let args = std::env::args().to_string();
	sh_cmd!("cargo", format!("new {args}").split_whitespace());

	let name = args.split_whitespace().last().unwrap().to_string();
	env::set_current_dir(name,)?;
	if args.contains("--lib",) {
		//When to lib package
		fs::write("src/lib.rs", LIB_RS,)?;
	} else {
		//When to bin package
		fs::write("src/main.rs", MAIN_RS,)?;
		fs::write(".gitignore", GITIGNORE,)?;
	}

	fs::write("README.md", README,)?;
	append_path("Cargo.toml", CARGO_TOML,)?;

	Ok((),)
}
