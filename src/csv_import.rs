use crate::models::PlanningTask;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn csv_import(filename: PathBuf) -> Result<Vec<PlanningTask>, String> {
    let file = File::open(filename).expect("Failed to open csv");
    let br = BufReader::new(file);

    let mut rdr = csv::Reader::from_reader(br);
    let tasks_result: Result<Vec<PlanningTask>, _> = rdr.deserialize().collect();
    match tasks_result {
        Ok(tasks) => Ok(tasks),
        Err(error) => {
            if let csv::ErrorKind::Deserialize { pos, err: _err } = error.kind() {
                if let Some(pos) = pos {
                    let error = format!("Error in line {}, check your input file", pos.line());
                    Err(error)
                } else {
                    Err("Failed to Deserialize tasks, unknown error".to_string())
                }
            } else {
                Err("Failed to import tasks, unknown error".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn import_csv() {
        let mut path =PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/demo-tasks.csv");
        let result = csv_import(path);
        assert!(result.is_ok());
    }
}
