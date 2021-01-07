use protobuf::Message;
use std::io;
use std::net::{TcpListener, TcpStream};
use structopt::StructOpt;
use tcp_protobuf_and_server::protos::small_model::*;
use tcp_protobuf_and_server::utils::*;

#[derive(StructOpt, Debug)]
struct Server {
    /// server address
    #[structopt(short, long)]
    server_addr: String,
}

fn work_conn(stream: TcpStream) -> io::Result<()> {
    // 0.读取TLV数据，提取data
    let payload = read_head_and_bytes(&stream)?;
    // 1.解析protobuf数据到结构体
    let req = protobuf::parse_from_bytes::<HotWordRequest>(&payload)?;
    // 2.获取请求参数
    let model_text = req.get_model_text();

    // 3.追加一些数据，然后返回
    let rsp_model_text = format!(
        "#Server read <client data:{}> <client type:{}>",
        model_text,
        req.get_model_type()
    );

    // 4.声明响应
    let mut rsp = HotWordResponse::new();

    // 5.设置回写数据
    rsp.set_gen_model(rsp_model_text.as_bytes().to_vec());

    // 6.protobuf 结构体转成bytes
    let out_rsp = rsp.write_to_bytes().unwrap();

    // 7.写入客户端
    write_head_and_bytes(&stream, &out_rsp)?;
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
