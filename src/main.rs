use std::io::{*};//引入标准库io

//标准库有封装好的TcpListener 和  TcpStream
use std::net::{TcpListener,TcpStream};
use std::str;


//rust里有两种库调用 1.标准库 use std::xx  . 2.外部依赖库 ,使用cargo调用.[在cargo.toml配置]

/**
 *  ===客户端===
 * 1.建立链接
 * 2.将客户端输入内容写入流中
 */
fn main() -> std::io::Result<()> {  //std::io::Result   与 std::result::Result
     let mut stream = TcpStream::connect("127.0.0.1:80")?; //链接到指定ip

     //stream.write(&[1])?; // "?" 错误处理语法糖, 有错误会返回 std::io::Error
     //stream.read(&mut [0; 128])?;
     println!("please input :");
     let mut input = String::new();
     std::io::stdin().read_line(&mut input)?;//将键盘输入的内容放到input中
     stream.write(input.as_bytes())?;//将input内容写入流中
     
    //打印输入内容
     println!("input content {}",input);

    


     Ok(())//成功
 }
