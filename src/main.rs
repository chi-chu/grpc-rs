#[allow(unused)]
#[allow(dead_code)]
#[macro_use]
mod macros;

use chrono::Local;
use client::args;
use client::log;
use std::time::Duration;
use tokio;
use tonic;
use tonic::codegen::{Body, Bytes, StdError};
use proto::credit_grpc;

const ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    args::init_cmd_line();
	let cli = credit_grpc::credit_client::CreditClient::connect(format!("http://{}", ADDRESS))
	    .await
	    .expect("init client failed");
	grpc_credit(cli).await;
	return;
}

async fn grpc_credit<T>(mut cli: credit_grpc::credit_client::CreditClient<T>)
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data=Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    let request = tonic::Request::new(credit_grpc::QueryCreditReq {
        uins: vec![123, 333].into(),
        source: "MC".into(),
    });
    let res = cli.query_credit(request).await.unwrap();
    println!("Grpc ans {:?}", res.into_inner().data);
}
