
use crate::models::TimeRecord;
use std::io;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

pub fn csv_export(time_records: &[TimeRecord]) {
    let mut writer = csv::Writer::from_writer(io::stdout());

    for time_record in time_records.iter() {
        writer.serialize(&time_record).unwrap();
    }

    writer.flush().unwrap();
}


#[cfg(test)]
mod tests {
}