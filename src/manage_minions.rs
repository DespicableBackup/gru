use diesel;
use diesel::prelude::*;
use rand::{OsRng, Rng};
use base64::encode;
use models;

const KEY_LENGTH: usize = 64;

pub fn generate_key() -> String {
    let mut rng = OsRng::new().expect("Could not get a proper random generator");
    let mut buf: Vec<u8> = vec![0; KEY_LENGTH];
    rng.fill_bytes(&mut buf);

    encode(&buf)
}

pub fn create_minion(conn: &SqliteConnection, name: &str) {
    use schema::minions;

    let key = generate_key();

    let minion = models::NewMinion {
        name: name,
        key: &key,
    };

    diesel::insert(&minion)
        .into(minions::table)
        .execute(conn)
        .expect("Error saving new minion");

    println!("Minion API key: {}", key);
}

pub fn delete_minion(conn: &SqliteConnection, name: &str) {
    use schema::minions::dsl;

    diesel::delete(dsl::minions.filter(dsl::name.eq(name)))
        .execute(conn)
        .expect("Error deleting minion");
}

pub fn list_minions(conn: &SqliteConnection) {
    use schema::minions::dsl::*;
    let results :Vec<models::Minion> = minions.load(conn).expect("Could not retrieve minions");

    for minion in results {
        // TODO: get rid of allocation
        println!(
            "{}\t{}\t{}@{}:{}",
            minion.name,
            minion.active,
            minion.username.unwrap_or("-".to_owned()),
            minion.ip.unwrap_or("-".to_owned()),
            minion.port.unwrap_or(22)
            );
    }
}

pub fn revoke_minion(conn: &SqliteConnection, name: &str) {
    use schema::minions::dsl;

    diesel::update(dsl::minions.filter(dsl::name.eq(name)))
        .set(&models::UpdateMinion {
            active: Some(false),
            key: Some(None),
            ip: Some(None),
            username: Some(None),
            port: Some(Some(22)),
        })
        .execute(conn)
        .expect(&format!("Could not revoke {}", name));
}

pub fn regen_minion(conn: &SqliteConnection, name: &str) {
    use schema::minions::dsl;

    let key = generate_key();

    diesel::update(dsl::minions.filter(dsl::name.eq(name)))
        .set(&models::UpdateMinion {
            // Disable the minion as the key may have been compromised
            active: Some(false),
            key: Some(Some(&key)),
            ip: Some(None),
            username: Some(None),
            port: Some(Some(22)),
        })
        .execute(conn)
        .expect(&format!("Could not revoke {}", name));
    println!("New key: {}", key);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generate_key() {
        let key = generate_key();
        // base64 encoding handle 6 bytes per character
        let expected_length = ( KEY_LENGTH % 6 + KEY_LENGTH) / 6 * 8 ;
        println!("expected_length: {}", KEY_LENGTH/6);
        assert!(key.capacity() == expected_length);
    }
}
