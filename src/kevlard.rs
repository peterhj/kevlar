extern crate kevlar;

use kevlar::server::{KevlarServer};

use std::str::{from_utf8_lossy};

fn main() {
  println!("Hello, world!");
  let mut server = KevlarServer::new();
  /*let mut txn = server.begin_txn();
  txn.put((~"foo").into_bytes(), (~"bar").into_bytes());
  txn.put((~"hello").into_bytes(), (~"world").into_bytes());
  server.commit(txn);*/
  //println!("{}", server.get((~"asdf").into_bytes()));
  println!("{}", from_utf8_lossy(server.get((~"foo").into_bytes())).as_slice());
  println!("{}", from_utf8_lossy(server.get((~"hello").into_bytes())).as_slice());
}
