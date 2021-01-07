use std::io;
use std::net::TcpStream;
use structopt::StructOpt;
use tcp_client_and_server::utils::*;

#[derive(StructOpt, Debug)]
struct Client {
    /// tcp server address
    #[structopt(short, long)]
    addr: String,

    /// data
    #[structopt(short, long)]
    data: String,
}

fn main() -> io::Result<()> {
    // 解析命令行
    let c = Client::from_args();

    // 1.连接服务端
    let stream = TcpStream::connect(c.addr).unwrap();

    // 2.写入请求
    write_head_and_bytes(&stream, c.data.as_bytes())?;

    // 3.读取rsp数据
    let payload = read_head_and_bytes(&stream)?;

    // 4.返回终端
    println!("{}", std::str::from_utf8(&payload).unwrap());

    Ok(())
}
