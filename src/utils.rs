use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use chrono_tz::Tz;

pub fn parse_and_describe(timestamp: DateTime<Utc>, tz: Tz) -> Result<String, chrono::ParseError> {
    let local_time: DateTime<Tz> = timestamp.with_timezone(&tz);

    // Get the current time in the same timezone
    let now: DateTime<Tz> = Utc::now().with_timezone(&tz);

    // Calculate the duration ago as a human-readable string
    let duration_ago = HumanTime::from(now - local_time);

    Ok(format!("{} ago", duration_ago))
}
