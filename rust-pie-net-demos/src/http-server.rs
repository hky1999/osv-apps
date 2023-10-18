#[cfg(target_os = "hermit")]
use hermit_sys as _;

use std::io;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream, ToSocketAddrs};
use std::thread;

const LOCAL_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
const LOCAL_PORT: u16 = 4444;

macro_rules! header {
	() => {
		"\
HTTP/1.1 200 OK\r\n\
Content-Type: text/html\r\n\
Content-Length: {}\r\n\
Connection: close\r\n\
\r\n\
{}"
	};
}

// const CONTENT: &str = r#"<html>
// <head>
//   <title>Hello world from Unishyper! 💙</title>
// </head>
// <body>
//   <center>
//     <h1>Hello, <a href="https://gitee.com/unishyper">Unishyper Unikernel</a></h1>
//   </center>
//   <hr>
//   <center>
//     <i>Powered by <a href="https://gitee.com/unishyper/unishyper/tree/master/examples/net_demos/src/http-server.rs">Unishyper Http server demo</a> v0.1.0</i>
//   </center>
// </body>
// </html>
// "#;

const CONTENT: &str = r#"hello"#;

fn http_server(mut stream: TcpStream) -> io::Result<()> {
	let mut buf = [0u8; 1024];
	stream.read(&mut buf)?;

	let reponse = format!(header!(), CONTENT.len(), CONTENT);
	stream.write_all(reponse.as_bytes())?;

	Ok(())
}

fn accept_loop() -> io::Result<usize> {
	let addr = (LOCAL_IP, LOCAL_PORT).to_socket_addrs()?.next().unwrap();
	let listener = TcpListener::bind(addr)?;
	println!(
		"listen on: http://{}/ on polling",
		listener.local_addr().unwrap()
	);

	let mut i = 0;
	loop {
		match listener.accept() {
			Ok((stream, _addr)) => {
				// println!("new client {}: {}", i, addr);
				thread::spawn(move || match http_server(stream) {
					Err(e) => println!("client connection error: {:?}", e),
					// Ok(()) => println!("client {} closed successfully", i),
					Ok(()) => {}
				});
			}
			Err(e) => return Err(e),
		}
		i += 1;
	}
}

fn main() {
	println!("Unishyper Http server demo");
	accept_loop().expect("Http server failed");
}
