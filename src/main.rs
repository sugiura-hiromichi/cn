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
	let mut arg_line = "new ".to_string();
	args.for_each(|a| {
		arg_line.push_str(&a,);
		arg_line.push(' ',);
	},);
	sh_cmd!("cargo", arg_line.split_whitespace())?;

	let name = arg_line.split_whitespace().last().unwrap().to_string();
	if arg_line.contains("--lib",) {
		//When to lib package
		fs::write(format!("{name}/src/lib.rs"), LIB_RS,)?;
	} else {
		//When to bin package
		fs::write(format!("{name}/src/main.rs"), MAIN_RS,)?;
		fs::write(format!("{name}/.gitignore"), GITIGNORE,)?;
	}

	fs::write(format!("{name}/README.md"), README,)?;
	append_path(format!("{name}/Cargo.toml"), CARGO_TOML,)?;
	Ok((),)
}

#[cfg(test)]
mod tests {
	//use super::*;

	#[test]
	fn sw() {
		let s = "a b c ".split_whitespace().collect::<Vec<&str,>>();
		assert_eq!(s, vec!["a", "b", "c"]);
	}
}
