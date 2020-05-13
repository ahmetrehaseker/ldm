use std::io::Error;
use systemstat::{ByteSize, Duration, Filesystem, NetworkStats, Platform, System};

fn main() {
    let sys = System::new();
    match sys.networks() {
        Ok(stats) => {
            for (interface, network) in stats {
                println!("{:?}", network);
                match sys.network_stats(interface.as_str()) {
                    Ok(stat) => {
                        std::thread::sleep(Duration::from_secs(1));
                        println!(
                            "Rx: {}-{}, Tx: {}-{}",
                            stat.rx_bytes.to_string_as(false),
                            stat.rx_bytes.to_string_as(true),
                            stat.tx_bytes.to_string_as(false),
                            stat.tx_bytes.to_string_as(true)
                        );
                    }
                    Err(err) => println!("{:?}", err),
                }
            }
        }
        Err(err) => panic!(err),
    }
}
