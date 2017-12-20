use diesel;
use diesel::prelude::*;
use rand::{OsRng, Rng};
use base64::encode;
use failure::Error;
use models;

const KEY_LENGTH: usize = 64;

pub fn generate_key() -> String {
    let mut rng = OsRng::new().expect("Could not get a proper random generator");
    let mut buf: Vec<u8> = vec![0; KEY_LENGTH];
    rng.fill_bytes(&mut buf);

    encode(&buf)
}

pub fn create_minion(conn: &SqliteConnection, name: &str) -> Result<(), Error> {
    use schema::minions;

    let key = generate_key();

    let minion = models::NewMinion {
        name: name,
        key: &key,
    };

    diesel::insert_into(minions::table)
        .values(&minion)
        .execute(conn)?;

    println!("Minion API key: {}", key);
    Ok(())
}

pub fn delete_minion(conn: &SqliteConnection, name: &str) -> Result<(), Error> {
    use schema::minions::dsl;

    diesel::delete(dsl::minions.filter(dsl::name.eq(name))).execute(conn)?;
    Ok(())
}

pub fn list_minions(conn: &SqliteConnection) -> Result<(), Error> {
    use schema::minions::dsl::*;
    let results: Vec<models::Minion> = minions.load(conn)?;

    for minion in results {
        // TODO: get rid of allocation
        println!(
            "{}\t{}\t{}@{}:{}",
            minion.name,
            minion.active,
            minion.username.unwrap_or_else(|| "-".to_owned()),
            minion.ip.unwrap_or_else(|| "-".to_owned()),
            minion.port.unwrap_or(22)
        );
    }
    Ok(())
}

pub fn revoke_minion(conn: &SqliteConnection, name: &str) -> Result<(), Error> {
    use schema::minions::dsl;

    diesel::update(dsl::minions.filter(dsl::name.eq(name)))
        .set(&models::UpdateMinion {
            active: Some(false),
            key: Some(None),
            ip: Some(None),
            port: Some(Some(22)),
            username: Some(None),
            directory: Some(None),
        })
        .execute(conn)?;
    Ok(())
}

pub fn regen_minion(conn: &SqliteConnection, name: &str) -> Result<(), Error> {
    use schema::minions::dsl;

    let key = generate_key();

    diesel::update(dsl::minions.filter(dsl::name.eq(name)))
        .set(&models::UpdateMinion {
            // Disable the minion as the key may have been compromised
            active: Some(false),
            key: Some(Some(&key)),
            ip: Some(None),
            port: Some(Some(22)),
            username: Some(None),
            directory: Some(None),
        })
        .execute(conn)?;
    println!("New key: {}", key);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generate_key() {
        let key = generate_key();
        // base64 encoding handle 6 bytes per character
        let expected_length = (KEY_LENGTH % 6 + KEY_LENGTH) / 6 * 8;
        println!("expected_length: {}", KEY_LENGTH / 6);
        assert!(key.capacity() == expected_length);
    }
}
