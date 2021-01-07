use std::io;
use std::net::{TcpListener, TcpStream};
use structopt::StructOpt;
use tcp_client_and_server::utils::*;

#[derive(StructOpt, Debug)]
struct Server {
    /// server address
    #[structopt(short, long)]
    server_addr: String,
}

fn work_conn(stream: TcpStream) -> io::Result<()> {
    // 1.读取payload
    let mut payload = read_head_and_bytes(&stream)?;

    // 2.追加一些数据，然后返回
    payload.append(&mut "#Server read".as_bytes().to_vec());

    // 3.写入响应
    write_head_and_bytes(&stream, &payload)?;
    Ok(())
}

impl Server {
    fn main_loop(&self) {
        // 创建listen
        let listener = TcpListener::bind(&self.server_addr).unwrap();

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(e) => {
                    /* connection failed */
                    println!("{:#?}", e);
                    continue;
                }
            };

            work_conn(stream).unwrap();
        }
    }
}

fn main() {
    let s = Server::from_args();

    s.main_loop();
}
