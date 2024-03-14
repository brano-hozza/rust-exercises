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

# On windows

# I have no idea, 
# you probably need to download whole postgres server 
# from the website (https://www.postgresql.org/download/)
# and set PQ_LIB_DIR to the lib folder in ProgramFiles
```

5. Install diesel cli

```sh
 cargo install diesel_cli --no-default-features --features postgres
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
2. Define tables
```sql
-- up.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL
);
-- down.sql
DROP TABLE users;
```
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
3. Apply migrations -> this is going to generate `schema.rs`
```sh
diesel migration run
```

## Create models






### Sources

<https://diesel.rs/guides/getting-started.html>
