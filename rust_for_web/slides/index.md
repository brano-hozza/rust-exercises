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
- State managment
- Serverless
  
---
layout: default
---

# Contents

- State of Art
- Front-end & WASM
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
layout: default
---

# Axum demo: setting up server

```rust
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // set up shared, mutable state.
    let app_state = Arc::new(Mutex::new(Vec::new()));
    // build our application with a route
    let app = Router::new()
        .route("/:name", get(handler))
        .with_state(app_state);
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---
layout: default
---
# Axum demo: request hander

```rust
/// A very long type name warrants a type alias
type AppState = State<Arc<Mutex<Vec<String>>>>;

async fn handler(
    Path(name): Path<String>,
    State(past_names): State<AppState>,
) -> Html<String> {
    let mut response = format!("<h1>Hello, {name}!</h1>");

    // Of course, locking here is not very fast
    let mut past_names = past_names.lock().await;

    if !past_names.is_empty() {
        response += "<h2>Names we saw earlier:</h2>";
        past_names
            .iter()
            .for_each(|name| response += &format!("<p>{name}</p>"))
    }

    past_names.push(name);

    Html(response)
}
```
---