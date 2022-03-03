pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::RunQueryDsl;
use diesel::Connection;
use diesel::Table;
use std::env;
use dotenv::dotenv;
use diesel::PgConnection;
use diesel::query_builder::{InsertStatement};
use diesel::query_dsl::methods::{ExecuteDsl};

pub fn insert<T, M>(conn: &PgConnection, table: T, records: M)
-> usize where
    T: Table,
    M: diesel::Insertable<T>,
    InsertStatement<T, M::Values>: ExecuteDsl<PgConnection>,
{
    diesel::insert_into(table)
        .values(records)
        .execute(conn).unwrap()
}

pub fn update<T, M>(
    conn: &PgConnection, table: T, records: M
) -> std::result::Result<usize, diesel::result::Error> {
    diesel::update(table.filter(id.eq(records.id)))
        .set(records)
        .execute(conn)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use crate::schema::users::dsl::users;
    use crate::insert;
    use crate::establish_connection;
    use crate::models::NewUser;
    use uuid::Uuid;
    #[test]
    fn test_insert(){
            let id = Uuid::new_v4();
            let user = NewUser{
                id
            };
            let connection = establish_connection();
            let number_user_inserted = insert(&connection, users, user);
            assert_eq!(number_user_inserted, 1);
        }

    #[test]
    fn test_delete(){
        
    }
}