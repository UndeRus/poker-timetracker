use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    calendar::{get_workdays, WorkCalendar},
    models::PlanningTask,
    prop_div::proportional_int_div,
};

#[derive(PartialEq, Debug)]
pub struct TimeRecord {
    pub task_id: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

pub fn calculate_hours_by_tasks(
    tasks: &[PlanningTask],
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    workday_start: &NaiveTime,
    workday_finish: &NaiveTime,
) -> Vec<TimeRecord> {
    let calendar = WorkCalendar::new();
    let workdays = get_workdays(start_date, end_date, &calendar);

    let workday_length = (*workday_finish - *workday_start).num_hours() as u64;
    let working_hours_full = workdays.len() as u64 * workday_length;

    let time_coeffs: Vec<u64> = tasks.iter().map(|t| t.priority).collect();
    let time_sizes = proportional_int_div(working_hours_full, &time_coeffs);

    let mut tracks = vec![];

    //TODO:
    // take first day => current day
    // take first task timespan => current task remain time
    // if current day + current task remain time < workday finish
    // add timespan to result
    // take next task
    // else if currentday + current task remain time = workday finish
    // take next task
    // take next day
    // else
    // cut off part of timespan
    // current task remain time -= part of timespan
    // take next day

    dbg!(time_sizes.clone());
    let mut task_iter = tasks.iter().zip(time_sizes);
    let mut workdays_iter = workdays.iter();

    let mut current_workday: Option<&NaiveDate> = workdays_iter.next();
    let mut current_task: Option<(&PlanningTask, u64)> = task_iter.next();
    let mut current_time_cursor = NaiveTime::from_hms_opt(10, 0, 0).unwrap();
    loop {
        if let Some(workday) = current_workday {
            if let Some((task, remain_hours)) = current_task {
                //println!("TASK {} {}", task.id.clone(), remain_hours);
                let next_time_estimate = NaiveDateTime::new(workday.clone(), current_time_cursor)
                    + Duration::hours(remain_hours as i64);
                let current_day_finish =
                    NaiveDateTime::new(workday.clone(), workday_finish.clone());
                //dbg!(current_day_finish);
                //dbg!(next_time_estimate);
                if next_time_estimate < current_day_finish {
                    tracks.push(TimeRecord {
                        task_id: task.id.clone(),
                        start: NaiveDateTime::new(workday.clone(), current_time_cursor.clone()),
                        end: NaiveDateTime::new(
                            workday.clone(),
                            current_time_cursor + Duration::hours(remain_hours as i64),
                        ),
                    });
                    current_time_cursor += Duration::hours(remain_hours as i64);
                    current_task = task_iter.next();
                    //println!("Underflow next task");
                    continue;
                } else if next_time_estimate == current_day_finish {
                    tracks.push(TimeRecord {
                        task_id: task.id.clone(),
                        start: NaiveDateTime::new(workday.clone(), current_time_cursor.clone()),
                        end: NaiveDateTime::new(
                            workday.clone(),
                            current_time_cursor + Duration::hours(remain_hours as i64),
                        ),
                    });
                    current_time_cursor = workday_start.clone();
                    current_task = task_iter.next();
                    current_workday = workdays_iter.next();
                    //println!("Overflow next day,next task");
                    continue;
                } else {
                    let hours_used =
                        (workday_finish.clone() - current_time_cursor).num_hours() as u64;
                    let hours_remain = remain_hours - hours_used;
                    //println!("Hours used {}, hours remain {}", hours_used, hours_remain);

                    tracks.push(TimeRecord {
                        task_id: task.id.clone(),
                        start: NaiveDateTime::new(workday.clone(), current_time_cursor.clone()),
                        end: NaiveDateTime::new(
                            workday.clone(),
                            current_time_cursor + Duration::hours(hours_used as i64),
                        ),
                    });

                    current_task = Some((task, hours_remain));
                    current_time_cursor = workday_start.clone(); // because of overflow
                    current_workday = workdays_iter.next();
                    continue;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    return tracks;
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::{
        models::PlanningTask,
        poker_planner::{calculate_hours_by_tasks, TimeRecord},
        pt,
    };

    //TODO: tests

    //1 day - 1 task, 3 days
    #[test]
    fn even_hours() {
        let tasks = vec![pt!("1", 1), pt!("2", 1), pt!("3", 1)];

        let time_records = calculate_hours_by_tasks(
            &tasks,
            &NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
            &NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
            &NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            &NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        );

        assert_eq!(time_records.len(), 3);
        assert_eq!(
            time_records,
            vec![
                TimeRecord {
                    task_id: "1".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "2".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 9).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 9).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "3".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
            ]
        );
    }

    //2,1,1 - 4 days
    #[test]
    fn uneven_full_hours() {
        let tasks = vec![pt!("1", 2), pt!("2", 1), pt!("3", 1)];

        let time_records = calculate_hours_by_tasks(
            &tasks,
            &NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
            &NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
            &NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            &NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        );

        assert_eq!(time_records.len(), 4);
        assert_eq!(
            time_records,
            vec![
                TimeRecord {
                    task_id: "1".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "1".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 9).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 9).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "2".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "3".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
            ]
        );
    }

    //1,1.5,0.5,1 - 4 days
    #[test]
    fn uneven_split_hours() {
        let tasks = vec![pt!("1", 2), pt!("2", 3), pt!("3", 1), pt!("4", 2)];

        let time_records = calculate_hours_by_tasks(
            &tasks,
            &NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
            &NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
            &NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            &NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        );

        assert_eq!(time_records.len(), 5);
        assert_eq!(
            time_records,
            vec![
                TimeRecord {
                    task_id: "1".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "2".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 9).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 9).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "2".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(14, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "3".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(14, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "4".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
            ]
        );
    }

    //1,1.7,1.3,1-5 days
    #[test]
    fn uneven_more_fractured_hours() {
        let tasks = vec![pt!("1", 10), pt!("2", 17), pt!("3", 13), pt!("4", 10)];

        let time_records = calculate_hours_by_tasks(
            &tasks,
            &NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
            &NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
            &NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            &NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        );


        assert_eq!(time_records.len(), 5);
        assert_eq!(
            time_records,
            vec![
                TimeRecord {
                    task_id: "1".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "2".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 9).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 9).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "2".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(14, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "3".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(14, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
                TimeRecord {
                    task_id: "4".to_string(),
                    start: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
                        NaiveTime::from_hms_opt(10, 0, 0).unwrap()
                    ),
                    end: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
                        NaiveTime::from_hms_opt(18, 0, 0).unwrap()
                    )
                },
            ]
        );
    }

    #[test]
    fn check_hours() {
        let tasks = vec![
            PlanningTask {
                priority: 1,
                id: "1".to_string(),
            },
            PlanningTask {
                priority: 5,
                id: "3".to_string(),
            },
            PlanningTask {
                priority: 10,
                id: "2".to_string(),
            },
        ];
        let result = calculate_hours_by_tasks(
            &tasks,
            &NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            &NaiveDate::from_ymd_opt(2023, 1, 31).unwrap(),
            &NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            &NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        );

        dbg!(result);
    }
}

#[macro_export]
macro_rules! pt {
    ($id:expr, $priority:expr) => {
        PlanningTask {
            // id: id.to_string(),
            // priority: priority
            id: $id.to_string(),
            priority: $priority,
        }
    };
}
