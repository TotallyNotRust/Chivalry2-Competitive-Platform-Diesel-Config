use chrono::{Local, NaiveDateTime};
use diesel::{prelude::Insertable, Associations, Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::lib::database::schema::*;

#[derive(
    Queryable, Eq, Insertable, Identifiable, Debug, PartialEq, Clone, Deserialize, Serialize,
)]
#[diesel(table_name = account)]
pub struct Account {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub email: String,
    #[serde(skip_serializing)]
    pub salted_password: String,
    #[serde(skip_serializing)]
    pub account_closed: bool,
    #[serde(skip_serializing)]
    pub punishment_id: Option<i32>,
}

impl Account {
    pub fn default() -> Account {
        return Account {
            id: 0,
            username: "".to_owned(),
            email: "".to_owned(),
            salted_password: "".to_owned(),
            account_closed: true,
            punishment_id: None,
        };
    }
}

#[derive(Queryable, Insertable, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[diesel(table_name = account)]
pub struct NewAccount {
    pub username: String,
    pub email: String,
    pub salted_password: String,
    pub account_closed: bool,
    pub punishment_id: Option<i32>,
}

#[derive(
    Associations,
    Queryable,
    Insertable,
    Identifiable,
    Debug,
    PartialEq,
    Clone,
    Deserialize,
    Serialize,
)]
#[diesel(table_name = tokens)]
#[diesel(belongs_to(Account))]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub account_id: i32,
    pub valid_until: NaiveDateTime,
    pub invalidated: bool,
}
#[derive(Queryable, Insertable, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[diesel(table_name = tokens)]
pub struct NewToken {
    pub token: String,
    pub account_id: i32,
    pub valid_until: NaiveDateTime,
    pub invalidated: bool,
}

impl Token {
    pub fn is_expired(&self) -> bool {
        return self.valid_until.lt(&Local::now().naive_utc());
    }
}

#[derive(Queryable, Insertable, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[diesel(table_name = made_matches)]
pub struct Match {
    pub id: i64,
    pub gamemode: i32,
}
#[derive(Queryable, Insertable, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[diesel(table_name = made_matches)]
pub struct NewMatch {
    pub gamemode: i32,
}