//!Overrides cargo new
use mylibrary::sh_cmd;
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
	let args = std::env::args();
	let mut arg_container = vec!["new".to_string()];
	let mut arg_line = "new ".to_string();
	args.for_each(|a| {
		arg_line.push_str(&a,);
		arg_line.push(' ',);
		arg_container.push(a,);
	},);
	sh_cmd!("cargo", arg_container.clone())?;

	let name = &arg_container[arg_container.len() - 1];
	if arg_line.contains("--lib",) {
		//When to lib package
		fs::write(name.clone() + "/src/lib.rs", LIB_RS,)?;
	} else {
		//When to bin package
		fs::write(name.clone() + "/src/main.rs", MAIN_RS,)?;
		fs::write(name.clone() + "/.gitignore", GITIGNORE,)?;
	}

	fs::write(name.clone() + "/README.md", README,)?;
	append_path(name.clone() + "/Cargo.toml", CARGO_TOML,)?;
	Ok((),)
}
