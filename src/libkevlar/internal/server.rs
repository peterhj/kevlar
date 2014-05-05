use internal::log::{
  LOG_FLAG_DELETE, LOG_FLAG_WRITE,
  Log, LogEntryHeader,
};
use internal::table::{Table};

use std::cmp::{max};
use std::mem::{size_of};

pub type KvId = u64;

pub struct Server {
  log: Log,
  table: Table,
  kvid_counter: KvId,
}

impl Server {
  pub fn new() -> Server {
    let mut server = Server{
      log: Log::new(),
      table: Table::new(),
      kvid_counter: 0,
    };
    server.replay_log();
    server
  }

  fn replay_log(&mut self) {
    let mut prev_seen_kvid: KvId = 0;
    let mut prev_good_kvid: KvId = 0;
    loop {
      if self.log.eof(0) {
        break;
      }
      let p = self.log.get_cursor(0);
      let header = self.log.get_header(0, p);
      // TODO truncate log on bad checksum.
      //if header.check != ... {
      //}
      if header.kvid < prev_seen_kvid {
        // Out of order kvid, skip.
        self.log.advance_cursor(0, header.key_size);
        self.log.advance_cursor(0, header.value_size);
      } else {
        if header.kvid > prev_seen_kvid && prev_good_kvid != prev_seen_kvid {
          // Uncommitted txn, rollback.
        }
        if header.key_size > 0 {
          let key_pos = p + size_of::<LogEntryHeader>() as u32;
          let key = self.log.get_buffer(0, key_pos, header.key_size);
          let value_pos = key_pos + header.key_size;
          if header.value_size > 0 {
            self.table.put(header.kvid, key, value_pos, header.value_size);
            self.log.set_cursor(0, value_pos + header.value_size);
          } else {
            self.table.delete(header.kvid, key, value_pos);
          }
          prev_good_kvid = header.kvid;
        } else {
          // TODO
        }
        prev_seen_kvid = header.kvid;
      }
      self.kvid_counter = max(self.kvid_counter, header.kvid);
    }
  }

  pub fn get(&mut self, key: ~[u8]) -> ~[u8] {
    let entry = self.table.get(&key);
    self.log.get_buffer(entry.file_id, entry.value_pos, entry.value_size)
  }

  fn allocate_kvid(&mut self) -> KvId {
    self.kvid_counter += 1;
    let kvid = self.kvid_counter;
    kvid
  }

  pub fn put(&mut self, key: ~[u8], value: ~[u8]) {
    let kvid = self.allocate_kvid();
    let value_pos = self.log.append(kvid, LOG_FLAG_WRITE, key, value);
    let value_size = value.len() as u32;
    self.table.put(kvid, key, value_pos, value_size);
  }

  pub fn delete(&mut self, key: ~[u8]) {
    let kvid = self.allocate_kvid();
    let value_pos = self.log.append(kvid, LOG_FLAG_DELETE, key, []);
    self.table.delete(kvid, key, value_pos);
  }
}
