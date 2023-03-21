use std::error::Error;
use std::io;
use gnuplot::{Figure, Caption, Graph, Color, PointSymbol, AxesCommon};

fn main() -> Result<(), Box<dyn Error>> {
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut reader = csv::Reader::from_reader(io::stdin());

    for result in reader.records() {
        let r = result?;
        x.push(r[0].parse().unwrap());
        y.push(r[1].parse().unwrap());
    }

    println!("read the csv file.");
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Cat body measurements", &[])
        .set_legend(Graph(0.9), Graph(0.1), &[], &[])
        .set_x_label("height (cm)", &[])
        .set_y_label("length (cm)", &[])
        .points(x, y, &[Caption("Cat")]);

    //fg.set_terminal("pngcairo", "./cat.png");
    fg.save_to_png("./cat.png", 1280, 720);

    Ok(())
}
