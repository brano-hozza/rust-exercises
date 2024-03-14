# C01 Lets add some diesel

## Requirements

First we need some DB to run on
1. Create empty directory `data`
2. Run docker and mount it to this directory

```sh
docker run -itd -e POSTGRES_USER=root -e POSTGRES_PASSWORD=root -p 5432:5432 -v ./data:/var/lib/postgresql/data --name postgresql postgres
```

3. Connect to DB and create a database

```sql
CREATE DATABASE rust_diesel;
```

4. We also need some driver to connect to the DB

```sh
# On mac
brew install libpq
brew link --force libpq
PQ_LIB_DIR="$(brew --prefix libpq)/lib"

# On linux
sudo apt-get install libpq-dev
# Set PQ_LIB_DIR to lib path

# On windows

# I have no idea, 
# you probably need to download whole postgres server 
# from the website (https://www.postgresql.org/download/)
# and set PQ_LIB_DIR to the lib folder in ProgramFiles
```

5. Install diesel cli and its extension

```sh
# https://diesel.rs/guides/getting-started.html
cargo install diesel_cli --no-default-features --features postgres
# https://github.com/abbychau/diesel_cli_ext
 cargo install diesel_cli_ext
```

6. Setup diesel
```sh
diesel setup
```

## Migrations

1. Generate some Migrations

```sh
diesel migration generate create_users
diesel migration generate create_posts
```
2. Define user migrations
```sql
-- up.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL
);
-- down.sql
DROP TABLE users;
```
3. Define posts migrations
```sql
-- up.sql-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE posts
    ADD FOREIGN KEY (user_id)
    REFERENCES users (id);
-- down.sql
DROP TABLE posts;
```
4. Apply migrations -> this is going to generate `schema.rs`
```sh
diesel migration run
```

## Prepare function to establish connection
We also need to prepare function to open new connection to DB. Do not forget to load ENV variables first.
```rs
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
```

## Create models

```rs
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub username: String,
}

```
We could also use diesel_ext to generate models and even proto-files

## Use models
For example if we want to query our models we could write:
```rs
let users = users::table
        .load::<User>(conn)
        .expect("Error loading users");
```
Or to create new users:
```rs
let new_user = diesel::insert_into(users::table)
        .values(&payload)
        .get_result(conn)
        .unwrap();
```





### Sources

<https://diesel.rs/guides/getting-started.html>
