# C01 Lets add some diesel

## Requirements
First we need some DB to run on
`docker run -itd -e POSTGRES_USER=root -e POSTGRES_PASSWORD=root -p 5432:5432 -v /data:/var/lib/postgresql/data --name postgresql postgres`

`CREATE DATABASE rust_diesel;`

For this example we only need postgres version
`cargo install diesel_cli --no-default-features --features postgres`


### Sources
https://diesel.rs/guides/getting-started.html