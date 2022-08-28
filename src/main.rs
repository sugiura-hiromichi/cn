<<<<<<< HEAD
use {
   mylibrary::sh_cmd,
   std::{
      env, fs,
      io::{self, Write},
      os::unix::process::CommandExt,
      process::Command,
   },
};

const GITIGNORE: &[u8] = b"
Cargo.lock";
const CARGO_TOML: &[u8] = b"
mylibrary={git=\"https://github.com/ah-y/mylibrary\"}";
const MAIN_RS: &[u8] = b"#![allow(unused)]

fn main(){

}";
const LIB_RS: &[u8] = b"#![allow(unused)]";

///If `path` exist, override it's content
fn override_path(path: &str, content: &[u8],) -> io::Result<(),> {
   let mut cntnt = fs::read_to_string(path,)?;
   cntnt = content;
   fs::write(path, cntnt,)
}

///If `path` exist, append it's content
fn append_path(path: &str, content: &[u8],) -> io::Result<(),> {
   let mut cntnt = fs::read_to_string(path,)?;
   cntnt.push_str(content,);
   fs::write(path, cntnt,)
}

//  todo!("==============================================================
//                [CommandLineArgument]
//                        is there any way to handle options & path as
// commandline argument
// ==============================================================");
fn main() -> io::Result<(),> {
   let mut buf = String::new();
   println!("input options & name");
   std::io::stdin().read_line(&mut buf,);

   sh_cmd!("cargo", "new", { buf.split_whitespace() });

   if buf.contains("--lib",) {
      //When to lib package
      override_path("src/lib.rs", LIB_RS,)?;
   } else {
      //When to bin package
      override_path("src/main.rs", MAIN_RS,)?;
      append_path(".gitignore", GITIGNORE,)?;
   }

   append_path("Cargo.toml", CARGO_TOML,)?;

   Ok((),)
}
=======
#![allow(unused)]

fn main(){

}
>>>>>>> refs/remotes/origin/master
