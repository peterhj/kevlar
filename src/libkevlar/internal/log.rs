use cast::{coerce_from_bytes, coerce_to_bytes};
use internal::txn::{TxnId};

use std::io::{Open, Read, SeekCur, SeekEnd, SeekSet, ReadWrite};
use std::io::fs::{File, stat};
use std::mem::{size_of};
//use std::str::{from_utf8_lossy};

#[packed]
pub struct LogHeader {
  check: u64,
  key_size: u32,
  value_size: u32,
  txnid: TxnId,
}

pub struct Log {
  log_path: Path,
  log_file: File,
}

impl Log {
  pub fn new() -> Log {
    let log_path = Path::new("kevlar.log");
    let mut log_file = match File::open_mode(&log_path, Open, ReadWrite) {
      Ok(file) => file,
      Err(e) => fail!("I/O error while opening log file for appending: {}", e),
    };
    log_file.seek(0, SeekSet);
    Log{
      log_path: log_path,
      log_file: log_file,
    }
  }

  pub fn advance(&mut self, file_id: u32, key_size: u32, value_size: u32) -> bool {
    let size = match stat(&self.log_path) {
      Ok(st) => st.size as u32,
      Err(e) => fail!("I/O error while querying log file stat: {}", e),
    };
    let p = self.get_pos(0);
    /*let np = (p as i64) + (key_size as i64) + (value_size as i64);
    self.log_file.seek(np, SeekSet);*/
    let p = self.get_pos(file_id);
    p < size
  }

  pub fn get_pos(&self, file_id: u32) -> u32 {
    match self.log_file.tell() {
      Ok(p) => p as u32,
      Err(e) => fail!("I/O error while querying log file pos: {}", e),
    }
  }

  pub fn get_header(&mut self, file_id: u32, header_pos: u32) -> LogHeader {
    //println!("getting header at {}", header_pos);
    self.log_file.seek(header_pos as i64, SeekSet);
    let bytes = match self.log_file.read_exact(size_of::<LogHeader>()) {
      Ok(b) => b,
      Err(e) => fail!("I/O error while reading log header: {}", e),
    };
    let header: &LogHeader = coerce_from_bytes(bytes);
    //println!("got header: len {}", bytes.len());
    //println!("  chk: {}", header.check);
    //println!("  key: {}", header.key_size);
    //println!("  val: {}", header.value_size);
    //println!("  txn: {}", header.txnid);
    *header
  }

  pub fn get_buffer(&mut self, file_id: u32, buffer_pos: u32, buffer_size: u32) -> ~[u8] {
    //println!("getting buffer at {} w/ len {}", buffer_pos, buffer_size);
    self.log_file.seek(buffer_pos as i64, SeekSet);
    let buffer = match self.log_file.read_exact(buffer_size as uint) {
      Ok(bytes) => bytes,
      Err(e) => fail!("I/O error while reading from log file: {}", e),
    };
    //println!("got buffer: len {}", buffer.len());
    //println!("  val: {}", from_utf8_lossy(buffer).as_slice());
    buffer
  }

  pub fn append(&mut self, txnid: TxnId, key: &[u8], value: &[u8]) -> u32 {
    self.log_file.seek(0, SeekEnd);
    let key_size = key.len() as u32;
    let value_size = value.len() as u32;
    let header = LogHeader{
      check: 0,
      key_size: key_size,
      value_size: value_size,
      txnid: txnid,
    };
    let header_bytes = coerce_to_bytes(&header);
    self.log_file.write(header_bytes);
    self.log_file.write(key);
    /*let value_pos = match self.log_file.tell() {
      Ok(n) => n,
      Err(e) => fail!("I/O error while writing to log file: {}", e),
    } as u32;*/
    let value_pos = self.get_pos(0);
    self.log_file.write(value);
    self.log_file.flush();
    self.log_file.fsync();
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
