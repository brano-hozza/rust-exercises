---
theme: teach-rs
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 5.1: Rust for Web"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 5.1: Rust for Web"
---

# Rust programming


## Rust for web

---

# Introduction

- Branislav Hozza
- Student@FIIT
- Rust enthusiast
- [Github](https://github.com/brano-hozza)
- xhozza@stuba.sk

---
layout: default
---

# Learning objective

- Web-focused crates
- Tracing
- State managment
- Serverless
  
---
layout: default
---

# Contents

- State of Art
- Front-end & WASM
- Tracing
- Back-end
- Database
  - SQL
  - NoSQL
- Serverless
- Best-practices

---
layout: section
---

# State of Art

---
layout: default
---

# [Are we web yet?](https://www.arewewebyet.org/)

- "Yes! And it's freaking fast!"
- Several web frameworks exist
  - [`rocket`](https://rocket.rs/)
  - [`actix-web`](https://actix.rs/)
  - [`warp`](https://github.com/seanmonstar/warp)
  - [`axum`](https://github.com/tokio-rs/axum)
  - ...lots more
- Several DB drivers and ORMs
- Much more!

*Tip: have a look if you want to do web stuff in your final project*
---
layout: section
---

# Front-end & WASM


---
layout: default
---

# Yew 

- [Build modern apps with WebAssembly](https://yew.rs/docs/tutorial)
- Install WASM target `rustup target add wasm32-unknown-unknown`
- Install trunk `cargo install --locked trunk`
- You are ready to go!

---
layout: default
---

# Yew 

- HTML with `html!` macro

```rust
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <p> Hello </p>
        </div>
    }
}
```

---
layout: default
---

# Yew 

- CSS with `classes!` macro

```rust
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <p class={classes!("container", "px-2")}> Hello </p>
        </div>
    }
}
```

---
layout: default
---

# Yew 

- JS with RS

```rust
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let optionalClass = Some("class")

    html! {
        <div>
            <button class={classes!(optionalClass)} {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
```

---
layout: default
---

# Yew 

- Simple setup

```rust
fn main() {
    yew::Renderer::<App>::new().render();
}
```

- Create `index.html` in the root

```html
<!doctype html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>Yew App</title>
    </head>
    <body></body>
</html>
```

- Run with `trunk serve`
---
layout: default
---

# Yew - trunk

- Custom configuration with `Trunk.toml`

```toml
[serve]
# The address to serve on LAN.
address = "127.0.0.1"
# The address to serve on WAN.
# address = "0.0.0.0"
# The port to serve on.
port = 8000
```
---
layout: section
---
# Tracing
---
layout: default
---

# Tracing

- span
- event
- subscriber
  
---
layout: default
---

# Tracing

- Crate `tracing` 
- Structured logging
```rust
use tracing::{event, span, Level};

// records an event outside of any span context:
event!(Level::INFO, "something happened");

let span = span!(Level::INFO, "my_span");
let _guard = span.enter();

// records an event within "my_span".
event!(Level::INFO, "something happened inside my_span");

span.exit()
// records an event outside "my_span".
event!(Level::INFO, "something happened outside my_span");
```

---
layout: default
---

# Tracing subscription

- Crate `tracing-subscriber` 

```rust
// Standard output
let stdout_log = tracing_subscriber::fmt::layer().pretty();

// File output
let debug_file = File::create("debug.log").expect("Unable to create debug.log");
let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(debug_file));

tracing_subscriber::registry()
    .with(stdout_log.with_filter(filter::LevelFilter::INFO))
    .with(debug_log.with_filter(filter::LevelFilter::DEBUG))
    .init();

```
---
layout: default
---

# Tracing subscription - output

```txt
  2024-04-06T15:26:50.450537Z  INFO c06_tracing: something happened
    at src/main.rs:20

  2024-04-06T15:26:50.450799Z  INFO c06_tracing: something happened inside my_span
    at src/main.rs:27
    in c06_tracing::my_span

  2024-04-06T15:26:50.450901Z  INFO c06_tracing: something happened outside my_span
    at src/main.rs:31
```
---
layout: section
---
# Back-end
Tokio

---
layout: default
---

# Tokio

- Async runtime
- Built for network apps
- Large ecosystem



---
layout: default
---

# Tokio - Listener

```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
```

---
layout: default
---

# Tokio - Shared state

- User status database (KV Storage)
- Key: username
- Value: status
- `Arc` + `Mutex` = ❤️

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::new()));

    // ...
}
```

---
layout: section
---
# Back-end
Axum

---
layout: default
---

# Axum

- [July 30, 2021](https://tokio.rs/blog/2021-07-announcing-axum)
- Tokio-based
- Simple usage
- Focused on web-development

---
layout: default
---

# Axum

```rust
use axum::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
```
---
layout: default
---
# Axum - extractors

```rust
use axum::{prelude::*, extract::Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

async fn create_user(Json(payload): Json<CreateUser>) {
    // `payload` is a `CreateUser`
}

let app = route("/users", post(create_user));
```
---
layout: default
---

# Axum - response data

- Handlers can return anything that implements IntoResponse and it will automatically be converted into a response

```rust
// Returning a tuple of `StatusCode` and another `IntoResponse` will
// change the status code
async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}

// `Html` gives a content-type of `text/html`
async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
```

---
layout: default
---

# Axum - routing

```rust
use axum::prelude::*;

let app = route("/", get(root))
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(show_user).delete(delete_user));
```

---
layout: default
---

# Axum - state managment

```rust
// Must be #[async-trait] implementation
type MyService = Arc<dyn service::MyService + Send + Sync>;

pub struct AppState {
    pub service: MyService,
}
// ...
let state = routes::RouterState {
    service: Arc::from(MyServiceImpl {}) as _,
};
// ... 
pub async fn get_all(State(state): State<RouterState>) -> Result<Json<Vec<Post>>, ApiError> {
    state
        .post_service
        .get_all()
        .await
        .map(Json)
        .map_err(Into::into)
}
```

---
layout: section
---

# The end