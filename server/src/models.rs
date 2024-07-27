use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, diesel::Identifiable, Serialize)]
#[diesel(table_name = crate::schema::clients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    pub id: i32,
    pub gid: i32,
    pub paid: bool
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::clients)]
pub struct NewClient {
    pub gid: i32
}
