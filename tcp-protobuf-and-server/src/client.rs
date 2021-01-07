use protobuf::Message;
use std::net::TcpStream;
use structopt::StructOpt;
use tcp_protobuf_and_server::protos::small_model::*;
use tcp_protobuf_and_server::utils::*;

#[derive(StructOpt, Debug)]
struct Client {
    /// tcp server address
    #[structopt(short, long)]
    addr: String,

    /// data
    #[structopt(short, long)]
    data: String,

    /// hotword
    #[structopt(short, long)]
    hot_world: bool,

    /// grammar
    #[structopt(short, long)]
    grammar: bool,

    /// small model
    #[structopt(short, long)]
    small_model: bool,
}

fn main() {
    // 解析命令行
    let c = Client::from_args();

    let t: String;

    if c.hot_world {
        t = "hot_world".to_string();
    } else if c.grammar {
        t = "grammar".to_string();
    } else if c.small_model {
        t = "small_model".to_string();
    } else {
        panic!("hotword(false), grammar(false), small-model(false)");
    }

    let mut req = HotWordRequest::new();
    req.model_text = c.data;
    req.model_type = t;

    // 0.protobuf结构体转成bytes
    let out_req = req.write_to_bytes().unwrap();

    // 1.连接服务端
    let stream = TcpStream::connect(c.addr).unwrap();

    // 2.写入数据，并加4个字节的头
    write_head_and_bytes(&stream, &out_req).unwrap();

    // 3.读取TLV数据，提取V
    let payload = read_head_and_bytes(&stream).unwrap();

    // 4.解析payload 到protobuf结构体里面
    let rsp = protobuf::parse_from_bytes::<HotWordResponse>(&payload).unwrap();

    // 5.读取rsp结构体里面数据
    let gen_model = rsp.get_gen_model();

    // 6.写入到终端
    println!("payload:{}", std::str::from_utf8(&gen_model).unwrap());
}
