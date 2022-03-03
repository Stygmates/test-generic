use uuid::Uuid;
use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub id: Uuid
}