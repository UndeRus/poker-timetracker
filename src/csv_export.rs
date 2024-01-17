use crate::models::TimeRecord;
use std::io;

pub fn csv_export<W: io::Write + Sized>(output: W, time_records: &[TimeRecord]) {
    let mut writer = csv::Writer::from_writer(output);

    for time_record in time_records.iter() {
        writer.serialize(&time_record).unwrap();
    }

    writer.flush().unwrap();
}
