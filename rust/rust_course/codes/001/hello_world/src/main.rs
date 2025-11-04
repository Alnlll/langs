fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    // for region in regions.iter() {
    for region in &regions {
        println!("{}", &region);
    }
}

fn main() {
    // println!("Hello, world!");
    // greet_world();

    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().is_empty() {
            continue; // Skip header line
        }

        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) {
            println!("Debug: {:?}: {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{} penguin is {} cm long", name, length);
        } else {
            eprintln!("Warning: Invalid length for {} on line {}: {}", name, i + 1, fields[1]);
        }
    }
}
