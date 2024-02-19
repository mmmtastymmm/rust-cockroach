#[macro_use]
extern crate diesel;
use diesel::dsl::count_star;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use rand::Rng;
use std::env;
use uuid::Uuid;

use self::accounts_example::dsl::*;

table! {
    accounts_example (id) {
        id -> Uuid,
        balance -> Int8,
    }
}

#[derive(Queryable)]
struct Account {
    id: Uuid,
    balance: i64,
}

#[derive(Insertable)]
#[table_name = "accounts_example"]
struct NewAccount {
    id: Uuid,
    balance: i64,
}

fn create_account(connection: &mut PgConnection, account: NewAccount) -> QueryResult<usize> {
    diesel::insert_into(accounts_example)
        .values(&account)
        .execute(connection)
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgresql://root@localhost:26257/defaultdb?sslmode=disable".to_string());
    PgConnection::establish(&database_url).unwrap_or_else(|e| panic!("{:?}", e))
}

fn make_table(connection: &mut PgConnection) {
    // Check for the existence of the table and create it if it doesn't exist
    let table_creation_query = r#"
        CREATE TABLE IF NOT EXISTS accounts_example (
            id UUID PRIMARY KEY,
            balance BIGINT
        );
    "#;

    match sql_query(table_creation_query).execute(connection) {
        Ok(_) => {}
        Err(e) => {
            panic!("Couldn't make the table with the following reason: {e}")
        }
    }
}

fn ensure_minimum_accounts(connection: &mut PgConnection) -> QueryResult<()> {
    let current_count = count_accounts(connection)?;

    let accounts_to_create = 100 - current_count;
    if accounts_to_create > 0 {
        for _ in 0..accounts_to_create {
            let new_account = NewAccount {
                id: Uuid::new_v4(),
                balance: 0, // Default balance
            };
            create_account(connection, new_account)?;
        }
        println!("Created {} new accounts.", accounts_to_create);
    } else {
        println!("No new accounts needed. Current count: {}", current_count);
    }

    Ok(())
}

fn get_account_balance(connection: &mut PgConnection, account_id: Uuid) -> QueryResult<i64> {
    accounts_example
        .find(account_id)
        .select(balance)
        .first(connection)
}

fn get_uuids(connection: &mut PgConnection) -> QueryResult<Vec<Uuid>> {
    accounts_example
        .order_by(balance)
        .select(id)
        .limit(1_000_000)
        .load(connection)
}

fn count_accounts(connection: &mut PgConnection) -> QueryResult<i64> {
    accounts_example.select(count_star()).first(connection)
}

fn transfer_funds(
    connection: &mut PgConnection,
    from: Uuid,
    to: Uuid,
    amount: i64,
) -> QueryResult<()> {
    connection.transaction(|conn| {
        let from_balance: i64 = accounts_example.find(from).select(balance).first(conn)?;
        assert!(from_balance >= amount, "Insufficient funds");

        diesel::update(accounts_example.find(from))
            .set(balance.eq(balance - amount))
            .execute(conn)?;

        diesel::update(accounts_example.find(to))
            .set(balance.eq(balance + amount))
            .execute(conn)?;

        Ok(())
    })
}

fn update_all_zero_balance_accounts(connection: &mut PgConnection) -> QueryResult<usize> {
    // Find all accounts with a balance of 0.
    let target_accounts: Vec<Uuid> = accounts_example
        .filter(balance.eq(0))
        .select(id)
        .load(connection)?;

    // Get the size for later
    let accounts_length = target_accounts.len();

    // Iterate over each account and update it with a new random balance.
    for account_id in target_accounts {
        let mut rng = rand::thread_rng();
        let new_balance: i64 = rng.gen_range(0..=1000);

        diesel::update(accounts_example.find(account_id))
            .set(balance.eq(new_balance))
            .execute(connection)?;
    }

    // Return the count of updated accounts
    Ok(accounts_length)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection();
    make_table(&mut connection);
    ensure_minimum_accounts(&mut connection)
        .expect("Couldn't ensure min number of accounts present");
    update_all_zero_balance_accounts(&mut connection).expect("Couldn't update accounts");

    // Example usage
    let account_ids = get_uuids(&mut connection)?;
    for local_id in account_ids {
        let balance_value = get_account_balance(&mut connection, local_id)?;
        println!("Account {} has balance {:?}", local_id, balance_value);
    }
    Ok(())
}
