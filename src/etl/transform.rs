pub fn transform_data<F>(data: Vec<String>, transform_fn: F) -> Vec<String>
where
    F: Fn(Vec<String>) -> Vec<String>,
{
    println!("Applying transformation...");
    transform_fn(data)
}

