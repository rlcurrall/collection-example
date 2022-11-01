use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Insertable, Queryable};
use poem_openapi::Object;

use crate::schema::comics;

#[derive(Queryable, Object)]
pub struct Comic {
    pub id: i32,
    pub username: String,
    pub title: String,
    pub issue_number: String,
    pub main_character: String,
    pub genre: String,
    pub cover_year: NaiveDate,
    pub publisher: String,
    pub grade: f64,
    pub price: f64,
    pub image_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Object)]
#[table_name = "comics"]
pub struct NewComic {
    pub username: String,
    pub title: String,
    pub issue_number: String,
    pub main_character: String,
    pub genre: String,
    pub cover_year: NaiveDate,
    pub publisher: String,
    pub grade: f64,
    pub price: f64,
    pub image_url: String,
}

#[derive(AsChangeset, Object)]
#[table_name = "comics"]
pub struct UpdateComic {
    pub title: Option<String>,
    pub issue_number: Option<String>,
    pub main_character: Option<String>,
    pub genre: Option<String>,
    pub cover_year: Option<NaiveDate>,
    pub publisher: Option<String>,
    pub grade: Option<f64>,
    pub price: Option<f64>,
    pub image_url: Option<String>,
}

#[derive(AsChangeset, Object)]
#[table_name = "comics"]
pub struct ReplaceComic {
    pub title: String,
    pub issue_number: String,
    pub main_character: String,
    pub genre: String,
    pub cover_year: NaiveDate,
    pub publisher: String,
    pub grade: f64,
    pub price: f64,
    pub image_url: String,
}
