extern  crate structopt;
use structopt::StructOpt;

extern crate flate2;
use flate2::Compression;
// use flate2::write::ZlibEncoder;
// use flate2::write::GzEncoder;
use flate2::GzBuilder;

use std::io::Write;
use std::io::{ BufReader, BufWriter, Read };
use std::fs::File;

#[derive(StructOpt, Debug)]
#[structopt(name = "compress", about = "Compress a file using gzip")]
struct Options {
    #[structopt(short = "s", long = "src", parse(from_os_str))]
    /// The file path to compress
    src_file : std::path::PathBuf,

    #[structopt(short = "t", long = "tgt", parse(from_os_str))]
    /// The file path to compressed
    tgt_file : std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let options = Options::from_args();

    // let src_path = options.src_file;
    // let tgt_path = options.tgt_file;

    // let src_file = File::open(&src_path.unwrap())?;
    // let tgt_file = File::create(&tgt_path.unwrap())?;

    // let mut reader = BufReader::new(&src_file);
    // let writer = BufWriter::new(&tgt_file);

    // let mut encoder = ZlibEncoder::new(writer, Compression::default());
    // std::io::copy(&mut reader, &mut encoder)?;
    /* let mut gz = GzEncoder::new(writer, Compression::default()); */
    /* std::io::copy(&mut reader, &mut gz)?; */
    // let compressed = gz.finish()?;

    // println!("input len : {:?}", src_file.metadata().unwrap().len());
    // println!("output len: {:?}", compressed.get_ref().metadata().unwrap().len());
    // let mut input_buf = BufReader::new(&File::open(&options.src_file));
    // let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    // e.write_all(b"foo");
    // e.write_all(b"bar");
    // let compressed_bytes = e.finish();

    // println!("{:?}", compressed_bytes.unwrap());
    //
    
    // read input file
    let mut input_file = File::open(&options.src_file)?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    // compress the file
    let mut encoder = GzBuilder::new()
                        .filename(options.src_file.file_name().unwrap().to_str().unwrap())
                        .write(File::create(&options.tgt_file)?, Compression::default());
    encoder.write_all(&input_data)?;
    encoder.finish()?;

    // print len info
    let input_len = input_data.len();
    let output_len = std::fs::metadata(options.tgt_file)?.len();

    println!("input_len: {:?}", input_len);
    println!("output_len: {:?}", output_len);

    return Ok(());
}
