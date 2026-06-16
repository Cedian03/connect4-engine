use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn load_test_suite<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<TestCase>, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(path)?);

    reader
        .lines()
        .map(|x| {
            x.map(|x| {
                let mut parts = x.split_ascii_whitespace();
                let opening = parts.next().unwrap();
                let expected = parts.next().unwrap().parse().unwrap();

                TestCase {
                    opening_sequence: opening.to_owned(),
                    expected_evaluation: expected,
                }
            })
            .map_err(Into::into)
        })
        .collect()
}

#[derive(Debug)]
pub struct TestCase {
    pub opening_sequence: String,
    pub expected_evaluation: i32,
}
