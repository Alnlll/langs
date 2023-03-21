extern crate serde;
use serde::Serialize;

extern crate rusty_machine as rm;
use rm::linalg::{ Matrix, BaseMatrix};

use rand::thread_rng;
use rand_distr::{Normal, Distribution};

use structopt::StructOpt;
use std::error::Error;

#[derive(StructOpt)]
struct Options {
    /// Output path of generated data, xxx.csv
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: std::path::PathBuf,
}

#[derive(Debug, Serialize)]
struct Sample<> {
    height : f64,
    length : f64,
    class : usize
}

fn generate_data(
    centroids: &Matrix<f64>, point_per_center: usize, noise: f64)
    -> Vec<Sample>
{
    let mut samples = Vec::with_capacity(point_per_center);
    let mut rng = thread_rng();
    let normal_rv = Normal::new(0f64, noise).unwrap();

    for _ in 0..point_per_center {
        for (centroid_id, centroid) in centroids.iter_rows().enumerate() {
            let mut point = Vec::with_capacity(centroids.cols());
            for f in centroid.iter() {
                point.push(f + normal_rv.sample(&mut rng));
            }
            samples.push(Sample { height: point[0], length: point[1], class: centroid_id, });
        }
    }

    //for 

    return samples;
}

const CENTRIODS: [f64;6] = [
    // height, Length
    22.5, 40.5, // persain
    38.0, 50.0, // british shorthair
    25.5, 48.0,  // Ragdoll
];

const NOSIE: f64 = 1.8;
const SAMPELS_PER_CENTER: usize = 2000;

fn main() -> Result<(), std::io::Error>{
    // Options
    let options = Options::from_args();
    
    let output_file = options.output;

    // generate data
    let centroids = Matrix::new(3, 2, CENTRIODS);
    let samples = generate_data(&centroids, SAMPELS_PER_CENTER, NOSIE);

    let mut writer = csv::Writer::from_path(std::io::stdout());
    for sample in samples.iter() {
        // println!("{:?}", sample);
        writer.serialize(sample)?;
    }

/*     let centroids = Matrix::new(3, 2, config.centroids.to_vec()); */
    /* let samples = generate_data(&centroids, config.sample_per_center, config.nosie); */
    /*  */
    /* // println!("{} {}", samples.rows(), samples.cols()); */
    /*  */
    /* let mut writer = csv::Writer::from_writer(io::stdout()); */
    /* writer.write_record(&["height", "length"])?; */
    /* for sample in samples.iter_rows() { */
    /*     writer.serialize(sample)?; */
    /* } */
/*  */
    Ok(())
}


