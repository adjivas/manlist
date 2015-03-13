#![feature(fs, io, path, collections)]
use std::os;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::AsPath;
use std::borrow::ToOwned;

fn list(roots: &Vec<String>) -> Vec<File> {
  let mut files: Vec<File> = Vec::with_capacity(roots.capacity());
  for path in roots {
    match std::fs::walk_dir(&Path::new(path)) {
      Err(why) => println!("walk_dir {:?}", why.kind()),
      Ok(paths) => {
        for path in paths {
          let buf = path.unwrap().path();
          if buf.extension().is_some()
          && buf.extension().unwrap() == "1" {
            match File::open(&buf) {
              Err(why) => println!("Could not open {:?}: {}", buf.file_name().unwrap(), why.description()),
              Ok(file) => {
                files.push(file)
              },
            }           
          }
        }
      },
    }
  }
  files
}

fn main() {
  let key = "MANPATH";

  match os::getenv(key) {
    None => println!("the ${:?} from environement is empty! ", key),
    Some(manpath) => {
      let roots: Vec<String> = manpath.split_str(":").map(|x| x.to_string()).collect();

      let files: Vec<File> = list(&roots);

      for file in files {
        let mut content = BufReader::new(file);
        let mut line = String::new();

        while content.read_line(&mut line).is_ok() && !line.is_empty() {
          println!("{:?}", line);
          break ;
          line.clear();
        }
      }
    },
  }
}