#![feature(fs, io, path, collections)]
#![feature(os)]

pub mod mans {
  use std::fs::walk_dir;
  use std::fs::File;
  use std::io::BufReader;
  use std::io::BufRead;

  pub struct Command {
    names: Vec<String>,
    description: String,
  }

  struct Argument {
    option: String,
    description: Vec<String>,
  }

  struct File_1 {
    open: File,
    name: String,
  }

  fn manlist(
    roots: &Vec<String>
  ) -> Vec<Man> {
    let mut mans: Vec<Man> = Vec::with_capacity(roots.capacity());
    for path in roots {
      match walk_dir(&Path::new(path)) {
        Err(why) => println!("walk_dir {:?}", why.kind()),
        Ok(paths) => {
          for path in paths {
            let buf = path.unwrap().path();
            if buf.extension().is_some()
            && buf.extension().unwrap() == "1" {
              match File::open(&buf) {
                Err(why) => println!("Could not open {:?}: {}", buf.file_name().unwrap(), why.description()),
                Ok(file_open) => {
                  match buf.file_name().unwrap().to_os_string().into_string() {
                    Err(_) => panic!("Could not into_string"),
                    Ok(mut file_name) => {
                      file_name.pop();
                      file_name.pop();
                      mans.push(Man {
                        file: File_1 {
                          open: file_open,
                          name: file_name,
                        },
                        name: String::new(),
                        work: false,
                        command: Command {
                          names: Vec::new(),
                          description: String::new(),
                        },
                        arguments: Vec::new(),
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

  /// The `line_clear_to` function first clears, moves the `line` variable
  /// to `find` and returns a boolean.
  fn line_clear_to(
    buff: &mut BufReader<&File>,
    line: &mut String,
    find: &str
  ) -> bool {
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
    buff: &mut BufReader<&File>,
    line: &mut String,
    finds: &[&str]
  ) -> u8 {
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

  pub struct Man {
    file: File_1,
    name: String,
    work: bool,
    command: Command,
    arguments: Vec<Argument>,
  }

  trait Man { 

    /// The `read_gnu_command_names` function returns the command
    /// names from the man's gnu ("bc", "echo", [...]).
    fn read_gnu_command_names(
      line: &mut String
    ) -> Vec<String> {
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

    /// The `read_gnu_command_description` function returns the description
    /// from the man's gnu ("An arbitrary...", [...]).
    fn read_gnu_command_description(
      line: &mut String
    ) -> String {
      let mut description: String = line.replace(",", "").replace("\\", "");

      description = description.trim().chars().collect();
      description
    }

    /// The `read_gnu_argument_comment` function returns a list of
    /// comment's arguments from the man's gnu ("Print the...", [...]).
    fn read_gnu_argument_comment (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Vec<String>, String> {
      let mut description:String = String::new();
      let mut descriptions:Vec<String> = Vec::new();

      line.clear();
      while buff.read_line(line).is_ok()
      && !line.is_empty()
      && !line.find_str(".SH").is_some()
      && !line.find_str("\\-").is_some()
      && !line.find_str(".TP").is_some() {
        description = line.trim().to_string();
        if !description.is_empty()
        && description.as_bytes()[0] == 46 {
          description = description.chars().skip_while(|x| *x != ' ').collect();
        }
        description = description.replace("\\\\fP", "");
        description = description.replace("\\\\fI", "");
        description = description.replace("\\f", "");
        description = description.replace("\\&", "");
        description = description.replace("\\ ", " ");
        description = description.replace("\\", "\\");
        description = description.replace("\\\"", "\"");
        description = description.replace("\\'", "'");
        if !description.is_empty() {
          descriptions.push(description);
        }
        line.clear();  
      }
      if descriptions.len() <= 0 {
        return Err(line.to_string());
      }
      Ok(descriptions)
    }

    /// The `read_gnu_argument_option` function returns a list of
    /// option's arguments from the man's gnu ([-h, --help], [[...], ...]).
    fn read_gnu_argument_option (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<String, String> {
      let mut option:String = String::new();

      line.clear();
      buff.read_line(line);
      option = line.trim().to_string();
      option = option.chars().take_while(|x| *x != '"').collect();
      option = option.chars().skip(3).collect();
      if option.find_str("^").is_some() {
        option = option.chars().skip_while(|x| *x != '^').collect();
        option = option.chars().take_while(|x| *x != ' ').collect();
        option = option.replace("^", "-");
      }
      else if option.find_str("-").is_some() || option.find_str("\\").is_some() {
        option = option.replace("\\-", "-");
        option = option.replace("\\\\fR", "\n");
        option = option.chars().take_while(|x| *x != '\n' || *x != ' ').collect();
        option = option.replace("*=", "--");
      }
      option = option.chars().take_while(|x| *x != '\\').collect();
      option = option.replace(" ", "");
      if !option.is_empty()
      && (option.as_bytes()[0] == 45
      || option.len() == 1) {
        return (Ok(option));
      }
      Err(option)
    }

    /// The `read_gnu_command` function returns the name and the description
    /// from the man's gnu command ("bc", "An arbitrary...").
    fn read_gnu_command (
      buff: &mut BufReader<&File>,
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
              command_names = Man::read_gnu_command_names(&mut name.to_string());
              match command.next() {
                Some(description) => {
                  
                  return Ok(Command {
                    names: command_names,
                    description: Man::read_gnu_command_description(&mut description.to_string()),
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

    /// The `read_gnu_argment` function returns all options and comments
    /// from the man's gnu command  (["-h", "Print..."], ["-i", "Force..."]).
    fn read_gnu_argument (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) {
      let mut arguments:Vec<Argument> = Vec::new();

      if line_to_multy(buff, line, &["OPTIONS"]) > 0 {
        line.clear();
        while line_to_multy(buff, line, &[".TP"]) > 0 {
          match Man::read_gnu_argument_option(buff, line) {
            Ok(option) => {
              match Man::read_gnu_argument_comment(buff, line) {
                Ok(description) => {
                  arguments.push(Argument {
                    option: option,
                    description: description,
                  });
                },
                Err(why) => {
                },
              }
            },
            Err(why) => {
            },
          }
        }
      }
    }

    /// The `read_gnu` function checks and parses the: name,
    /// description, options and commants from a man's gnu.
    fn read_gnu(
      buff: &mut BufReader<&File>,
      line: &mut String
    ) {
      match Man::read_gnu_command(buff, line) {
        Ok(command) => {
          Man::read_gnu_argument(buff, line);
        },
        Err(why) => {
        },
      }
    }

    /// The `read_unix_command_names` function returns the command
    /// names from the man's unix ("bc", "echo", [...]).
    fn read_unix_command_names(
      line: &mut String
    ) -> Vec<String> {
      let words: String = line.replace(",", "").trim().chars().skip(4).collect();
      let mut names: Vec<String> = Vec::new();

      for name in words.split_str(" ") {
        if !name.is_empty() {
          names.push(name.to_string());
        }
      }
      names
    }

    /// The `read_unix_command_description` function returns the description
    /// from the man's unix ("An arbitrary...", [...]).
    fn read_unix_command_description(
      line: &mut String
    ) -> String {
      let mut description: String = line.replace(",", "");

      description = description.trim().chars().skip(4).collect();
      description = description.chars().take_while(|x| *x != '\\').collect();
      description
    }

    /// The `read_unix_argument_option` function returns a list of
    /// option's arguments from the man's unix ([-h, --help], [[...], ...]).
    fn read_unix_argument_option(
      line: &mut String
    ) -> Result<String, String> {
      let mut result:String = String::from_str("-");
      let mut option:String = String::new();

      option = line.trim().to_string();
      option = option.replace("\\-", "-");
      option = option.replace("\\&", "");
      option = option.chars().skip(7).collect();
      option = option.chars().take_while(|x| *x != ' ' && *x != '\\').collect();
      if option.is_empty() {
        return Err(line.to_string());
      }
      result.push_str(option.as_slice());
      Ok(result)
    }

    /// The `read_unix_argument_comment` function returns a list of
    /// comment's arguments from the man's unix ("Print the...", [...]).
    fn read_unix_argument_comment(
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Vec<String>, String> {
      line.clear();
      let mut description:String = String::new();
      let mut descriptions:Vec<String> = Vec::new();

      while buff.read_line(line).is_ok()
      && !line.is_empty()
      && 46 != line.as_bytes()[0] {
        description = line.to_string();
        description = description.replace("\\fI", "");
        description = description.replace("\\fR", "");
        description = description.replace("\\fB", "");
        description = description.replace("\\-", "-");
        description = description.replace("\\(aa", "");
        if !description.is_empty() {
          println!("\t\t{}", description);
          descriptions.push(description);
        }
        line.clear();  
      }
      if descriptions.len() <= 0 {
        return Err(line.to_string());
      }
      Ok(descriptions)
    }

    /// The `read_unix_command` function returns the name and the description
    /// from the man's unix command ("bc", "An arbitrary...").
    fn read_unix_command(
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Command, String> {
      let mut command_names: Vec<String> = Vec::new();

      if line_clear_to(buff, line, ".Nm") {
        command_names = Man::read_unix_command_names(line);
        if line_clear_to(buff, line, ".Nd") {
          return Ok(Command {
            names: command_names,
            description: Man::read_unix_command_description(line),
          });
        }
      }
      Err("H: invalid format's command".to_string())
    }

    /// The `read_unix_argument` function returns all options and comments
    /// from the man's unix command  (["-h", "Print..."], ["-i", "Force..."]).
    fn read_unix_argument(
      buff: &mut BufReader<&File>,
      line: &mut String
    ) {
      let mut arguments:Vec<Argument> = Vec::new();

      while line_clear_to(buff, line, ".It Fl ") {
        match Man::read_unix_argument_option(line) {
          Ok(option) => {
            println!("{}", option);
            Man::read_unix_argument_comment(buff, line);
          },
          Err(why) => {
          },
        }
      }
    }

    /// The `read_unix` function checks and parses the: name,
    /// description, options and commants from a man's unix.
    fn read_unix(
      buff: &mut BufReader<&File>,
      line: &mut String
    ) {
      match Man::read_unix_command(buff, line) {
        Ok(command) => {
          Man::read_unix_argument(buff, line);
        },
        Err(why) => {
        },
      }
    }

    fn new(
      man: &Man
    ) -> bool {
      let mut buff = BufReader::new(&man.file.open);
      let mut line = String::new();

      match line_to_multy(&mut buff, &mut line, &[".Sh", ".SH"]) {
        104 => {
          print!("{}\n", man.file.name);
          Man::read_unix(&mut buff, &mut line);
          println!("");
        },
        72 => {
          Man::read_gnu(&mut buff, &mut line);
        },
        _ => return false,
      }
      false
    }
  }

  /// Returns a new MAN Object.
  pub fn new(
    manpath: String
  ) -> Self {
    let roots: Vec<String> = manpath.split_str(":").map(|x| x.to_string()).collect();
    let mut mans: Vec<Man> = manlist(&roots);
    for man in mans.iter() {
      read(&man);
    }
    Self
  }
}

fn main() {
  let key = "MANPATH";

  match std::os::getenv(key) {
    None => println!("the ${:?} from environement is empty! ", key),
    Some(manpath) => {
      let man = mans::Man::new(manpath);
    },
  }
}
