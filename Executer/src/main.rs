use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },
    // The timer we wrote in the previous section:
//    timer_future::TimerFuture,
};

use futures::executor::block_on;

async fn hello_world() {

println!("Hello, world!");
}

fn main() {
//    println!("Hello, world!");
    let f1 = hello_world();
    block_on(f1);
}
