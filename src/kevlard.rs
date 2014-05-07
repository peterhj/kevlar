extern crate kevlar;
extern crate getopts;

use kevlar::db::{KevlarDb};

use getopts::{getopts, optflag, optopt, usage};
use std::fmt::{format};
use std::io::{Acceptor, Listener};
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::net::tcp::{TcpListener};
use std::io::{signal};
use std::num::{from_str_radix};
use std::{os};

struct KevlarDaemon {
  db: KevlarDb,
  prefix: Path,
  port: u16,
}

impl KevlarDaemon {
  pub fn new() -> KevlarDaemon {
    let args = os::args();
    let program = args[0].clone();
    let opts = ~[
      optflag("h", "help", "display this message"),
      optopt("", "prefix", "specify db path prefix", "<prefix>"),
      optopt("", "port", "specify listening port", "<port>"),
    ];
    let matches = match getopts(args.tail(), opts) {
      Ok(m) => m,
      Err(e) => fail!("{}", e),
    };
    if matches.opt_present("h") || matches.opt_present("help") {
      let brief = format_args!(format, "Usage: {} [options]", program);
      print!("{}", usage(brief, opts));
      fail!();
    }
    let prefix = match matches.opt_str("prefix") {
      Some(prefix_str) => Path::new(prefix_str),
      None => os::getcwd(),
    };
    let port: u16 = match matches.opt_str("port") {
      Some(port_str) => from_str_radix(port_str, 10).unwrap(),
      None => 6379,
    };
    let db = KevlarDb::new();
    KevlarDaemon{
      db: db,
      prefix: prefix,
      port: port,
    }
  }

  pub fn main(&mut self) {
    spawn(proc () {
      let mut listener = signal::Listener::new();
      match listener.register(signal::Interrupt) {
        Ok(_) => (),
        Err(e) => fail!("Failed to register signal handler: {}", e),
      }
      loop {
        match listener.rx.recv() {
          signal::Interrupt => println!("Received Ctrl-C"),
          _ => (),
        }
      }
    });
    self.runloop();
  }

  fn runloop(&mut self) {
    let addr = SocketAddr{ip: Ipv4Addr(127, 0, 0, 1), port: self.port};
    let listener = TcpListener::bind(addr);
    let mut acceptor = listener.listen();
    for stream in acceptor.incoming() {
      spawn(proc () {
      });
    }
    drop(acceptor);
  }
}

fn main() {
  KevlarDaemon::new().main();
}
