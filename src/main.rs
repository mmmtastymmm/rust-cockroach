#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;
use std::env;

use self::accounts_tom::dsl::*;

table! {
    accounts_tom (id) {
        id -> Uuid,
        balance -> Int8,
    }
}

#[derive(Queryable)]
struct Account {
    id: Uuid,
    balance: i64,
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn get_account_balance(connection: &mut PgConnection, account_id: Uuid) -> QueryResult<i64> {
    accounts_tom
        .find(account_id)
        .select(balance)
        .first(connection)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection();

    // Example usage
    let account_id: Uuid = Uuid::parse_str("Your UUID here")?;
    let balance_value = get_account_balance(&mut connection, account_id)?;
    println!("Account balance: {:?}", balance_value);

    Ok(())
}
