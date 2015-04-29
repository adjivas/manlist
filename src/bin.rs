// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/manlist/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate manslib;

fn main() {
  match std::env::var("MANPATH") {
    Err(why) => println!("the MANPATH from environement is empty!: {:?}", why),
    Ok(manpath) => {
      let mans: Vec<manslib::mans::Man> = manslib::mans::from_env(
        &manpath.split(":").map(|x|
          x.to_string()
        ).collect()
      );
      let mut line:String = String::new();
      loop {
        let out:String = match std::io::stdin().read_line(&mut line) {
          Ok(_) => {
             let binary:String = line.chars().take_while(|x|
              *x != '\n'
            ).collect();
            manslib::mans::display(&mans, &binary)
          },
          Err(why) => panic!("Unable to read to stderr: {}", why),
        };
        match std::io::Write::write(&mut std::io::stderr(), &out.into_bytes()) {
          Ok(_) => line.clear(),
          Err(why) => panic!("Unable to write to stderr: {}", why),
        }
      }
    },
  }
}
