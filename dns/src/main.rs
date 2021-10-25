use std::{
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use clap::{App, Arg};
use trust_dns::{
    op::{Message, MessageType, OpCode, Query},
    rr::{Name, RecordType},
    serialize::binary::{BinEncodable, BinEncoder},
};

fn main() {
    let app = App::new("resolve")
        .about("A simple DNS resolver")
        .arg(
            Arg::with_name("dns-server")
                .short("s")
                .default_value("1.1.1.1"),
        )
        .arg(Arg::with_name("domain-name").required(true))
        .get_matches();

    let domain_name = app.value_of("domain-name").unwrap();
    let domain_name = Name::from_ascii(&domain_name).unwrap();

    let dns_server = app.value_of("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server)
        .parse()
        .expect("invalid address");

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: Vec<u8> = vec![0; 512];

    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0").expect("cannot bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let _amt = localhost
        .send_to(&request_as_bytes, dns_server)
        .expect("socket misconfigured");

    let (_amt, _remote) = localhost
        .recv_from(&mut response_as_bytes)
        .expect("timeout reached");

    let dns_message = Message::from_vec(&response_as_bytes).expect("unable to parse response");

    dns_message
        .answers()
        .iter()
        .filter(|answer| answer.record_type() == RecordType::A)
        .map(|answer| answer.rdata())
        .map(|rdata| rdata.to_ip_addr().expect("invalid IP address received"))
        .for_each(|ip| println!("{}", ip.to_string()));
}
