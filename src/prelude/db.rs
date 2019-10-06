use rocket_contrib::databases::diesel;
use rocket_contrib::databases::*;

#[database("claim_maker_experiment")]
pub struct Connection(diesel::PgConnection);
