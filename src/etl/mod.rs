pub mod extract;
pub mod transform;
pub mod load;

pub struct ETLBuilder;

impl ETLBuilder {
    pub fn new() -> Self {
        ETLBuilder
    }

    pub fn extract(self, source: &str) -> Self {
        println!("Extracting data from: {}", source);
        self
    }

    pub fn transform<F>(self, transform_fn: F) -> Self
    where
        F: FnOnce(Vec<String>),
    {
        println!("Transforming data...");
        transform_fn(vec!["sample data".to_string()]);
        self
    }

    pub fn load(self, destination: &str) -> Self {
        println!("Loading data to: {}", destination);
        self
    }

    pub fn build(self) -> Self {
        println!("Building ETL pipeline...");
        self
    }

    pub fn run(self) {
        println!("Running ETL pipeline...");
    }
}

