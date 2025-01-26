mod etl;
mod quality;
mod integration;

use etl::ETLBuilder;
use quality::DataQuality;

fn main() {
    let etl_pipeline = ETLBuilder::new()
        .extract("data/input.csv")
        .transform(|data| {
            println!("Transforming data: {:?}", data);
        })
        .load("data/output.csv")
        .build();

    etl_pipeline.run();
    println!("ETL Pipeline executed successfully!");

    let quality_report = DataQuality::assess("data/output.csv");
    println!("Data Quality Report: {:?}", quality_report);
}

