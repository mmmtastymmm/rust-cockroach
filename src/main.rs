#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use uuid::Uuid;

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
        .unwrap_or("postgresql://root@localhost:26257/defaultdb?sslmode=disable".to_string());
    match PgConnection::establish(&database_url)
    {
        Ok(connection) => {connection}
        Err(e) => {panic!("{:?}", e)}
    }
}

fn get_account_balance(connection: &mut PgConnection, account_id: Uuid) -> QueryResult<i64> {
    accounts_tom
        .find(account_id)
        .select(balance)
        .first(connection)
}

fn get_uuids(connection: &mut PgConnection) -> QueryResult<Vec<Uuid>>{
    accounts_tom.select(id).limit(1_000_000).load(connection)
}

fn transfer_funds(
    connection: &mut PgConnection,
    from: Uuid,
    to: Uuid,
    amount: i64,
) -> QueryResult<()> {
    connection.transaction(|conn| {
        let from_balance: i64 = accounts_tom.find(from).select(balance).first(conn)?;
        assert!(from_balance >= amount, "Insufficient funds");

        diesel::update(accounts_tom.find(from))
            .set(balance.eq(balance - amount))
            .execute(conn)?;

        diesel::update(accounts_tom.find(to))
            .set(balance.eq(balance + amount))
            .execute(conn)?;

        Ok(())
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection();

    // Example usage
    let account_ids = get_uuids(&mut connection)?;
    for local_id in account_ids {
        let balance_value = get_account_balance(&mut connection, local_id)?;
        println!("Account {} has balance {:?}", local_id, balance_value);
    }
    Ok(())
}
