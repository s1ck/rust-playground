use std::{error::Error, io::Write, net::TcpStream};

use reqwest;

fn get_request() -> Result<(), Box<dyn Error>> {
    let url = "http://www.github.com/";
    let mut response = reqwest::get(url)?;
    let content = response.text()?;

    println!("{}", content);

    Ok(())
}

fn get_request_manual() -> std::io::Result<()> {
    let host = "www.github.com:80";

    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    conn.write_all(b"Host: www.github.com")?;
    conn.write_all(b"\r\n\r\n")?;

    // noice
    std::io::copy(&mut conn, &mut std::io::stdout())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_request, get_request_manual};

    #[test]
    fn test_get_request() {
        get_request().unwrap();
    }

    #[test]
    fn test_get_request_manual() {
        get_request_manual().unwrap();
    }
}
