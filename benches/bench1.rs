use crate::poker_planner::{calculate_hours_by_tasks, PlanningTask};
use chrono::{NaiveDate, NaiveTime};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| {
            let tasks = vec![
                PlanningTask {
                    priority: 1,
                    id: "1".to_string(),
                },
                PlanningTask {
                    priority: 1,
                    id: "2".to_string(),
                },
                PlanningTask {
                    priority: 1,
                    id: "3".to_string(),
                },
            ];
            let time_records = calculate_hours_by_tasks(
                &tasks,
                &NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                &NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                &NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
                &NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
