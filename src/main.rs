mod calendar;
mod csv_export;
mod csv_import;
mod models;
pub mod poker_planner;
mod prop_div;

use std::path::PathBuf;
use chrono::{NaiveDate, NaiveTime, ParseError};
use clap::Parser;

use models::PlanningTask;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    tasks: PathBuf,

    #[arg(short, long, value_parser = parse_date)]
    start_date: NaiveDate,

    #[arg(short, long, value_parser = parse_date)]
    end_date: NaiveDate,

    #[arg(long, default_value_t = 10)]
    day_start: u64,

    #[arg(long, default_value_t = 18)]
    day_end: u64,

}

fn parse_date(arg: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(arg, "%d-%m-%Y")
}

fn main() {
    let cli = Cli::parse();
    run_pokerplanner(cli.tasks, cli.start_date, cli.end_date);
}

fn run_pokerplanner(tasks_file: PathBuf, date_from: NaiveDate, date_to: NaiveDate) {
    let calendar = calendar::WorkCalendar::new();
    let workday_start = NaiveTime::from_hms_opt(10, 0, 0).unwrap();
    let workday_end = NaiveTime::from_hms_opt(18, 0, 0).unwrap();
    let tasks = csv_import::csv_import(tasks_file).unwrap();
    let time_records = poker_planner::calculate_hours_by_tasks(
        &tasks,
        &date_from,
        &date_to,
        &workday_start,
        &workday_end,
        &calendar,
    );
    csv_export::csv_export(&time_records);
}

/*
 * TODO: settings: specify calendar
 * TODO: input: specify tasks with priority
 * TODO: input: specify workday
 * TODO: input: specify start/finish dates
 */
