use ai_data_engineering_toolkit::{ETLBuilder, DataQuality};

fn main() {
    let etl = ETLBuilder::new()
        .extract("examples/input.csv")
        .transform(|data| {
            println!("Custom transformation logic here!");
        })
        .load("examples/output.csv")
        .build();

    etl.run();

    let quality = DataQuality::assess("examples/output.csv");
    println!("{}", quality);
}

