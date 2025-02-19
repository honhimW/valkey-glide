#![allow(
    clippy::enum_variant_names,
    clippy::unused_unit,
    clippy::let_and_return,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::cast_lossless,
    clippy::blacklisted_name,
    clippy::too_many_arguments,
    clippy::trivially_copy_pass_by_ref,
    clippy::let_unit_value,
    clippy::clone_on_copy
)]

include!(concat!(env!("OUT_DIR"), "/async_callback.rs"));


#[cfg(test)]
mod test {
    use std::time::Duration;
    use anyhow::{Error, Result};
    use log::info;
    use redis::{AsyncCommands, Client, Cmd, RedisResult};
    use crate::redis::query;
    use std::sync::{Arc, LockResult, Mutex, RwLock};
    use std::thread::sleep;
    use redis::aio::MultiplexedConnection;
    use redis::GlideConnectionOptions;
    use tokio::join;
    use crate::redis::create_client_from_url;
    use crate::{spawn, RUNTIME};

    #[test]
    fn test() -> Result<()> {
        tracing_subscriber::fmt().init();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("rs-4-java")
            .build()?;
        let handle = runtime.spawn(async move {
            info!("tokio");
            tokio::time::sleep(Duration::from_secs(1)).await;
            info!("tokio");
        });
        // runtime.block_on(handle)?;
        Ok(())
    }

    // #[derive(Clone)]
    struct RedisClient {
        client: redis::Client,
        connection: Arc<RwLock<Option<redis::aio::MultiplexedConnection>>>,
        con: Option<redis::Connection>,
    }

    impl RedisClient {

        fn new_client(url: &str) -> RedisClient {
            let client = create_client_from_url(url)
                .expect("Failed to create redis client");
            RedisClient {
                client,
                connection: Arc::new(RwLock::new(None)),
                con: None,
            }
        }

        fn connect(&mut self) {
            let client = self.client.clone();
            let arc = Arc::clone(&self.connection);
            spawn(async move {
                if let Ok(connection) = client.get_multiplexed_async_connection(GlideConnectionOptions::default()).await {
                    if let Ok(mut guard) = arc.write() {
                        *guard = Some(connection);
                    }
                }
            });
        }
    }

    #[test]
    fn connect_async() -> Result<()> {
        let mut client = RedisClient::new_client("redis://:123456@10.37.1.132:6379");
        client.connect();

        sleep(Duration::from_secs(1));
        match client.connection.read() {
            Ok(guard) => {
                match *guard {
                    None => {
                        panic!("none");
                    }
                    Some(ref c) => {
                        let mut connection = c.clone();
                        spawn(async move {
                            let json: RedisResult<String> = Cmd::get("json").query_async(&mut connection).await;
                            match json {
                                Ok(s) => println!("{s}"),
                                Err(_) => {},
                            }
                            Ok::<(), Error>(())
                        });
                        sleep(Duration::from_secs(1));
                    }
                }
            }
            Err(_) => {}
        };
        Ok(())
    }
}
