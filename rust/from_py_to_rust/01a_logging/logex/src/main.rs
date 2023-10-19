use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{ Appender, Config, Logger, Root };

mod foo {
  pub fn run() {
    log::info!("{:?}", "And every where that Mary went");
    log::warn!("{:#?}", "The lamb was sure to go");
  }
}

fn main() {
  // let stdout = ConsoleAppender::builder()
  //   .encoder(Box::new(
  //     log4rs::encode::pattern::PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {P}:{i} {h({l})} - [{M}] {m}{n}")))
  //   .build();

  // let requests = FileAppender::builder()
  //   .encoder(
  //     Box::new(
  //       log4rs::encode::pattern::PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {P}:{i} {h({l})} - [{M}] {m}{n}")))
  //   .build("logs/requests.log")
  //   .unwrap();

  // let config = Config::builder()
  //   .appender(Appender::builder().build("stdout", Box::new(stdout)))
  //   .appender(Appender::builder().build("requests", Box::new(requests)))
  //   .build(
  //     Root::builder()
  //       .appender("stdout")
  //       .appender("requests")
  //       .build(LevelFilter::Debug))
  //   .unwrap();

  // let _handle = log4rs::init_config(config).unwrap();
  let _handle = log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

  log::debug!("Mary has a little lamb");
  log::error!("{}", "Its fleece was white as snow");
  log::info!("{:?}", "And every where that Mary went");
  log::warn!("{:#?}", "The lamb was sure to go");
  foo::run();
}
