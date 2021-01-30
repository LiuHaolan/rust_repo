use async_std::prelude::*;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use std::fs;
use async_std::task::spawn;
use futures::stream::StreamExt;
use std::time::Duration;

#[async_std::main]
async fn main(){

      let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
/*
    for stream in listener.incoming() {
        let stream = stream.unwrap();

    //    println!("Connection established!");
        handle_connection(stream).await;
    }
*/

       listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}

use async_std::task;
async fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
 let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/hello.html")
    } else if buffer.starts_with(sleep) {
          task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/hello.html")
    } 
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./html/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
