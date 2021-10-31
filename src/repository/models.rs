use chrono::{DateTime};
use chrono_tz::{TimeZone, Utc};

pub struct Tweet {
    pub id: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub user: User,
}

pub struct User {
    pub id: String,
    pub name: String,
    pub screen_name: String,
    pub description: String,
    pub protected: bool,
    pub created_at: DateTime<Utc>,
}
