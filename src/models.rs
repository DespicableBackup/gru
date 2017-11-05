use super::schema::minions;

#[derive(Queryable, Identifiable)]
pub struct Minion {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub key: Option<String>,
}

#[derive(AsChangeset)]
#[table_name="minions"]
pub struct UpdateMinion {
    pub active: Option<bool>,
    pub key: Option<Option<String>>,
}

#[derive(Insertable)]
#[table_name="minions"]
pub struct NewMinion<'a> {
    pub name: &'a str,
    pub key: &'a str,
}
