mod calendar;
mod models;
pub mod poker_planner;
mod prop_div;
mod csv_export;


use chrono::{NaiveDate, NaiveTime};
use models::PlanningTask;

fn main() {
    run_pokerplanner();
}


fn run_pokerplanner() {
    let calendar = calendar::WorkCalendar::new();
    let date_from = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let date_to = NaiveDate::from_ymd_opt(2023, 1, 30).unwrap();
    let workday_start = NaiveTime::from_hms_opt(10, 0, 0).unwrap();
    let workday_end = NaiveTime::from_hms_opt(18, 0, 0).unwrap();
    let tasks = vec![
        PlanningTask::new("MUH-111", 5),
    ];
    let time_records =poker_planner::calculate_hours_by_tasks(
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
