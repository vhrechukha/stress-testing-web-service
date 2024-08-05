use super::schema::employees;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub position: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = employees)]
pub struct NewEmployee {
    pub name: String,
    pub age: i32,
    pub position: String,
}
