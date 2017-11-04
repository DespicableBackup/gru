use super::schema::minions;

#[derive(Queryable)]
pub struct Minion {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name="minions"]
pub struct NewMinion<'a> {
    pub name: &'a str,
}
