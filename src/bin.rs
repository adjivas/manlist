extern crate manslib;

fn main() {
  match std::env::var("MANPATH") {
    Err(why) => println!("the MANPATH from environement is empty!: {:?}", why),
    Ok(manpath) => {
      let mans: Vec<manslib::mans::Man> = manslib::mans::new(manpath.split(":").map(|x| x.to_string()).collect());

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
