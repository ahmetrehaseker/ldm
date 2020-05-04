use simplelog::*;

#[macro_use]
extern crate log;
extern crate simplelog;

pub fn init_logger() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, simplelog::Config::default(), TerminalMode::Mixed).unwrap(),
        ]
    ).unwrap();
}

fn main() {
    init_logger();
    info!("Hello World!!")
}
