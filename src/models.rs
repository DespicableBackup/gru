use super::schema::minions;

#[derive(Queryable, Identifiable)]
pub struct Minion {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub key: Option<String>,
    pub ip: Option<String>,
    pub username: Option<String>,
    pub port: Option<i32>,
}

#[derive(AsChangeset)]
#[table_name="minions"]
pub struct UpdateMinion<'a> {
    pub active: Option<bool>,
    pub key: Option<Option<&'a str>>,
    pub ip: Option<Option<&'a str>>,
    pub username: Option<Option<&'a str>>,
    pub port: Option<Option<i32>>,
}

#[derive(Insertable)]
#[table_name="minions"]
pub struct NewMinion<'a> {
    pub name: &'a str,
    pub key: &'a str,
}
