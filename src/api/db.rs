use crate::api::Story;
use sqlite;

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
pub async fn create_cache(stories_ids: &Vec<i32>) {
    let connection = sqlite::open("cache.db").unwrap();
    /*
    let query = "
        CREATE TABLE index (ngram TEXT, id INTEGER);
    ";
    connection.execute(query).unwrap();
    */

    let mut i = 1;
    for id in stories_ids {
        if i > 5 {
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

        let story: Story = serde_json::from_str(&body.as_str()).unwrap();
        let words = split(&story.title.clean());

        for word in words {
            let query = format!("INSERT INTO tri_index VALUES ('{}', {});", word, id);
            println!("Query: {:#?}", query);
            connection.execute(query).unwrap();
        }
    }
}

fn split(string: &String) -> Vec<String> {
    let mut words = Vec::new();
    let mut tmp = String::new();

    for c in string.chars() {
        if c != ' ' {
            tmp.push(c);
        } else {
            words.push(tmp.clone());
            tmp.clear();
        }
    }
    words.push(tmp);

    words
}
