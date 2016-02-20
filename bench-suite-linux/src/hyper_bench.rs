use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, Read, Write, Cursor};
use std::net::SocketAddr;
use std::time::Duration;

use hyper::net;
use hyper::header::{Header, HeaderFormat};
use hyper::error::Error;
use hyper::Result;

static README: &'static [u8] = include_bytes!("../Cargo.toml");

struct MockStream {
    read: Cursor<Vec<u8>>
}

impl MockStream {
    fn new() -> MockStream {
        let head = b"HTTP/1.1 200 OK\r\nServer: Mock\r\n\r\n";
        let mut res = head.to_vec();
        res.extend_from_slice(README);
        MockStream {
            read: Cursor::new(res)
        }
    }
}

impl Clone for MockStream {
    fn clone(&self) -> MockStream {
        MockStream {
            read: Cursor::new(self.read.get_ref().clone())
        }
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read.read(buf)
    }
}

impl Write for MockStream {
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        // we're mocking, what do we care.
        Ok(msg.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Foo;

impl Header for Foo {
    fn header_name() -> &'static str {
        "x-foo"
    }
    fn parse_header(_: &[Vec<u8>]) -> Result<Foo> {
        Err(Error::Header)
    }
}

impl HeaderFormat for Foo {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("Bar")
    }
}

impl net::NetworkStream for MockStream {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        Ok("127.0.0.1:1337".parse().unwrap())
    }
    fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        // can't time out
        Ok(())
    }
    fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        // can't time out
        Ok(())
    }
}

struct MockConnector;

impl net::NetworkConnector for MockConnector {
    type Stream = MockStream;
    fn connect(&self, _: &str, _: u16, _: &str) -> Result<MockStream> {
        Ok(MockStream::new())
    }
}

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("bench_mock_hyper".to_string()).or_insert(vec![]).push(bench_mock_hyper());
    }

    results
}

fn bench_mock_hyper() -> u64 {
    let url = "http://127.0.0.1:1337/";
    bench!(100_000, {
        use hyper::client::Request;
        use hyper::Get;
        use hyper::Url;
        let mut req = Request::with_connector(
            Get, Url::parse(url).unwrap(), &MockConnector
        ).unwrap();
        req.headers_mut().set(Foo);

        let mut s = String::new();
        req
            .start().unwrap()
            .send().unwrap()
            .read_to_string(&mut s).unwrap();
    })
}
