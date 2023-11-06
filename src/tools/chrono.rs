use chrono::{DateTime, Duration, Local, Utc};

use super::darth_tools::DarthTools;
pub trait ChronoTrait {
    fn new_date_utc_add_time_by_weeks(weeks: i64) -> DateTime<Utc>;
    fn new_date_utc_add_time_by_days(days: i64) -> DateTime<Utc>;
    fn new_date_utc_add_time_by_hours(hours: i64) -> DateTime<Utc>;
    fn new_date_utc_add_time_by_minutes(minutes: i64) -> DateTime<Utc>;
    fn new_date_utc_add_time_by_seconds(seconds: i64) -> DateTime<Utc>;
    fn new_date_local_add_time_by_weeks(weeks: i64) -> DateTime<Local>;
    fn new_date_local_add_time_by_days(days: i64) -> DateTime<Local>;
    fn new_date_local_add_time_by_hours(hours: i64) -> DateTime<Local>;
    fn new_date_local_add_time_by_minutes(minutes: i64) -> DateTime<Local>;
    fn new_date_local_add_time_by_seconds(seconds: i64) -> DateTime<Local>;
    fn new_date_utc_now() -> DateTime<Utc>;
    fn new_date_local_now() -> DateTime<Local>;
}

impl ChronoTrait for DarthTools {
    fn new_date_utc_add_time_by_hours(hours: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::hours(hours);
        now + duration
    }
    fn new_date_utc_add_time_by_minutes(minutes: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::minutes(minutes);
        now + duration
    }
    fn new_date_utc_add_time_by_seconds(seconds: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::seconds(seconds);
        now + duration
    }
    fn new_date_utc_add_time_by_weeks(weeks: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::weeks(weeks);
        now + duration
    }
    fn new_date_utc_add_time_by_days(days: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let duration = Duration::days(days);
        now + duration
    }
    fn new_date_local_add_time_by_minutes(minutes: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::minutes(minutes);
        now + duration
    }
    fn new_date_local_add_time_by_weeks(weeks: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::weeks(weeks);
        now + duration
    }
    fn new_date_local_add_time_by_seconds(seconds: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::seconds(seconds);
        now + duration
    }
    fn new_date_local_add_time_by_hours(hours: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::hours(hours);
        now + duration
    }
    fn new_date_local_add_time_by_days(days: i64) -> DateTime<Local> {
        let now = Local::now();
        let duration = Duration::days(days);
        now + duration
    }
    fn new_date_local_now() -> DateTime<Local> {
        Local::now()
    }
    fn new_date_utc_now() -> DateTime<Utc> {
        Utc::now()
    }
}