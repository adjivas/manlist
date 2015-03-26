#![feature(fs, io, path, collections)]
use std::os;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::path::AsPath;
use std::borrow::ToOwned;

struct Command {
  names: Vec<String>,
  description: String,
}

struct File {
  open: std::fs::File,
  name: String,
}

struct Man {
  file: File,
  name: String,
  work: bool,
  command: Command,
}

fn manlist(roots: &Vec<String>) -> Vec<Man> {
  let mut mans: Vec<Man> = Vec::with_capacity(roots.capacity());
  for path in roots {
    match std::fs::walk_dir(&Path::new(path)) {
      Err(why) => println!("walk_dir {:?}", why.kind()),
      Ok(paths) => {
        for path in paths {
          let buf = path.unwrap().path();
          if buf.extension().is_some()
          && buf.extension().unwrap() == "1" {
            match std::fs::File::open(&buf) {
              Err(why) => println!("Could not open {:?}: {}", buf.file_name().unwrap(), why.description()),
              Ok(file_open) => {
                match buf.file_name().unwrap().to_os_string().into_string() {
                  Err(why) => panic!("Could not into_string"),
                  Ok(mut file_name) => {
                    file_name.pop();
                    file_name.pop();
                    mans.push(Man {
                      file: File {
                        open: file_open,
                        name: file_name,
                      },
                      name: String::new(),
                      work: false,
                      command: Command {
                        names: Vec::new(),
                        description: String::new(),
                      },
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

// let result: String = line.chars().skip_while(|x| *x != ' ').skip(4).collect();

/// The `line_clear_to` function first clears, moves the `line` variable
/// to `find` and returns a boolean.

fn line_clear_to(
  buff: &mut BufReader<&std::fs::File>,
  line: &mut String,
  find: &str) -> bool {
  line.clear();
  while buff.read_line(line).is_ok()
  && !line.is_empty() {
    if line.find_str(find).is_some() {
      return true;
    }
    line.clear();
  }
  false
}

/// The `line_to_multy` function moves the `line` variable to `finds`, founds
/// the first egality for returns the two letters or returns zero.

fn line_to_multy(
  buff: &mut BufReader<&std::fs::File>,
  line: &mut String,
  finds: &[&str]) -> u8 {
  while buff.read_line(line).is_ok()
  && !line.is_empty() {
    for find in finds {
      if line.find_str(find).is_some() {
        return line.as_bytes()[2];
      }
    }
    line.clear();
  }
  0
}

/// The `read_command_h_names` function returns a collection of command names
/// from the man.

fn read_command_h_names(line: &mut String) -> Vec<String> {
  let words: String = line.replace(",", "").trim().chars().skip(4).collect();
  let mut names: Vec<String> = Vec::new();

  for name in words.split_str(" ") {
    if !name.is_empty() {
      names.push(name.to_string());
    }
  }
  names
}

/// The `read_command_h_names` function returns the description
/// from the man.

fn read_command_h_description(line: &mut String) -> String {
  let mut description: String = line.replace(",", "");

  description = description.trim().chars().skip(4).collect();
  description = description.chars().take_while(|x| *x != '\\').collect();
  description
}

fn read_command_h(
  buff: &mut BufReader<&std::fs::File>,
  line: &mut String
) -> Result<Command, String> {
  let mut command_names: Vec<String> = Vec::new();

  if line_clear_to(buff, line, ".Nm") {
    command_names = read_command_h_names(line);
    if line_clear_to(buff, line, ".Nd") {
      return Ok(Command {
        names: command_names,
        description: read_command_h_description(line),
      });
    }
  }
  Err("H: invalid format's command".to_string())
}

/// The `read_command_h_names` function returns a collection of command names
/// from the man.

fn read_command_H_names(line: &mut String) -> Vec<String> {
  let mut words: String = line.replace(",", "");
  let mut names: Vec<String> = Vec::new();

  if 92 == words.as_bytes()[0] {
    words = words.chars().skip(3).collect();
  }
  words = words.chars().take_while(|x| *x != '\\').collect();
  for name in words.split_str(" ") {
    if !name.is_empty() {
      names.push(name.to_string());
    }
  }
  names
}

/// The `read_command_h_names` function returns the description
/// from the man.

fn read_command_H_description(line: &mut String) -> String {
  let mut description: String = line.replace(",", "").replace("\\", "");

  description = description.trim().chars().collect();
  description
}

fn read_command_H(
  buff: &mut BufReader<&std::fs::File>,
  line: &mut String
) -> Result<Command, String> {
  let mut command_names: Vec<String> = Vec::new();

  line.clear();
  if buff.read_line(line).is_ok() {
    let command: String = line.replace("\\- ", "- ");
    if line.find_str("- ").is_some() {
      let mut command = command.split_str("- ");
      match command.next() {
        Some(name) => {
          command_names = read_command_H_names(&mut name.to_string());
          match command.next() {
            Some(description) => {
              
              return Ok(Command {
                names: command_names,
                description: read_command_H_description(&mut description.to_string()),
              });
            }
            None => {},
          }
        },
        None => {},
      }
    }
  }
  Err("H: invalid format's command".to_string())
}

/// The `read` function checks and parses the name and description from
/// a man.

fn read_command(
  buff: &mut BufReader<&std::fs::File>,
  line: &mut String
) -> Result<Command, String> {
  match line_to_multy(buff, line, &[".Sh NAME", ".SH NAME", ".SH \"NAME\""]) {
    104 => return read_command_h(buff, line),
    72 => return read_command_H(buff, line),
    _ => return (Err("have not found NAME main".to_string())),
  }
}

/// The `read` function checks and parses the man according to name,
/// description and argument.

fn read(man: &Man) {
  let mut buff = BufReader::new(&man.file.open);
  let mut line = String::new();

  match read_command(&mut buff, &mut line) {
    Ok(command) => {
      println!("{} @ {}", man.file.name, command.description);
    },
    Err(why) => {
      print!("{} - ", man.file.name);
      println!("{}", why);
    },
  }
}


fn main() {
  let key = "MANPATH";

  match os::getenv(key) {
    None => println!("the ${:?} from environement is empty! ", key),
    Some(manpath) => {
      let roots: Vec<String> = manpath.split_str(":").map(|x| x.to_string()).collect();
      let mut mans: Vec<Man> = manlist(&roots);
      for man in mans.iter() {
        //if man.file.name == "pfbtops" {
          read(&man);
        //}
      }
    },
  }
}
