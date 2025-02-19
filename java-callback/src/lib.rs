mod async_callback;
pub mod jni_c_header;
mod redis;

use std::future::Future;
pub use crate::async_callback::*;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use tokio::runtime::Runtime;
use tokio::task::{JoinHandle, LocalSet};

pub const MAX_REQUEST_ARGS_LENGTH_IN_BYTES: usize = 2_i32.pow(12) as usize; // TODO: find the right number

pub const TYPE_STRING: &str = "string";
pub const TYPE_LIST: &str = "list";
pub const TYPE_SET: &str = "set";
pub const TYPE_ZSET: &str = "zset";
pub const TYPE_HASH: &str = "hash";
pub const TYPE_STREAM: &str = "stream";

struct Level(i32);

static RUNTIME: Lazy<RwLock<Runtime>> = Lazy::new(|| {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("rs4java")
        .build()
        .expect("Failed to create tokio runtime");
    RwLock::new(runtime)
});

static LOCAL_SET: Lazy<RwLock<LocalSet>> = Lazy::new(|| {
    let local_set = LocalSet::new();
    RwLock::new(local_set)
});

fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    LOCAL_SET.read().expect("Failed to get tokio local set").spawn(future)
    // tokio::task::spawn_local()
    // RUNTIME.read().expect("Failed to get tokio runtime")
    //     .spawn(future)
}
