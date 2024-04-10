use crate::api::Story;
use rusqlite::{Connection, Result};
use fasthash::city;

pub trait Clean {
    fn clean(&self) -> Self;
}

impl Clean for String {
    fn clean(&self) -> String {
        let mut tmp = String::new();
        for c in self.chars() {
            if c != '\'' {
                tmp.push(c);
            }
        }
        tmp
    }
}

struct Search {
    hash: i32,
    word: String,
    story_ids: Vec<i32>,
}

pub async fn create_cache(stories_ids: &Vec<i32>) {
    // create connection and set up table
    let db = Connection::open("cache.db").unwrap();
    // stories
    db.execute(
        "CREATE TABLE IF NOT EXISTS stories (
            id     INTEGER PRIMARY KEY,
            story  BLOB
        )",
        (), // empty list of parameters.
    )
    .unwrap();
    // search
    db.execute(
        "CREATE TABLE IF NOT EXISTS search (
            hash       INTEGER PRIMARY KEY,
            word       TEXT,
            story_ids  BLOB
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    let mut i = 1;
    for id in stories_ids {
        if i > 10 {
            break;
        }
        i += 1;
        let request_path = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
        let body = reqwest::get(request_path)
            .await.unwrap()
            .text()
            .await.unwrap();

        let story: Story = serde_json::from_str(&body.as_str()).unwrap();

        db.execute(
            "INSERT OR IGNORE INTO stories (id, story) VALUES (?1, ?2)",
            (&story.id, serde_json::to_vec(&story).unwrap()),
        ).unwrap();

        let words = split(&story.title.clean());
        for word in words {
            let mut stmt = db.prepare(
                "SELECT id, name, data FROM person WHERE person.hash (?1)").unwrap();
            let search_iter = stmt.query_map([city::hash32(word.clone())], |row| {
                let ids: Vec<u8> = row.get(2).unwrap();
                Ok( Search {
                    hash: row.get(0).unwrap(),
                    word: row.get(1).unwrap(),
                    story_ids: serde_json::from_slice(&ids).unwrap(),
                })
            }).unwrap();
            if search_iter.

                        db.execute(
                "INSERT OR IGNORE INTO search (hash, word, story_ids) VALUES (?1, ?2, ?3)",
                (city::hash32(word.clone()), 
                 &word, 
                 serde_json::to_vec(&vec![story.id]).unwrap()),
                ).unwrap();
        }
    }
}

fn split(string: &String) -> Vec<String> {
    let mut words = Vec::new();
    let mut tmp = String::new();

    for c in string.chars() {
        if c != ' ' || c != ':' || c != ',' {
            tmp.push(c.to_ascii_lowercase());
        } else {
            if tmp.len() > 3 {
                words.push(tmp.clone());
                tmp.clear();
            }
        }
    }
    if tmp.len() > 3 {
        words.push(tmp);
    }

    words
}
