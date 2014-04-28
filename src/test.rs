extern crate kevlar;

use kevlar::server::{KevlarServer};

fn main() {
  println!("Hello, world!");
  let mut server = KevlarServer::new();
  println!("{}", server.get_str(~"foo"));
  println!("{}", server.get_str(~"hello"));
  println!("{}", server.get_str(~"asdf"));
  let mut txn = server.begin_txn();
  txn.put((~"asdf").into_bytes(), (~"qwerty").into_bytes());
  server.commit(txn);
}
