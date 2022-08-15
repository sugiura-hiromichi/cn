use std::{env, fs, io, os::unix::process::CommandExt, process::Command};

const GITIGNORE: &str = "\nCargo.lock";
const MAIN_RS: &[u8] = b"#![allow(unused)]

fn main(){

}";
const LIB_RS: &[u8] = b"";

fn main() -> io::Result<(),> {
   //Detect name
   let mut args: Vec<String,> = env::args().collect();
   args.sort();
   //If there is no name given, set default name "dflt_cargo_prj"
   let name = args.pop().map_or("dflt_cargo_prj".to_string(), |is_name| {
      if is_name.clone().into_bytes()[0] == b"-"[0] {
         "dflt_cargo_prj".to_string()
      } else {
         is_name
      }
   },);

   Command::new("cargo",).arg("new",).args(args.iter(),).arg(name.clone(),).exec();
   Command::new("cd",).arg(name,).exec();

   if env::args().find(|x| x == "--lib",) == None {
      //When to bin package
      fs::write("src/main.rs", MAIN_RS,)?;
   } else {
      //When to lib package
      fs::write("src/lib.rs", LIB_RS,)?;
   }

   let mut ignore = fs::read_to_string(".gitginore",)?;
   ignore.push_str(GITIGNORE,);
   fs::write(".gitginore", ignore,)?;

   Ok((),)
}
