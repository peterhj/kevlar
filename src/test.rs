extern crate kevlar;

use kevlar::server::{KevlarServer};

fn main() {
  println!("Hello, world!");
  let mut server = KevlarServer::new();
  server.put_str(~"foo", ~"bar");
  server.put_str(~"hello", ~"world");
  server.put_str(~"asdf", ~"qwerty");
  println!("{}", server.get_str(~"foo"));
  println!("{}", server.get_str(~"hello"));
  println!("{}", server.get_str(~"asdf"));
}
