#![feature(fs, io, path, collections)]
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::AsPath;

fn list(roots: &Vec<String>) {
  let mut files: Vec<File> = Vec::with_capacity(roots.capacity());
  for path in roots {

    match std::fs::walk_dir(&Path::new(path)) {
      Err(why) => println!("!walk_dir {:?}", why.kind()),
      Ok(paths) => {
        for path in paths {
          let buf = path.unwrap().path();

          if buf.file_name().is_some() {
            //if buf.extension() == "1" {
              println!("{:?}", buf);
              println!("{:?}", buf.file_name());
            //}
          }
        }
      },
    }
  }
}

fn main() {
  let mut roots = Vec::new();

  roots.push("/nfs/zfs-student-5/users/2013/adjivas/.brew/share/man/man1/aclocal-1.15.1".to_string());

  roots.push("/nfs/zfs-student-5/users/2013/adjivas/.brew/share/man".to_string());
  roots.push("/usr/local/share/man".to_string());
  roots.push("/usr/share/man".to_string());
  roots.push("/nfs/zfs-student-5/users/2013/adjivas/.brew/Cellar/rust/1.0.0-alpha.2/share/man".to_string());
  roots.push("/nfs/zfs-student-5/users/2013/adjivas/usr/share/man".to_string());
  roots.push("/Applications/Xcode.app/Contents/Developer/usr/share/man".to_string());
  roots.push("/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/folders.push(share/man".to_string());

  list(&roots);

}