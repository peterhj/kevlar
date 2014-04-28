use internal::log::{Log, LogHeader};
use internal::table::{Table};
use internal::txn::{Txn, TxnId};

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
    server.init();
    server
  }

  fn init(&mut self) {
    enum TxnAction {
      Put(~[u8], u32, u32),
      Delete(~[u8], u32),
    }
    let mut prev_seen_txnid: TxnId = 0;
    let mut prev_good_txnid: TxnId = 0;
    let mut curr_txn: Option<Vec<TxnAction>> = Some(Vec::new());
    loop {
      let p = self.log.get_cursor(0);
      let header = self.log.get_header(0, p);
      // TODO truncate log on bad checksum.
      assert!(header.check == 0);
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
          let key_pos = p + size_of::<LogHeader>() as u32;
          let key = self.log.get_buffer(0, key_pos, header.key_size);
          let value_pos = key_pos + header.key_size;
          if header.value_size > 0 {
            curr_txn.get_mut_ref().push(Put(key, value_pos, header.value_size));
            self.log.set_cursor(0, value_pos + header.value_size);
          } else {
            curr_txn.get_mut_ref().push(Delete(key, value_pos));
          }
        } else {
          assert!(header.value_size == 0);
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
      if !self.log.eof(0) {
        break;
      }
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
    for kv in txn.kvs.move_iter() {
      let (key, value) = kv;
      let value_pos = self.log.append(txnid, key, value);
      let value_size = value.len() as u32;
      self.table.put(txnid, key, value_pos, value_size); // FIXME
    }
    self.log.commit(txnid);
  }
}
