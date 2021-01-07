use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn write_head_and_bytes(mut stream: &TcpStream, data: &[u8]) -> io::Result<()> {
    // 存放头
    let buffer = (data.len() as u32).to_be_bytes();

    // 写入头
    stream.write_all(&buffer)?;

    // 写入body
    stream.write_all(data)?;

    Ok(())
}

pub fn read_head_and_bytes(mut stream: &TcpStream) -> io::Result<Vec<u8>> {
    let mut buffer = [0u8; 4];
    // 读取4个字节定长数据
    stream.read_exact(&mut buffer[..])?;

    // 计算buffer长度
    let size = u32::from_be_bytes(buffer);

    // 数据body
    let mut payload = vec![0; size as usize];

    // 读取数据body
    stream.read_exact(&mut payload[..])?;

    Ok(payload)
}
