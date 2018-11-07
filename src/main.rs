extern crate snap;
#[macro_use]
extern crate clap;

use std::io::{BufReader,BufWriter,Read,Write};
use clap::Arg;

#[derive(Debug)]
enum Error {
    Arg(clap::Error),
    Read(std::io::Error),
    Write(std::io::Error),
    Flush(std::io::Error),
}

fn main() -> Result<(),Error> {
    let matches = app_from_crate!()
        .arg(Arg::with_name("mode")
             .display_order(1)
             .short("m")
             .long("mode")
             .value_name("MODE")
             .help("Run mode")
             .possible_values(&["compress","decompress"])
             .takes_value(true))
        .get_matches();


    match &value_t!(matches, "mode", String).map_err(Error::Arg)? as &str{
        "compress" => {
            let mut bin = BufReader::new(std::io::stdin());
            let mut bout = snap::Writer::new(BufWriter::new(std::io::stdout()));
            let mut buf = [0; 1024]; 
            loop {
                let n = bin.read(&mut buf[..]).map_err(Error::Read)?;
                if n==0 { break; }
                bout.write(&buf[..n]).map_err(Error::Write)?;
            }
            bout.flush().map_err(Error::Flush)?;
        },
        "decompress" => {
            let mut bin = snap::Reader::new(BufReader::new(std::io::stdin()));
            let mut bout = BufWriter::new(std::io::stdout());
            let mut buf = [0; 1024]; 
            loop {
                let n = bin.read(&mut buf[..]).map_err(Error::Read)?;
                if n==0 { break; }
                bout.write(&buf[..n]).map_err(Error::Write)?;
            }
            bout.flush().map_err(Error::Flush)?;
        },
        _ => unreachable!(),
    }
    Ok(())
}
