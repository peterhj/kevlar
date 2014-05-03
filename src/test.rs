extern crate kevlar;

use kevlar::server::{KevlarServer};

fn main() {
  println!("Hello, world!");
  let mut server = KevlarServer::new();
  /*let mut txn = server.begin_txn();
  txn.put_str(~"foo", ~"bar");
  txn.put_str(~"hello", ~"world");
  txn.put_str(~"asdf", ~"qwerty");
  server.commit(txn);*/
  println!("{}", server.get_str(~"foo"));
  println!("{}", server.get_str(~"hello"));
  println!("{}", server.get_str(~"asdf"));
  let mut txn = server.begin_txn();
  txn.put_str(~"asdf", ~"qwerty");
  server.commit(txn);
}
