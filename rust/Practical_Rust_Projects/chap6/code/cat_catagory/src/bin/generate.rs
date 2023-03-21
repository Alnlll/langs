extern crate rusty_machine;
extern crate rand;

use rusty_machine::linalg:: {Matrix, BaseMatrix};
use rand::thread_rng;
use rand::distributions::Distribution;
use rand_distr::Normal;
use structopt::StructOpt;
use std::io;
use std::vec::Vec;
use std::fs::read_to_string;
use serde::Deserialize;

#[derive(StructOpt)]
struct Options {
    #[structopt (short = "c", long = "config_file", parse(from_os_str))]
    /// config file path
    config_file : std::path::PathBuf,
}

#[derive(Deserialize)]
struct Config {
    centroids: [f64;6],
    nosie: f64,
    sample_per_center: usize,
}

fn generate_data(
    centroids: &Matrix<f64>, points_per_center: usize, nosie: f64)
    -> Matrix<f64>
{
    assert!(centroids.cols() > 0, "centroids should have 1 features as least.");
    assert!(centroids.rows() > 0, "centroids should have 1 samples as least.");
    assert!(nosie >= 0f64, "nosie must be non-neg.");

    let mut raw_cluster_data = Vec::with_capacity(
        centroids.rows() * centroids.cols() * points_per_center);
    let mut rng = thread_rng();
    let normal_rv = Normal::new(0f64, nosie).unwrap();

    for _ in 0..points_per_center {
        // generate points from each centroid
        for centroid in centroids.iter_rows() {
            // generate data around the centroid
            let mut point = Vec::with_capacity(centroids.cols());
            for feature in centroid.iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }
            raw_cluster_data.extend(point);
        }
    }

    return Matrix::new(
        centroids.rows() * points_per_center, centroids.cols(),
        raw_cluster_data);
}

/* const CENTRIODS: [f64;6] = [ */
/*     // height, Length */
/*     22.5, 40.5, // persain */
/*     38.0, 50.0, // british shorthair */
/*     25.5, 48.0,  // Ragdoll */
/* ]; */
/*  */
/* const NOSIE: f64 = 1.8; */
/* const SAMPELS_PER_CENTER: usize = 2000; */

fn main() -> Result<(), std::io::Error> {
    // Options
    let options = Options::from_args();
    let config_file_str = read_to_string(options.config_file)?;

    // parse config
    let config: Config = toml::from_str(&config_file_str)?;

    let centroids = Matrix::new(3, 2, config.centroids.to_vec());
    let samples = generate_data(&centroids, config.sample_per_center, config.nosie);

    // println!("{} {}", samples.rows(), samples.cols());

    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["height", "length"])?;
    for sample in samples.iter_rows() {
        writer.serialize(sample)?;
    }

    Ok(())
}

