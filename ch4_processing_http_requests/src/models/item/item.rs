use crate::schema::to_do;

#[derive(Queryable, Identifiable)]
#[table_name = "to_do"]
pub struct Item {
    pub id: u32,
    pub title: String,
    pub status: String,
}
