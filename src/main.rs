/*
 * @Author: JakeWang 1875406398@qq.com
 * @Date: 2023-03-25 17:23:57
 * @LastEditors: JakeWang 1875406398@qq.com
 * @LastEditTime: 2023-03-26 15:49:01
 * @FilePath: \echo-server\src\main.rs
 * @Description:
 *
 * Copyright (c) 2023 by Chiyu, All Rights Reserved.
 */
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9900").expect("bind addr failed");

    for stream in server.incoming() {
        let stream = stream.unwrap();
        println!("accpet onew clinet");
        thread::spawn(|| {
            handle_tcp(stream);
        });
        println!("accpet end");
    }
}
fn handle_tcp(mut stream: TcpStream) {
    // 注意这里：必须使用已经创建好的空间，如果使用其他的类似vec::new
    //会报错是因为read不会自动扩充其空间
    let mut buff = vec![0u8; 1024];
    loop {
        match stream.read(&mut buff) {
            Ok(n) if n > 0 => {
                println!("recv one message");
                stream.write_all(&buff[..n]).unwrap();
            }
            Ok(_) => {
                println!("peer close");
                break;
            }
            Err(e) => {
                println!("read error: {}", e);
                break;
            }
        }
    }
    stream.shutdown(std::net::Shutdown::Both).unwrap();
    println!("exit pthread");
}
