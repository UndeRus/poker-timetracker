use chrono::{Datelike, Duration, NaiveDate, Weekday};

pub struct WorkCalendar {
    holidays: Vec<NaiveDate>,
    weekend_days: Vec<Weekday>,
    special_workdays: Vec<NaiveDate>,
    special_holidays: Vec<NaiveDate>,
}

impl WorkCalendar {
    pub fn new() -> Self {
        WorkCalendar {
            holidays: vec![],
            weekend_days: vec![Weekday::Sat, Weekday::Sun],
            special_workdays: vec![],
            special_holidays: vec![],
        }
    }

    pub fn add_holiday(&mut self, holiday: &NaiveDate) {
        self.holidays.push(holiday.clone());
    }

    pub fn add_special_workday(&mut self, date: &NaiveDate) {
        self.special_workdays.push(date.clone());
    }

    pub fn add_special_holiday(&mut self, date: &NaiveDate) {
        self.special_holidays.push(date.clone());
    }

    pub fn is_workday(&self, date: &NaiveDate) -> bool {
        let is_special_workday = self.special_workdays.contains(date);
        let is_special_holiday = self.special_holidays.contains(date);
        let is_weekend = self.weekend_days.contains(&date.weekday());
        let is_holiday = self.holidays.contains(date);

        if is_special_workday {
            return true;
        }

        if is_special_holiday {
            return false;
        }

        if is_holiday {
            return false;
        }

        if is_weekend {
            return false;
        }

        return true;
    }
}

pub fn get_workdays(
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    work_calendar: &WorkCalendar,
) -> Vec<NaiveDate> {
    assert!(end_date >= start_date);

    let mut result = vec![];

    let mut curr_date = start_date.clone();
    while &curr_date <= end_date {
        if work_calendar.is_workday(&curr_date) {
            result.push(curr_date);
        }

        curr_date += Duration::days(1);
    }

    result
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::calendar::WorkCalendar;

    use super::get_workdays;

    #[test]
    fn no_workdays() {
        let work_calendar = WorkCalendar::new();

        let result = get_workdays(
            &NaiveDate::from_ymd_opt(2023, 12, 30).unwrap(),
            &NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
            &work_calendar,
        );

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn five_days_week() {
        let work_calendar = WorkCalendar::new();

        let result = get_workdays(
            &NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            &NaiveDate::from_ymd_opt(2023, 12, 29).unwrap(),
            &work_calendar,
        );

        assert_eq!(result.len(), 5);
    }

    #[test]
    fn week_after_new_year() {
        let mut work_calendar = WorkCalendar::new();
        work_calendar.add_holiday(&NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        work_calendar.add_holiday(&NaiveDate::from_ymd_opt(2024, 1, 2).unwrap());

        let result = get_workdays(
            &NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            &NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            &work_calendar,
        );

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn long_week() {
        let mut work_calendar = WorkCalendar::new();
        work_calendar.add_special_workday(&NaiveDate::from_ymd_opt(2024, 1, 13).unwrap());

        let result = get_workdays(
            &NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
            &NaiveDate::from_ymd_opt(2024, 1, 14).unwrap(),
            &work_calendar,
        );

        assert_eq!(result.len(), 6);
    }
}
