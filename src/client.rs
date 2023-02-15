use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    // 连接服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    // 发送数据到服务器
    let data = "Hello from client!";
    stream.write(data.as_bytes()).unwrap();

    // 接收来自服务器的回应数据
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(_) => {
            // 将接收到的数据打印到控制台
            let received = String::from_utf8_lossy(&buf);
            println!("{}", received);
        }
        Err(e) => {
            println!("读取数据失败：{}", e);
        }
    }
}
