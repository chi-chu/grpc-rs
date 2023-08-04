# Grpc-rs

l love golang,  
l love python,  
l love php,  
and also rust..  

this project is a demo for using grpc by rust.  

虽然这是个小demo，但是包含了自定义宏，静态变量，文件操作，以及grpc实例，  
包含rust编译前置build，包含文件引入 include， json反序列化。  
算是麻雀虽小，五脏俱全吧，希望能对你有所帮助，能一起学习rust。

### run  
```shell script
cargo run
```

#### build.rs
```
#![allow(unused)]
extern crate tonic_build;
use std::{fs, thread};
use std::io::{Read, Write};
use std::time;
use std::process::{Command, ExitStatus};
use std::io::Result;
use std::env;

fn main() {
    println!("now exec dir is {:?}", env::current_dir().unwrap().display());
    let rs = get_proto();
    if !rs {
        panic!("pull proto failed!!");
    }
    gen_and_rm();
}


fn get_proto() -> bool {
    let f = fs::File::open("./proto/credit.proto");
    match f {
        Ok(file) => {
            //TODO run git checkout .  && git pull
            return true;
        }
        _ => {}
    }
    let state = Command::new("git").arg("clone").
        arg("git@abc.gasj.skafha.git").
        status().unwrap();
    if state.success() {
        return true;
    }
    return false;
}

fn rm_trpc() -> Result<()> {
    let mut data = fs::read_to_string("./backend-partner-credit/credit.proto")?;
    let newpr= data.replace("import \"rpc.proto\";", "");
    fs::write("./src/credit.proto",newpr.as_bytes())
}

#[cfg(not(target_os= "windows"))]
fn gen_and_rm(){
    rm_trpc().expect("write err");
    tonic_build::configure()
        .build_server(false)
        .out_dir("./src")
        .compile(
            &["src/folder.proto"], &["src"],
        ).unwrap();
    Command::new("rm").
        arg(" -rf").
        arg("./folder").
        status();
}
```


```
#[allow(unused)]
#[macro_use]
mod macros;

use tokio;
use tonic;
use serde_json;
use rspkg::credit;
use tonic::codegen::{StdError, Body, Bytes};

#[tokio::main]
async fn main() {
    let mut cli = credit::credit_client::CreditClient::connect("http://127.0.0.1:8000").
        await.expect("init client failed");
    test_credit(cli).await;
}

async fn test_credit<T>(mut cli: credit::credit_client::CreditClient<T>)
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data=Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    let request = tonic::Request::new(credit::QueryCreditReq {
        uins: vec![123, 333].into(),
        source: "MC".into(),
    });
    let res = cli.query_credit(request).await.unwrap();
    println!("RESPONSE={:?}", res);
}
```
