extern crate rusty_machine;
use rusty_machine::learning::k_means::KMeansClassifier;
use rusty_machine::learning::UnSupModel;
use rusty_machine::prelude::{Matrix, BaseMatrix};


use std::error::Error;
use std::io;

const CLUSTER_COUNT: usize = 3;

fn read_data_from_stdin() -> Result<Matrix<f64>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut data: Vec<f64> = vec!();

    for result in reader.records() {
        let r = result?;
        data.push(r[0].parse().unwrap());
        data.push(r[1].parse().unwrap());
    }

    return Ok(Matrix::new(&data.len() / 2, 2, data));
}

fn export_result_to_stdout(samples: Matrix<f64>, classes: Vec<usize>)
    -> Result<(), Box<dyn Error>>
{
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["height", "length", "class"])?;
    for sample in samples.iter_rows().zip(classes) {
        writer.serialize(sample)?;
    }

    return Ok(());
}

fn main() {
    let samples = read_data_from_stdin().unwrap();

    let mut model = KMeansClassifier::new(CLUSTER_COUNT);
    model.train(&samples).unwrap();

    let classes = model.predict(&samples).unwrap();

    export_result_to_stdout(samples, classes.into_vec()).unwrap();
}

