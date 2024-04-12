# News App Backend
Implementation of Rest API for a sudo news app.
It is written in rust. 

This is demo program, thus it acts as shim on Hacker News API. The API endpoints fetch data from Hacker News.

> [!IMPORTANT] 
> Search only works for offline stories in cache
> Thus to use, first create cache using `/cache_top`

## Setup for dev mode
To start you need rust toolchain installed
Start in dev mode using command `cargo run` 
It will start server at `localhost:3000` 
Refer to `localhost:3000/docs` for API documentation

### Optionally
For optimized build run: `cargo build -r` and `cargo run -r`
For internal documentation: `cargo doc --open`


