use std::{net::IpAddr, thread::{self}, time::{Duration}};
use chrono::{Local};
use winping::{Buffer, Pinger};
use log::{Level, LevelFilter, Metadata, Record, info, warn};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
     metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let local = Local::now();
            println!("{} {} - {}",local.format("%d/%m/%Y %T"), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
fn main() {
    log::set_logger(&CONSOLE_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
    let args: Vec<String> =  std::env::args().collect();

    let dst = args.get(1)
        .unwrap_or(&String::from("127.0.0.1"))
        .parse::<IpAddr>()
        .expect("Could not parse IP Address");
    let threshold = args
        .get(2)
        .unwrap_or(&String::from("50"))
        .parse()
        .expect("Could not parse IP Address");

    info!("starting ping to {}", dst);
    info!("threshold: {}ms", threshold);
    let pinger = Pinger::new().unwrap();
    let mut buffer = Buffer::new();
     
    loop {
        match pinger.send(dst, &mut buffer) {
            Ok(rtt) if rtt > threshold => {
                info!("Response time {} ms.", rtt)
            },
            Ok(_) => {}
            Err(err) => warn!("{}.", err),
        }
        thread::sleep(Duration::from_millis(30));
    }
}