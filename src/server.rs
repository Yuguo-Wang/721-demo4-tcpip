use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    // 绑定服务器的地址和端口号
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 接受来自客户端的连接
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("连接成功！");

                // 读取客户端发送的数据
                let mut buf = [0; 1024];
                match stream.read(&mut buf) {
                    Ok(_) => {
                        // 将接收到的数据打印到控制台
                        let received = String::from_utf8_lossy(&buf);
                        println!("{}", received);

                        // 发送回应数据
                        let response = "Hello from server!";
                        stream.write(response.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        println!("读取数据失败：{}", e);
                    }
                }
            }
            Err(e) => {
                println!("连接失败：{}", e);
            }
        }
    }
}
