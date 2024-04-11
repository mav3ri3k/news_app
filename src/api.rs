use axum::{debug_handler, extract::Path, Json};
use reqwest;
use serde::{Deserialize, Serialize};

pub mod db;

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Story {
    by: String,
    descendants: i32,
    id: i32,
    kids: Vec<i32>,
    score: i32,
    time: i64,
    title: String,
    url: String,
}

#[utoipa::path(
    get,
    path="/story/{id}",
    responses(
        (status = 200, description = "Story found", body = User),
        (status = NOT_FOUND, description = "Story not found"),
        ),
        params(
            ("id" = u32, Path, description = "Story database id to get the story"),
            )
    )]
#[debug_handler]
pub async fn story(Path(id): Path<i32>) -> Json<Story> {
    let request_path = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
    let body = reqwest::get(request_path)
        .await
        .unwrap()
        .json::<Story>()
        .await
        .unwrap();
    Json(body)
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct Comment {
    by: String,
    id: i32,
    kids: Vec<i32>,
    parent: i32,
    text: String,
    time: i64,
}

#[utoipa::path(
    get,
    path="/comment/{id}",
    responses(
        (status = 200, description = "Comment found", body = User),
        (status = NOT_FOUND, description = "Comment not found"),
        ),
        params(
            ("id" = u32, Path, description = "Comment database id to get the comment"),
            )
    )]
#[debug_handler]
pub async fn comment(Path(id): Path<i32>) -> Json<Comment> {
    let comment = Comment {
        by: "mav3ri3k".to_string(),
        id: id,
        kids: vec![21, 34],
        parent: 32,
        text: "what".to_string(),
        time: 43,
    };
    Json(comment)
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct User {
    about: String,
    created: i32,
    delay: i32,
    id: String,
    karma: i32,
    submitted: Vec<i32>,
}

#[utoipa::path(
    get,
    path="/user/{id}",
    responses(
        (status = 200, description = "User found", body = User),
        (status = NOT_FOUND, description = "User not found"),
        ),
        params(
            ("id" = u32, Path, description = "User database id to get the User"),
            )
    )]
#[debug_handler]
pub async fn user(Path(id): Path<i32>) -> Json<User> {
    let item = User {
        about: "What!".to_string(),
        created: 12,
        delay: 32,
        id: id.to_string(),
        karma: 43,
        submitted: vec![32, 34],
    };
    Json(item)
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct TopStories {
    stories_ids: Vec<i32>,
}

#[utoipa::path(
    get,
    path="/topstories",
    responses(
        (status = 200, description = "Return id to top stories", body = User),
        (status = NOT_FOUND, description = "Bad response"),
        ),
    )]
#[debug_handler]
pub async fn topstories() -> Json<TopStories> {
    let request_path = format!("https://hacker-news.firebaseio.com/v0/topstories.json");
    let body = reqwest::get(request_path)
        .await
        .unwrap()
        .json::<TopStories>()
        .await
        .unwrap();
    Json(body)
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct CacheStories {
    stories_ids: Vec<i32>,
}

#[utoipa::path(
    get,
    path="/cache_top",
    responses(
        (status = 200, description = "Cache created for top stories", body = User),
        (status = NOT_FOUND, description = "Bad response"),
        ),
    )]
#[debug_handler]
pub async fn cache() -> String {
    let request_path = format!("https://hacker-news.firebaseio.com/v0/topstories.json");
    let body = reqwest::get(request_path)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let stories_ids: Vec<u32> = serde_json::from_str(&body.as_str()).unwrap();
    db::create_cache(&stories_ids).await;
    "Done".to_string()
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct SearchResults {
    story_ids: Vec<u32>,
}

#[utoipa::path(
    get,
    path="/search/story/{word}",
    responses(
        (status = 200, description = "Story ids corresponding stories found", body = User),
        (status = NOT_FOUND, description = "Bad response"),
        ),
        params(
            ("word" = string, Path, description = "Keyword to search in story titles"),
            )
    )]
#[debug_handler]
pub async fn search_story(Path(word): Path<String>) -> Json<SearchResults> {
    match db::search_word(word) {
        Some(ids) => Json(SearchResults {
           story_ids: ids, 
        }),
        None => Json(SearchResults { 
            story_ids: vec![],
        }),
    }
}
