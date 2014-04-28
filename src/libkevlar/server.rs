use cast::{coerce_from_bytes};
use internal::server::{Server};
use internal::txn::{Txn};

use std::cast::{transmute};
use std::str::{from_utf8_lossy};

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

  pub fn get_raw(&mut self, key: ~[u8]) -> ~[u8] {
    self._inner.get(key)
  }

  pub fn get_bytes(&mut self, key: ~str) -> ~[u8] {
    self._inner.get(key.into_bytes())
  }

  pub fn get_str(&mut self, key: ~str) -> ~str {
    let bytes = self.get_bytes(key);
    from_utf8_lossy(bytes).into_owned()
  }

  //pub fn get_vec<T>(&mut self, key: ~str) -> ~Vec<T> {
  //}

  /*pub fn get<T>(&mut self, key: ~str) -> ~T {
    let bytes = self.get_bytes(key);
    let r: &T = coerce_from_bytes(bytes);
    unsafe {
      let ptr: ~T = transmute(r);
      ptr
    }
  }*/

  pub fn commit(&mut self, txn: KevlarTxn) {
    self._inner.commit(txn._inner);
  }
}
