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
    #[deriving(Clone)]
    enum TxnAction {
      Put(~[u8], u32, u32),
      Delete(~[u8], u32),
    }
    let mut prev_seen_txnid: TxnId = 0;
    let mut prev_good_txnid: TxnId = 0;
    let mut txn_actions: Option<Vec<TxnAction>> = Some(Vec::new());
    loop {
      let p = self.log.get_pos(0);
      let header = self.log.get_header(0, p);
      if header.txnid < prev_seen_txnid {
        // TODO out of order txnid.
      }
      if header.txnid > prev_seen_txnid && prev_good_txnid != prev_seen_txnid {
        // Uncommitted txn, rollback.
        txn_actions.get_mut_ref().clear();
      }
      if header.key_size > 0 {
        let key_pos = p + size_of::<LogHeader>() as u32;
        let key = self.log.get_buffer(0, key_pos, header.key_size);
        let value_pos = key_pos + header.key_size;
        if header.value_size > 0 {
          let value = self.log.get_buffer(0, value_pos, header.value_size);
          txn_actions.get_mut_ref().push(Put(key, value_pos, header.value_size));
        } else {
          txn_actions.get_mut_ref().push(Delete(key, value_pos));
        }
      } else {
        assert!(header.value_size == 0);
        if header.txnid <= prev_good_txnid {
          // TODO duplicate commit.
        } else {
          for x in txn_actions.take_unwrap().move_iter() {
            match x {
              Put(key, value_pos, value_size) => {
                self.table.put(header.txnid, key, value_pos, value_size);
              },
              Delete(key, value_pos) => {
                self.table.delete(header.txnid, &key, value_pos);
              },
            }
          }
          txn_actions = Some(Vec::new());
          prev_good_txnid = header.txnid;
        }
      }
      self.txnid_counter = max(self.txnid_counter, header.txnid);
      prev_seen_txnid = header.txnid;
      if !self.log.advance(0, header.key_size, header.value_size) {
        break;
      }
    }
  }

  pub fn begin_txn(&mut self) -> Txn {
    self.txnid_counter += 1;
    let txnid = self.txnid_counter;
    Txn::new(txnid)
  }

  pub fn get(&mut self, key: ~[u8]) -> ~[u8] {
    let entry = self.table.get(&key);
    self.log.get_buffer(entry.file_id, entry.value_pos, entry.value_size)
  }

  pub fn commit(&mut self, txn: Txn) {
    let txnid = txn.id;
    for kv in txn.kvs.move_iter() {
      let (key, value) = kv;
      let value_pos = self.log.append(txnid, key, value);
      let value_size = value.len() as u32;
      self.table.put(txnid, key, value_pos, value_size); // FIXME
    }
    self.log.commit(txnid);
  }
}
