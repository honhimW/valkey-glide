use anyhow::{Context, Result};
use redis::{Client, Cmd, GlideConnectionOptions};
use redis::aio::MultiplexedConnection;

#[macro_export]
macro_rules! str_cmd {
    ($cmd:expr) => {{
        let mut command = Cmd::new();
        let parts: Vec<String> = split_args($cmd);
        for arg in &parts {
            command.arg(arg);
        }
        command
    }};
}

pub fn create_client_from_url(url: &str) -> Result<Client> {
    Client::open(url).context("Failed to create redis client")
}

pub async fn query(client: &Client, cmd: impl Into<String>) -> Result<String> {
    let mut connection = client.get_multiplexed_async_connection(GlideConnectionOptions::default()).await?;
    let value: String = str_cmd!(cmd).query_async(&mut connection).await?;
    Ok(value)
}

pub async fn do_query(ref mut con: MultiplexedConnection, cmd: impl Into<String>) -> Result<String> {
    let value: String = str_cmd!(cmd).query_async(con).await?;
    Ok(value)
}


pub fn split_args(cmd: impl Into<String>) -> Vec<String> {
    let cmd = cmd.into();

    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';

    for c in cmd.chars() {
        if in_quotes {
            if c == quote_char {
                in_quotes = false;
                parts.push(current.clone());
                current.clear();
            } else {
                current.push(c);
            }
        } else {
            if c.is_whitespace() {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            } else if c == '\'' || c == '"' || c == '`' {
                in_quotes = true;
                quote_char = c;
            } else {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }
    parts
}
