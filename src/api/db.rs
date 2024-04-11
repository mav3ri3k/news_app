use crate::api::Story;
use fasthash::city;
use rusqlite::{Connection, Result};

/// Removes "'" from string, as sqlite consider it as special item
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
#[derive(Debug)]
/// Internally used to work with cache
struct Search {
    hash: u32,
    word: String,
    story_ids: Vec<u32>,
}

/// Initiate local cache database
pub async fn create_cache(stories_ids: &Vec<u32>) {
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
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let story: Story = match serde_json::from_str(&body.as_str()) {
            Ok(story) => story,
            Err(_) => continue,
        };

        db.execute(
            "INSERT OR IGNORE INTO stories (id, story) VALUES (?1, ?2)",
            (&story.id, serde_json::to_vec(&story).unwrap()),
        )
        .unwrap();

        let words = split(&story.title.clean());
        for word in words {
            match search_cache_word(&db, &word) {
                Some(result) => match result {
                    Ok(search) => {
                        let mut v: Vec<u32> = Vec::new();
                        for tmp_id in search.story_ids {
                            if tmp_id == id.clone() {
                                continue;
                            }
                            v.push(tmp_id);
                        }
                        db.execute(
                            "UPDATE search
                            SET (hash, word, story_ids) = (?1, ?2, ?3)
                            WHERE search.hash = (?4)
                            ",
                            (
                                search.hash,
                                search.word,
                                serde_json::to_vec(&v).unwrap(),
                                search.hash,
                            ),
                        )
                        .unwrap();
                    }
                    Err(_) => (),
                },
                None => {
                    db.execute(
                        "insert or ignore into search (hash, word, story_ids) values (?1, ?2, ?3)",
                        (
                            city::hash32(word.clone()),
                            &word,
                            serde_json::to_vec(&vec![story.id]).unwrap(),
                        ),
                    )
                    .unwrap();
                }
            }
        }
    }
}

/// Internal helper function
fn search_cache_word(
    db: &rusqlite::Connection,
    word: &String,
) -> Option<Result<Search, rusqlite::Error>> {
    let mut stmt = db
        .prepare("SELECT * FROM search WHERE search.hash = (?1)")
        .unwrap();
    let mut search_iter = stmt
        .query_map([city::hash32(word.clone())], |row| {
            let ids: Vec<u8> = row.get(2).unwrap();
            Ok(Search {
                hash: row.get(0).unwrap(),
                word: row.get(1).unwrap(),
                story_ids: serde_json::from_slice(&ids).unwrap(),
            })
        })
        .unwrap();

    search_iter.next()
}

/// Helper function to splite title into words and serve only normalized words
fn split(string: &String) -> Vec<String> {
    let mut words = Vec::new();
    let mut tmp = String::new();

    for c in string.chars() {
        if c != ' ' && c != ':' && c != ',' {
            tmp.push(c.to_ascii_lowercase());
        } else {
            if tmp.len() > 3 {
                words.push(tmp.clone());
            }
            tmp.clear();
        }
    }
    if tmp.len() > 3 {
        words.push(tmp);
    }

    words
}

/// find given word in cache only
pub fn search_word(word: String ) -> Option<Vec<u32>> {
    let db = Connection::open("cache.db").unwrap();
    let mut stmt = db
        .prepare("SELECT * FROM search WHERE search.hash = (?1)")
        .unwrap();
    let mut search_iter = stmt
        .query_map([city::hash32(word.clone())], |row| {
            let ids: Vec<u8> = row.get(2).unwrap();
            Ok(Search {
                hash: row.get(0).unwrap(),
                word: row.get(1).unwrap(),
                story_ids: serde_json::from_slice(&ids).unwrap(),
            })
        })
        .unwrap();

    match search_iter.next() {
        Some(search) => match search {
            Ok(search) => search.story_ids.into(),
            Err(_) => None,
        },
        None => None,
    }
}
