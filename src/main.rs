extern crate data_encoding;
#[macro_use]
extern crate structopt;
extern crate byteorder;

use data_encoding::BASE32;
use structopt::StructOpt;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "m", long = "modulus", default_value = "1000")]
    modulus: u64,

    auction_ids: Vec<String>,
}

fn main() {
    let args = Args::from_args();

    for mut auction_id in args.auction_ids {
        let mut output = vec![0; BASE32.decode_len(auction_id.len()).unwrap()];
        auction_id.make_ascii_uppercase();
        let len = BASE32.decode_mut(auction_id.as_bytes(), &mut output).unwrap();
        let val = Cursor::new(&output[..len]).read_u64::<BigEndian>().unwrap() & 0x7fff_ffff_ffff_ffff;
        auction_id.make_ascii_lowercase();
        println!("{} {:>24} {:>24}", auction_id, val, val % args.modulus);
    }
}
