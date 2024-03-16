// Cron input could be "* * * * * Mon" which means every minute every Monday.

pub struct Fields {
    minute: u64,
    hour: u64,
    day_of_month: u64,
    month: u64,
    day_of_week: u64,
}

pub struct CronSchedule {}

impl CronSchedule {
    pub fn next_date() -> String {
        chrono::Utc::now().to_rfc3339()
    }
}
