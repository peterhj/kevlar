pub type TxnId = u64;

pub struct Txn {
  kvs: Vec<(~[u8], ~[u8])>,
}

impl Txn {
  pub fn new() -> Txn {
    Txn{
      kvs: Vec::new(),
    }
  }

  pub fn put(&mut self, key: ~[u8], value: ~[u8]) {
    self.kvs.push((key, value));
  }

  pub fn delete(&mut self, key: ~[u8]) {
    self.kvs.push((key, ~[]));
  }
}
