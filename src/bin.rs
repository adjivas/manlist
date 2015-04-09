extern crate manslib;

fn main() {
  let key = "MANPATH";
  match std::env::var(key) {
    Err(why) => println!("the ${} from environement is empty!: {:?}", key, why),
    Ok(manpath) => {
      let mans: Vec<manslib::mans::Man> = manslib::mans::from_string(manpath);

      for man in mans.iter() {
        for names in man.command.names.iter() {
          println!("command.names:\t\t{}", names);
        }
        println!("command.description:\t\t{}", man.command.description);
        for argument in man.arguments.iter() {
          println!("argument.option:\t\t\t{}", argument.option);
          for comment in argument.comments.iter() {
            print!("argument.comment:\t\t\t\t{}", comment);
          }
        }
      }
    },
  }
}
