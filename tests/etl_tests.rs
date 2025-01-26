#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_etl_pipeline() {
        let etl = ETLBuilder::new()
            .extract("test.csv")
            .transform(|data| {
                assert!(data.len() > 0);
            })
            .load("output.csv")
            .build();

        etl.run();
    }
}

