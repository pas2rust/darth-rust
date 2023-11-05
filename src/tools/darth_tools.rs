use crate::tools::random_bytes::random_bytes;
use chrono::{DateTime, Utc, Local, Duration};
use uuid::Uuid;
pub struct DarthTools;

pub trait DarthToolsTrait {
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
    fn new_uuid() -> String;
    fn new_date_utc_now() -> DateTime<Utc>;
    fn new_date_local_now() -> DateTime<Local>;
    fn new_random_bytes(
        gen_uppercase: Option<u32>,
        gen_lowercase: Option<u32>,
        gen_number: Option<u32>,
        gen_special_characters: Option<u32>,
        gen_emoji: Option<u32>,
    ) -> String;
}

impl DarthToolsTrait for DarthTools {
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
    fn new_uuid() -> String {
       Uuid::new_v4().to_string()
    }
    fn new_random_bytes(
            gen_uppercase: Option<u32>,
            gen_lowercase: Option<u32>,
            gen_number: Option<u32>,
            gen_special_characters: Option<u32>,
            gen_emoji: Option<u32>,
        ) -> String {
        random_bytes(
            gen_uppercase,
            gen_lowercase,
            gen_number,
            gen_special_characters,
            gen_emoji,
        )
    }
}