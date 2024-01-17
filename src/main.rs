mod calendar;
mod csv_export;
mod csv_import;
mod models;
pub mod poker_planner;
mod prop_div;

use chrono::{NaiveDate, NaiveTime, ParseError};
use clap::Parser;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    tasks: PathBuf,

    #[arg(short, long, value_parser = parse_date)]
    start_date: NaiveDate,

    #[arg(short, long, value_parser = parse_date)]
    end_date: NaiveDate,

    #[arg(long, default_value = "10:00", value_parser = parse_time)]
    day_start: NaiveTime,

    #[arg(long, default_value = "18:00", value_parser = parse_time)]
    day_end: NaiveTime,
}

fn parse_date(arg: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(arg, "%d-%m-%Y")
}

fn parse_time(arg: &str) -> Result<NaiveTime, ParseError> {
    NaiveTime::parse_from_str(arg, "%H:%M")
}

fn main() {
    let cli = Cli::parse();
    run_pokerplanner(
        cli.tasks,
        cli.start_date,
        cli.end_date,
        cli.day_start,
        cli.day_end,
    );
}

fn run_pokerplanner(
    tasks_file: PathBuf,
    date_from: NaiveDate,
    date_to: NaiveDate,
    workday_start: NaiveTime,
    workday_end: NaiveTime,
) {
    let calendar = calendar::WorkCalendar::new();
    let tasks = csv_import::csv_import(tasks_file).unwrap();
    let time_records = poker_planner::calculate_hours_by_tasks(
        &tasks,
        &date_from,
        &date_to,
        &workday_start,
        &workday_end,
        &calendar,
    );
    csv_export::csv_export(io::stdout(), &time_records);
}

/*
 * TODO: settings: specify calendar
 * TODO: input: specify tasks with priority
 * TODO: input: specify workday
 * TODO: input: specify start/finish dates
 */
