# C00 - Tokio / Mini-redis

## Requirements
```sh
cargo install mini-redis
mini-redis-server
mini-redis-cli get foo
``` 

## Testing script

```rs
// examples/redis.rs
#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
```

## Main redis script

First we need to handle incoming connection:
```rs
#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
```

But we have problem with blocking thread, we should implement conccurency


```rs
loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
```

## Storing data

We now need to somehow store the data, we are going to use the `HashMap<String, Vec<u8>>` structure for it.
But now we have problem with accessing the data through multiple threads. We could use Arc + Mutex to solve that.

```rs
type Db = Arc<Mutex<HashMap<String, Vec<u8>>>>;
...
let db: Db = Arc::new(Mutex::new(HashMap::<String, Vec<u8>>::new()));
```

Now we can rewrite our process function to match `mini-redis` library types and use their parsing 
function to process incoming requests:

```rs
use mini_redis::Command::{self, Get, Set};
// Connection, provided by `mini-redis`, handles parsing frames from
// the socket
let mut connection = Connection::new(socket);

// Use `read_frame` to receive a command from the connection.
while let Some(frame) = connection.read_frame().await.unwrap() {
    let response = match Command::from_frame(frame).unwrap() {
        Set(cmd) => {
            // The value is stored as `Vec<u8>`
            println!("set key = {:?}, value = {:?}", cmd.key(), cmd.value());
            let mut db = db.lock().unwrap();
            db.insert(cmd.key().to_string(), cmd.value().to_vec());
            Frame::Simple("OK".to_string())
        }
        Get(cmd) => {
            println!("get key = {:?}", cmd.key());
            let db = db.lock().unwrap();
            if let Some(value) = db.get(cmd.key()) {
                // `Frame::Bulk` expects data to be of type `Bytes`. This
                // type will be covered later in the tutorial. For now,
                // `&Vec<u8>` is converted to `Bytes` using `into()`.
                Frame::Bulk(value.clone().into())
            } else {
                Frame::Null
            }
        }
        cmd => panic!("unimplemented {:?}", cmd),
    };

    // Write the response to the client
    connection.write_frame(&response).await.unwrap();
}
```