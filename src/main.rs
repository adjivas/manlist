#![feature(fs, io, path, collections)]
use std::os;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::AsPath;
use std::borrow::ToOwned;

struct Man {
  open: File,
  name_file: String,
  name_man: String,
  error: bool
}

fn list(roots: &Vec<String>) -> Vec<Man> {
  let mut mans: Vec<Man> = Vec::with_capacity(roots.capacity());
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
                match buf.file_name().unwrap().to_os_string().into_string() {
                  Err(why) => panic!("Could not into_string"),
                  Ok(mut name_file) => {
                    name_file.pop();
                    name_file.pop();
                    mans.push(Man {
                      open: file,
                      name_file: name_file,
                      name_man: String::new(),
                      error: true,
                    })
                  },
                }
              },
            }
          }
        }
      },
    }
  }
  mans
}

fn next(content: &mut BufReader<&std::fs::File>, line: &mut String) {
  line.clear();
  content.read_line(line).is_ok();
}

fn read(man: &Man) {
  let mut content = BufReader::new(&man.open);
  let mut line = String::new();

  println!("{}", man.name_file);
  while content.read_line(&mut line).is_ok() && !line.is_empty() {
    if line.find_str("NAME").is_some() {
      next(&mut content, &mut line);
      if line.find_str(".\\\"").is_some() {
        next(&mut content, &mut line);
      }
      if line.find_str("- ").is_some() {
        println!("\t{:?}", line);
      }
      line.clear();
      break ;
    }
    line.clear();
  }
  while content.read_line(&mut line).is_ok() && !line.is_empty() {
    if line.find_str("OPTIONS").is_some()
    || line.find_str("options").is_some()
    || line.find_str("option").is_some() {
      line.clear();
      while content.read_line(&mut line).is_ok() && !line.is_empty() {
        if !line.find_str(".TP\n").is_some()
        && !line.find_str(".PP\n").is_some()
        && !line.find_str(".Pp\n").is_some()
        && !line.find_str(".RS\n").is_some()
        && !line.find_str(".ft ").is_some()
        && !line.find_str(".SH ").is_some()
        && !line.find_str(".Nm ").is_some()
        && !line.find_str(".Ql ").is_some()
        && !line.find_str(".Sq ").is_some()
        && !line.find_str(".Ar ").is_some()
        && !line.find_str(".Fl ").is_some()
        && !line.find_str(".St ").is_some()
        && !line.find_str(".IR ").is_some()
        && !line.find_str(".Pa ").is_some()
        && !line.find_str(".Xr ").is_some()
        && !line.find_str(".BR ").is_some() {
          if line.find_str(".B ").is_some()
          || line.find_str(".IP ").is_some()
          || line.find_str(".\\\"").is_some()
          || line.find_str(".Bl ").is_some()
          || line.find_str(".BI ").is_some()
          || line.find_str(".It ").is_some() {
            print!("\t\t{}", line);
          }
          else if 46 != line.as_bytes()[0]
          && 10 != line.as_bytes()[0] {
            print!("\t\t\t{}", line);
          }
          else {
            print!("E\t\t\t{}", line);
            break ;
          }
        }
        line.clear();
      }
      break ;
    }
    line.clear();
  }
  //let result: String = line.chars().skip_while(|x| *x != ' ' ).skip(4).collect();
}

fn main() {
  let key = "MANPATH";

  match os::getenv(key) {
    None => println!("the ${:?} from environement is empty! ", key),
    Some(manpath) => {
      let roots: Vec<String> = manpath.split_str(":").map(|x| x.to_string()).collect();
      let mut mans: Vec<Man> = list(&roots);
      for man in mans.iter() {
        if man.name_file == "top" {
          read(&man);
        }
      }
    },
  }
}
