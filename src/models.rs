use serde::Serialize;


#[derive(Queryable)]
#[derive(Serialize)]
pub struct Contact {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub address: String,
    pub zipcode: String,
    pub city: String,
    pub country: String,
}