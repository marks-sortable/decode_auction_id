extern crate data_encoding;
#[macro_use]
extern crate structopt;
extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt};
use data_encoding::BASE32;
use std::io::Cursor;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "m", long = "modulus", default_value = "1000")]
    modulus: u64,

    #[structopt(short = "t", long = "threshold")]
    threshold: Option<u64>,
    #[structopt(short="f", long="file", parse(from_os_str))]
    input: Option<PathBuf>,
    auction_ids: Vec<String>,
}

fn main() {
    let mut args = Args::from_args();
    let mut file_ids = vec![];
    if let Some(input_file) = args.input {
        let f = File::open(input_file).unwrap();
        let rdr = BufReader::new(f);
        let lines: Result<Vec<_>, _> = rdr.lines().collect();
        file_ids = lines.unwrap();
    }

    for auction_id in args.auction_ids.iter_mut().chain(file_ids.iter_mut()) {
        let mut output = vec![0; BASE32.decode_len(auction_id.len()).unwrap()];
        auction_id.make_ascii_uppercase();
        let len = BASE32
            .decode_mut(auction_id.as_bytes(), &mut output)
            .unwrap();
        let val =
            Cursor::new(&output[..len]).read_u64::<BigEndian>().unwrap() & 0xff_ffff_ffff_ffff;
        auction_id.make_ascii_lowercase();
        let marker = if let Some(ref threshold) = args.threshold {
            if val % args.modulus < *threshold {
                "***"
            } else {
                ""
            }
        } else {
            ""
        };
        println!(
            "{} {:>24} {:>24}  {}",
            auction_id,
            val,
            val % args.modulus,
            marker
        );
    }
}
