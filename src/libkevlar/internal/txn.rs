pub type TxnId = u64;

pub enum TxnOp {
  Put(~[u8], ~[u8]),
  Delete(~[u8]),
}

pub struct Txn {
  ops: Vec<TxnOp>,
}

impl Txn {
  pub fn new() -> Txn {
    Txn{
      ops: Vec::new(),
    }
  }

  pub fn put(&mut self, key: ~[u8], value: ~[u8]) {
    self.ops.push(Put(key, value));
  }

  pub fn delete(&mut self, key: ~[u8]) {
    self.ops.push(Delete(key));
  }
}
