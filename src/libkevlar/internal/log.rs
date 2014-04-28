use cast::{coerce_from_bytes, coerce_to_bytes};
use internal::txn::{TxnId};

use std::io::{Open, SeekEnd, SeekSet, ReadWrite};
use std::io::fs::{File, stat};
use std::mem::{size_of};

#[packed]
pub struct LogHeader {
  check: u64,
  key_size: u32,
  value_size: u32,
  txnid: TxnId,
}

pub struct Log {
  prefix: Path,
  curr_file_id: u32,
  log_file: File,
}

impl Log {
  pub fn new() -> Log {
    let prefix = Path::new("kevlar.log");
    let log_file = match File::open_mode(&prefix, Open, ReadWrite) {
      Ok(file) => file,
      Err(e) => fail!("I/O error while opening log file for appending: {}", e),
    };
    Log{
      prefix: prefix,
      curr_file_id: 0,
      log_file: log_file,
    }
  }

  pub fn eof(&mut self, file_id: u32) -> bool {
    let size = match stat(&self.prefix) {
      Ok(st) => st.size as u32,
      Err(e) => fail!("I/O error while querying log file stat: {}", e),
    };
    let p = self.get_cursor(file_id);
    p < size
  }

  pub fn get_cursor(&self, file_id: u32) -> u32 {
    match self.log_file.tell() {
      Ok(p) => p as u32,
      Err(e) => fail!("I/O error while querying log file pos: {}", e),
    }
  }

  pub fn set_cursor(&mut self, file_id: u32, pos: u32) {
    match self.log_file.seek(pos as i64, SeekSet) {
      Err(e) => fail!("I/O error while manipulating file stream: {}", e),
      _ => (),
    }
  }

  pub fn set_cursor_end(&mut self, file_id: u32) {
    match self.log_file.seek(0, SeekEnd) {
      Err(e) => fail!("I/O error while manipulating file stream: {}", e),
      _ => (),
    }
  }

  pub fn advance_cursor(&mut self, file_id: u32, offset: u32) {
    let pos = self.get_cursor(file_id);
    let new_pos = pos + offset;
    self.set_cursor(file_id, new_pos);
  }

  pub fn get_header(&mut self, file_id: u32, header_pos: u32) -> LogHeader {
    self.set_cursor(file_id, header_pos);
    let bytes = match self.log_file.read_exact(size_of::<LogHeader>()) {
      Ok(b) => b,
      Err(e) => fail!("I/O error while reading log header: {}", e),
    };
    let header: &LogHeader = coerce_from_bytes(bytes);
    *header
  }

  pub fn get_buffer(&mut self, file_id: u32, buffer_pos: u32, buffer_size: u32) -> ~[u8] {
    self.set_cursor(file_id, buffer_pos);
    let buffer = match self.log_file.read_exact(buffer_size as uint) {
      Ok(bytes) => bytes,
      Err(e) => fail!("I/O error while reading from log file: {}", e),
    };
    buffer
  }

  pub fn append(&mut self, txnid: TxnId, key: &[u8], value: &[u8]) -> u32 {
    let file_id = self.curr_file_id;
    self.set_cursor_end(file_id);
    let key_size = key.len() as u32;
    let value_size = value.len() as u32;
    let header = LogHeader{
      check: 0,
      key_size: key_size,
      value_size: value_size,
      txnid: txnid,
    };
    let header_bytes = coerce_to_bytes(&header);
    let _ = self.log_file.write(header_bytes);
    let _ = self.log_file.write(key);
    let value_pos = self.get_cursor(file_id);
    let _ = self.log_file.write(value);
    let _ = self.log_file.flush();
    let _ = self.log_file.fsync();
    value_pos
  }

  pub fn commit(&mut self, txnid: TxnId) {
    self.append(txnid, [], []);
  }

  pub fn delete(&mut self, txnid: TxnId, key: &[u8]) {
    self.append(txnid, key, []);
  }

  fn rollover(&self) {
  }
}
