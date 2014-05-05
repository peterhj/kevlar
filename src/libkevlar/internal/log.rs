use cast::{coerce_from_bytes, coerce_to_bytes};
use internal::server::{KvId};

use std::io::{Open, SeekEnd, SeekSet, ReadWrite};
use std::io::fs::{File, stat};
use std::mem::{size_of};

#[packed]
pub struct LogEntryHeader {
  check0: u32,
  check1: u32,
  flags: u32,
  key_size: u32,
  value_size: u32,
  kvid: KvId,
}

fn fletcher64_consume(c0: u32, c1: u32, bytes: &[u8]) -> (u32, u32) {
  let mut c0 = c0;
  let mut c1 = c1;
  let mut word: u32 = 0;
  let mut byte_offset = 0;
  for &byte in bytes.iter() {
    if byte_offset == 32 {
      c0 += word;
      c1 += c0;
      word = 0;
      byte_offset = 0;
    }
    word |= byte as u32 << byte_offset;
  }
  c0 += word;
  c1 += c0;
  (c0, c1)
}

impl LogEntryHeader {
  pub fn new(kvid: KvId, flags: u32, key: &[u8], value: &[u8]) -> LogEntryHeader {
    let key_size = key.len() as u32;
    let value_size = value.len() as u32;
    let header = LogEntryHeader{
      check0: 0,
      check1: 0,
      flags: flags,
      key_size: key_size,
      value_size: value_size,
      kvid: kvid,
    };
    /*let (c0, c1) = {
      let header_bytes = coerce_to_bytes(&header);
      let mut c0: u32 = 0;
      let mut c1: u32 = 0;
      match fletcher64_consume(c0, c1, header_bytes.slice_from(8)) {
        (c0_, c1_) => {
          c0 = c0_;
          c1 = c1_;
        }
      }
      match fletcher64_consume(c0, c1, key) {
        (c0_, c1_) => {
          c0 = c0_;
          c1 = c1_;
        }
      }
      match fletcher64_consume(c0, c1, value) {
        (c0_, c1_) => {
          c0 = c0_;
          c1 = c1_;
        }
      }
      (c0, c1)
    };
    header.check0 = c0;
    header.check1 = c1;*/
    header
  }

  pub fn verify(&self, key: &[u8], value: &[u8]) -> bool {
    //let mut c0: u32 = 0;
    //let mut c1: u32 = 0;
    //fletcher64_consume(c0, c1, );
    // TODO
    true
  }
}

pub static LOG_FLAG_WRITE: u32  = 0x01;
pub static LOG_FLAG_DELETE: u32 = 0x02;
pub static LOG_FLAG_PAD_8: u32  = 0x0100;
pub static LOG_FLAG_PAD_64: u32 = 0x0200;

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
    p >= size
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

  pub fn get_header(&mut self, file_id: u32, header_pos: u32) -> LogEntryHeader {
    self.set_cursor(file_id, header_pos);
    let bytes = match self.log_file.read_exact(size_of::<LogEntryHeader>()) {
      Ok(b) => b,
      Err(e) => fail!("I/O error while reading log header: {}", e),
    };
    let header: &LogEntryHeader = coerce_from_bytes(bytes);
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

  pub fn append(&mut self, kvid: KvId, flags: u32, key: &[u8], value: &[u8]) -> u32 {
    let file_id = self.curr_file_id;
    self.set_cursor_end(file_id);
    let header = LogEntryHeader::new(kvid, flags, key, value);
    let header_bytes = coerce_to_bytes(&header);
    let _ = self.log_file.write(header_bytes);
    let _ = self.log_file.write(key);
    let value_pos = self.get_cursor(file_id);
    let _ = self.log_file.write(value);
    let _ = self.log_file.flush();
    let _ = self.log_file.fsync();
    value_pos
  }

  fn rollover(&self) {
  }
}
