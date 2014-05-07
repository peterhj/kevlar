extern crate kevlar;

use kevlar::db::{KevlarDb};

fn main() {
  println!("Hello, world!");
  let mut db = KevlarDb::new();
  db.put_str(~"foo", ~"bar");
  db.put_str(~"hello", ~"world");
  db.put_str(~"asdf", ~"qwerty");
  db.delete(~"asdf");
  println!("{}", db.get_str(~"foo"));
  println!("{}", db.get_str(~"hello"));
  //println!("{}", db.get_str(~"asdf"));
}
