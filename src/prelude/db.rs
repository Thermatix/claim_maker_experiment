use rocket_contrib::databases::diesel;
use rocket_contrib::databases::database;

#[database("claim_maker_experiment")]
pub struct Connection(diesel::PgConnection);
