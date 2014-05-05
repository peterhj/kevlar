use internal::server::{Server};

use std::str::{from_utf8_lossy};

pub struct KevlarServer {
  _inner: Server,
}

impl KevlarServer {
  pub fn new() -> KevlarServer {
    KevlarServer{
      _inner: Server::new(),
    }
  }

  pub fn raw_get(&mut self, key: ~[u8]) -> ~[u8] {
    self._inner.get(key)
  }

  pub fn get(&mut self, key: ~str) -> ~[u8] {
    self._inner.get(key.into_bytes())
  }

  pub fn get_str(&mut self, key: ~str) -> ~str {
    let bytes = self.get(key);
    from_utf8_lossy(bytes).into_owned()
  }

  pub fn raw_put(&mut self, key: ~[u8], value: ~[u8]) {
    self._inner.put(key, value);
  }

  pub fn put(&mut self, key: ~str, value: ~[u8]) {
    self._inner.put(key.into_bytes(), value);
  }

  pub fn put_str(&mut self, key: ~str, value: ~str) {
    self._inner.put(key.into_bytes(), value.into_bytes());
  }

  pub fn delete(&mut self, key: ~[u8]) {
    self._inner.delete(key);
  }
}
