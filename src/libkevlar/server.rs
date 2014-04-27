use internal::server::{Server};
use internal::txn::{Txn};

pub struct KevlarTxn {
  _inner: Txn,
}

impl KevlarTxn {
  pub fn put(&mut self, key: ~[u8], value: ~[u8]) {
    self._inner.put(key, value);
  }

  pub fn delete(&mut self, key: ~[u8]) {
    self._inner.delete(key);
  }
}

pub struct KevlarServer {
  _inner: Server,
}

impl KevlarServer {
  pub fn new() -> KevlarServer {
    KevlarServer{
      _inner: Server::new(),
    }
  }

  pub fn begin_txn(&mut self) -> KevlarTxn {
    KevlarTxn{
      _inner: self._inner.begin_txn(),
    }
  }

  pub fn get(&mut self, key: ~[u8]) -> ~[u8] {
    self._inner.get(key)
  }

  pub fn commit(&mut self, txn: KevlarTxn) {
    self._inner.commit(txn._inner);
  }
}
