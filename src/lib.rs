#![feature(collections)]
#![feature(core)]
#![feature(fs_walk)]

pub mod mans {
  use std::fs::walk_dir;
  use std::fs::File;
  use std::io::BufReader;
  use std::io::BufRead;
  use std::path::PathBuf;

  /// The `command` structure is a name ("bc") and
  /// a description ("An arbitrary precision...").
  pub struct Command {
    pub names: Vec<String>,
    pub description: String,
  }

  impl Command {

    /// The `new` constructor function returns a new Command.
    fn new (
      names: Vec<String>,
      description: String,
    ) -> Self {
      Command {
        names: names,
        description: description,
      }
    }

    /// The `from_gnu` constructor function returns the name and the
    /// description from the man's gnu command ("bc", "An arbitrary...").
    fn from_gnu (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Self, String> {
      if buff.read_line(line).is_ok() {
        let command: String = line.replace("\\- ", "- ");

        if line.find("- ").is_some() {
          let mut command = command.split("- ");
          match command.next() {
            Some(line_name) => {
              match Command::gnu_names(&mut line_name.to_string()) {
                Ok(names) => {
                  match command.next() {
                    Some(line_description) => {
                      match Command::gnu_description(
                        &mut line_description.to_string()
                      ) {
                        Ok(description) => {
                          return Ok(Command::new(
                            names,
                            description,
                          ));
                        },
                        Err(_) => {
                        },
                      }
                    },
                    None => {},
                  }
                },
                Err(_) => {},
              }
            },
            None => {},
          }
        }
      }
      Err(String::from_str("invalid gnu's command"))
    }

    /// The `from_unix` constructor function returns the name and the
    /// description from the man's unix command ("bc", "An arbitrary...").
    fn from_unix (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Self, String> {
      if line_clear_to(buff, line, ".Nm") {
        match Command::unix_names(line) {
          Ok(names) => {
            if line_clear_to(buff, line, ".Nd") {
              match Command::unix_description(line) {
                Ok(description) => {
                  return Ok(Command::new(
                    names,
                    description,
                  ));
                },
                Err(_) => {},
              }
            }
          },
          Err(_) => {},
        }
      }
      Err(String::from_str("invalid unix's command")) 
    }

    /// The `gnu_names` function returns the command
    /// names from the man's gnu ("bc", "echo", [...]).
    fn gnu_names (
      line: &mut String
    ) -> Result<Vec<String>, String> {
      let mut lines: String = line.replace(",", "");
      let mut names: Vec<String> = Vec::new();

      if 92 == lines.as_bytes()[0] {
        lines = lines.chars().skip(3).collect();
      }
      lines = lines.chars().take_while(|x| *x != '\\').collect();
      for name in lines.split(" ") {
        if !name.is_empty() {
          names.push(name.to_string());
        }
      }
      if names.len() > 0 {
        return Ok(names);
      }
      Err(lines)
    }

    /// The `gnu_description` function returns the description
    /// from the man's gnu ("An arbitrary...", [...]).
    fn gnu_description (
      line: &mut String
    ) -> Result<String, String> {
      let mut description: String = line.replace(",", "").replace("\\", "");

      description = description.trim().chars().collect();
      if !description.is_empty() {
        return Ok(description);
      }
      Err(description)
    }

    /// The `unix_names` function returns the command
    /// names from the man's unix ("bc", "echo", [...]).
    fn unix_names (
      line: &mut String
    ) -> Result<Vec<String>, String> {
      let lines: String = line.replace(",", "").trim().chars().skip(4).collect();
      let mut names: Vec<String> = Vec::new();

      for name in lines.split(" ") {
        if !name.is_empty() {
          names.push(name.to_string());
        }
      }
      if names.len() > 0 {
        return Ok(names);
      }
      Err(lines)
    }

    /// The `unix_description` function returns the description
    /// from the man's unix ("An arbitrary...", [...]).
    fn unix_description (
      line: &mut String
    ) -> Result<String, String> {
      let mut description: String = line.replace(",", "");

      description = description.trim().chars().skip(4).collect();
      description = description.chars().take_while(|x| *x != '\\').collect();
      description = description.replace("\"", "");
      if !description.is_empty() {
        return Ok(description);
      }
      Err(description)
    }
  }

  /// The `argument` structure is the first option ("-h") and
  /// all comments ("Print the usage and exit.", ...).
  pub struct Argument {
    pub option: String,
    pub comments: Vec<String>,
  }

  impl Argument {

    /// The `Argument::new` Constructor function returns a new Argument.
    fn new (
      option: String,
      comments: Vec<String>,
    ) -> Self {
      Argument {
        option: option,
        comments: comments,
      }
    }

    /// The `from_gnu` Constructor function returns all options and comments
    /// from the man's gnu command  (["-h", "Print..."], ["-i", "Force..."]).
    fn from_gnu (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Vec<Self>, String> {
      let mut arguments:Vec<Argument> = Vec::new();

      if line_to_multy(buff, line, &["OPTIONS"]) > 0 {
        line.clear();
        while line_to_multy(buff, line, &[".TP"]) > 0 {
          match Argument::gnu_option(buff, line) {
            Ok(option) => {
              match Argument::gnu_comment(buff, line) {
                Ok(description) => {
                  arguments.push(Argument::new(
                    option,
                    description,
                  ));
                },
                Err(_) => {},
              }
            },
            Err(_) => {},
          }
        }
      }
      if arguments.len() > 0 {
        return Ok(arguments);
      }
      Err(String::from_str("invalid gnu's argument"))
    }

    /// The `from_unix` Constructor function returns all options and comments
    /// from the man's unix command  (["-h", "Print..."], ["-i", "Force..."]).
    fn from_unix (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Vec<Self>, String> {
      let mut arguments:Vec<Argument> = Vec::new();

      while line_clear_to(buff, line, ".It Fl ") {
        match Argument::unix_option(line) {
          Ok(option) => {
            match Argument::unix_comment(buff, line) {
              Ok(description) => {
                arguments.push(Argument::new(
                  option,
                  description,
                ));
              },
              Err(_) => {},
            }
          },
          Err(_) => {},
        }
      }
      if arguments.len() <= 0 {
        return Err(String::from_str("invalid unix's argument"));
      }
      Ok(arguments) 
    }

    /// The `unix_option` function returns a list of
    /// option's arguments from the man's unix ([-h, --help], [[...], ...]).
    fn unix_option (
      line: &mut String
    ) -> Result<String, String> {
      let mut option:String = String::from_str("-");
      let mut opt:String = line.trim().to_string();

      opt = opt.replace("\\-", "-");
      opt = opt.replace("\\&", "");
      opt = opt.chars().skip(7).collect();
      opt = opt.chars().take_while(
        |x| *x != ' ' &&
        *x != '\\' &&
        *x != '"' &&
        *x != ','
      ).collect();
      if !opt.is_empty() {
        option.push_str(opt.as_slice());
        return Ok(option);
      }
      Err(line.to_string())
    }

    /// The `unix_comment` function returns a list of
    /// comment's arguments from the man's unix ("Print the...", [...]).
    fn unix_comment (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Vec<String>, String> {
      line.clear();
      let mut descriptions:Vec<String> = Vec::new();
      while buff.read_line(line).is_ok()
      && !line.is_empty()
      && 46 != line.as_bytes()[0] {
        let mut description:String = line.to_string();

        description = description.replace("\\fI", "");
        description = description.replace("\\fR", "");
        description = description.replace("\\fB", "");
        description = description.replace("\\-", "-");
        description = description.replace("\\(aa", "");
        if !description.is_empty() {
          descriptions.push(description);
        }
        line.clear();  
      }
      if descriptions.len() > 0 {
        return Ok(descriptions);
      }
      Err(line.to_string())
    }

    /// The `gnu_comment` function returns a list of
    /// comment's arguments from the man's gnu ("Print the...", [...]).
    fn gnu_comment (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Vec<String>, String> {
      let mut descriptions:Vec<String> = Vec::new();

      line.clear();
      while buff.read_line(line).is_ok() && !line.is_empty()
      && !line.find(".SH").is_some()
      && !line.find("\\-").is_some()
      && !line.find(".TP").is_some() {
        let mut description:String = line.trim().to_string();
        if !description.is_empty() && description.as_bytes()[0] == 46 {
          description = description.chars().skip_while(
            |x| *x != ' '
          ).collect();
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
        return Ok(descriptions);
      }
      Err(line.to_string())      
    }

    /// The `gnu_option` function returns a list of
    /// option's arguments from the man's gnu ([-h, --help], [[...], ...]).
    fn gnu_option (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<String, String> {
      let mut option:String = line.trim().to_string();

      line.clear();
      if buff.read_line(line).is_ok() {
        option = option.chars().take_while(|x| *x != '"').collect();
        option = option.chars().skip(3).collect();
        if option.find("^").is_some() {
          option = option.chars().skip_while(|x| *x != '^').collect();
          option = option.chars().take_while(|x| *x != ' ').collect();
          option = option.replace("^", "-");
        }
        else if option.find("-").is_some()
        || option.find("\\").is_some() {
          option = option.replace("\\-", "-");
          option = option.replace("\\\\fR", "\n");
          option = option.chars().take_while(
            |x| *x != '\n' || *x != ' '
          ).collect();
          option = option.replace("*=", "--");
        }
        option = option.chars().take_while(|x| *x != '\\').collect();
        option = option.replace(" ", "");
        if !option.is_empty()
        && option.as_bytes()[0] == 45 {
          return Ok(option);
        }
      }
      Err(option)
    }
  }

  /// The `mans` structure is a defined by the two
  /// structures command and Argument
  pub struct Man {
    pub command: Command,
    pub arguments: Vec<Argument>,
  }

  impl Man {
    /// The `new` constructor function returns a new Man.
    pub fn new (
      command: Command,
      arguments: Vec<Argument>,
    ) -> Self {
      Man {
        command: command,
        arguments: arguments,
      }
    }

    /// The `from_open` constructor function returns a new Man
    /// according the path.
    pub fn from_open (
      path: PathBuf,
    ) -> Result<Self, String> {
      return match File::open(&path) {
        Err(why) => Err(why.to_string()),
        Ok(open) => Man::from_buff(open),
      }
    }

    /// The `from_buff` constructor function returns a new Man
    /// according to a file descriptor.
    pub fn from_buff (
      open: File,
    ) -> Result<Self, String> {
      let mut buff = BufReader::new(&open);
      let mut line:String = String::new();
      return match line_to_multy(
        &mut buff,
        &mut line,
        &[".Sh", ".SH"],
      ) {
        104 => Man::read_unix(&mut buff, &mut line),
        72 => Man::read_gnu(&mut buff, &mut line),
        _ => Err(String::from_str("unknown man")),
      }
    }

    /// The `read_gnu` function checks and parses the: name,
    /// description, options and commants from a man's gnu.
    fn read_gnu (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Self, String> {
      line.clear();
      match Command::from_gnu(buff, line) {
        Ok(command) => {
          match Argument::from_gnu(buff, line) {
            Ok(argument) => return Ok(Man::new(
              command,
              argument,
            )),
            Err(_) => {},
          }
        },
        Err(_) => {},
      }
      Err(String::from_str("invalid gnu's man"))
    }

    /// The `read_unix` function checks and parses the: name,
    /// description, options and commants from a man's unix.
    fn read_unix (
      buff: &mut BufReader<&File>,
      line: &mut String
    ) -> Result<Self, String> {
      line.clear();
      match Command::from_unix(buff, line) {
        Ok(command) => {
          match Argument::from_unix(buff, line) {
            Ok(argument) => return Ok(Man::new(
              command,
              argument,
            )),
            Err(_) => {},
          }
        },
        Err(_) => {},
      }
      Err(String::from_str("invalid unix's man"))
    }
  }

  /// The `new` constructor function return a valid list
  /// of man.
  pub fn from_string (
    manpath: String
  ) -> Vec<Man> {
    let roots: Vec<String> = manpath.split(":").map(
      |x| x.to_string()
    ).collect();
    from_root(&roots) 
  }

  /// The `new` constructor function returns a list of valid man
  /// according to a list of possibely roots for .1's files.
  pub fn from_root (
    roots: &Vec<String>
  ) -> Vec<Man> {
    let mut mans: Vec<Man> = Vec::with_capacity(roots.capacity());
    for path in roots {
      match walk_dir(&PathBuf::from(path)) {
        Err(why) => println!("walk_dir {:?}", why.kind()),
        Ok(paths) => {
          for path in paths {
            let buf:PathBuf = path.unwrap().path();
            if buf.extension().is_some()
            && buf.extension().unwrap() == "1" {
              match Man::from_open(buf) {
                Err(_) => {

                },
                Ok(man) => {
                  mans.push(man);
                },
              }
            }
          }
        },
      }
    }
    mans
  }

  /// The `line_clear_to` function first clears, moves
  /// the `line` variable to `find` and returns a boolean.
  fn line_clear_to (
    buff: &mut BufReader<&File>,
    line: &mut String,
    find: &str
  ) -> bool {
    line.clear();
    while buff.read_line(line).is_ok()
    && !line.is_empty() {
      if line.find(find).is_some() {
        return true;
      }
      line.clear();
    }
    false
  }

  /// The `line_to_multy` function moves the `line` variable to `finds`,
  /// founds the first egality for returns the two letters or returns zero.
  fn line_to_multy (
    buff: &mut BufReader<&File>,
    line: &mut String,
    finds: &[&str]
  ) -> u8 {
    while buff.read_line(line).is_ok()
    && !line.is_empty() {
      for find in finds {
        if line.find(find).is_some() {
          return line.as_bytes()[2];
        }
      }
      line.clear();
    }
    0
  }
}
