use internal::log::{
  LOG_FLAG_COMMIT, LOG_FLAG_DELETE, LOG_FLAG_WRITE,
  Log, LogEntryHeader,
};
use internal::table::{Table};
use internal::txn::{Delete, Put, Txn, TxnId};

use std::cmp::{max};
use std::mem::{size_of};

pub struct Server {
  log: Log,
  table: Table,
  txnid_counter: TxnId,
}

impl Server {
  pub fn new() -> Server {
    let mut server = Server{
      log: Log::new(),
      table: Table::new(),
      txnid_counter: 0,
    };
    server.replay_log();
    server
  }

  fn replay_log(&mut self) {
    enum LogTxnOp {
      Put(~[u8], u32, u32),
      Delete(~[u8], u32),
    }
    let mut prev_seen_txnid: TxnId = 0;
    let mut prev_good_txnid: TxnId = 0;
    let mut curr_txn: Option<Vec<LogTxnOp>> = Some(Vec::new());
    loop {
      if self.log.eof(0) {
        break;
      }
      let p = self.log.get_cursor(0);
      let header = self.log.get_header(0, p);
      // TODO truncate log on bad checksum.
      //if header.check != ... {
      //}
      if header.txnid < prev_seen_txnid {
        // Out of order txnid, skip.
        self.log.advance_cursor(0, header.key_size);
        self.log.advance_cursor(0, header.value_size);
      } else {
        if header.txnid > prev_seen_txnid && prev_good_txnid != prev_seen_txnid {
          // Uncommitted txn, rollback.
          curr_txn.get_mut_ref().clear();
        }
        if header.key_size > 0 {
          let key_pos = p + size_of::<LogEntryHeader>() as u32;
          let key = self.log.get_buffer(0, key_pos, header.key_size);
          let value_pos = key_pos + header.key_size;
          if header.value_size > 0 {
            curr_txn.get_mut_ref().push(Put(key, value_pos, header.value_size));
            self.log.set_cursor(0, value_pos + header.value_size);
          } else {
            curr_txn.get_mut_ref().push(Delete(key, value_pos));
          }
        } else {
          // TODO rather than accept an empty value, instead encode the number
          // of ops in the txn as a u32.
          //assert!(header.value_size == 0);
          assert!(header.txnid == prev_seen_txnid);
          for x in curr_txn.take_unwrap().move_iter() {
            match x {
              Put(key, value_pos, value_size) => {
                self.table.put(header.txnid, key, value_pos, value_size);
              },
              Delete(key, value_pos) => {
                self.table.delete(header.txnid, key, value_pos);
              },
            }
          }
          curr_txn = Some(Vec::new());
          prev_good_txnid = header.txnid;
        }
        prev_seen_txnid = header.txnid;
      }
      self.txnid_counter = max(self.txnid_counter, header.txnid);
    }
  }

  pub fn begin_txn(&mut self) -> Txn {
    Txn::new()
  }

  pub fn get(&mut self, key: ~[u8]) -> ~[u8] {
    let entry = self.table.get(&key);
    self.log.get_buffer(entry.file_id, entry.value_pos, entry.value_size)
  }

  fn allocate_txnid(&mut self) -> TxnId {
    self.txnid_counter += 1;
    let txnid = self.txnid_counter;
    txnid
  }

  pub fn commit(&mut self, txn: Txn) {
    let txnid = self.allocate_txnid();
    for op in txn.ops.move_iter() {
      match op {
        Put(key, value) => {
          let value_pos = self.log.append(txnid, LOG_FLAG_WRITE, key, value);
          let value_size = value.len() as u32;
          self.table.put(txnid, key, value_pos, value_size);
        },
        Delete(key) => {
          let value_pos = self.log.append(txnid, LOG_FLAG_DELETE, key, []);
          self.table.delete(txnid, key, value_pos);
        },
      }
    }
    self.log.append(txnid, LOG_FLAG_COMMIT, [], []);
  }
}
