// use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use diesel::prelude::*;
use diesel::*;
// use diesel::sql_types::Datetime;
use serde::{Serialize, Deserialize};
// use rocket_contrib::databases::diesel::PgConnection;
mod model;
use crate::schema;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub addresses: Vec<Addresses>,
    pub claims: Vec<Claims>,
    pub identity: Option<Identity>,

    created_at: i32,
    updated_at: i32,
}

impl model::Base for User {
    type Model: Self;
    type Table: user;
    type Connection: PgConnection;
}

pub enum PropertyType {
    House,
    ApartmantFlat,
    CommercialPremises,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "addresses"]
pub struct Addresses {
    pub id: i32,
    pub user_id: i32,
    pub door_number: Option<i8>,
    pub postcode: Option<String>,
    pub description: Option<PropertyType>,

    created_at: i32,
    updated_at: i32,
}

pub enum LossDamageType {
  FireDamage,
  WaterDamage,
  MouldDamage,
  StormDamage,
  TheftAndVandalism,
  Sinkhole,
  MarineVessel,
  BusinessInteruption,

}

pub enum YNType {
    Yes,
    No
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "claims"]
pub struct Claims {
    pub id: u32,
    pub user_id: u32,
    pub against: Option<LossDamageType>,
    pub structure_affected: Option<YNType>,
    pub documents: Vec<Documents>,

    date_of_incident: u32,
    created_at: u32,
    updated_at: u32,
}


#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "documents"]
pub struct Documents {
    pub id: u32,
    pub claim_id: u32,
    pub url: Option<String>,
    pub file_name: Option<String>,
    pub description: Option<String>,

    created_at: u32,
    updated_at: u32,
}

enum ClaimentType {
    Homeowner,
    Busisness,
    LandlordPropertyManager
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "identity"]
pub struct Identity {
    pub id: u32,
    pub user_id: u32,
    pub is_a: Option<ClaimentType>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,

    created_at: u32,
    updated_at: u32,
}

enum PNumberType {
    Home,
    Mobile,
    Work,
    Other,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "phone_numbers"]
pub struct PhoneNumbers {
    pub id: i32,
    pub identity: Option<Identity>,
    pub is_a: Option<PNumberType>,
    pub name: Option<String>,
    pub number: Option<u8>,
    pub extention: Option<u8>,
    pub calling_code: Option<u8>,

    created_at: u32,
    updated_at: u32,
}
