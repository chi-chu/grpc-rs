use crate::args;
use crate::MediaTransfer::*;
use crate::MediaTransfer_grpc::*;
use crate::response;
use std::thread;
use chrono::Local;
use serde_json;

struct MediaTransferServer;

impl MediaExchangeService for MediaTransferServer {
	fn add_media_session(&self, 
        _o: grpc::ServerHandlerContext, 
        req: grpc::ServerRequestSingle<AddSessionRequest>, 
        resp: grpc::ServerResponseUnarySink<AddSessionResponse>) 
        -> grpc::Result<()> {
    	let mut r = AddSessionResponse::new();
        let sdp_type = req.message.get_local_sdp();
        let file = {
                if sdp_type.contains("sendonly") {
                    "DecLocalSDP.sdp"
                } else if sdp_type.contains("recvonly") {
                    "EncLocalSDP.sdp"
                } else {
                    ""
                }
            };
        let sdp = match response::get_response(file) {
                Ok(data) => data,
                Err(e) => {
                    error!("sdp file err: {:?}", e);
                    String::from("")
                }
            };
        r.set_local_sdp(sdp);
        let res = match response::get_response("AddMediaSessionRes.json") {
            Ok(data) => decode_result(data),
            Err(e) => {
                error!("AddMediaSessionRes.json unmarshal err: {:?}", e);
                Result::new()
            }
        };
        r.set_result(res);
    	resp.finish(r)
    }

    fn update_media_session(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<UpdateSessionRequest>, 
        resp: grpc::ServerResponseUnarySink<UpdateSessionResponse>) 
        -> grpc::Result<()> {
        let r = UpdateSessionResponse::new();
        resp.finish(r)
    }

    fn delete_media_session(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<DeleteSessionRequest>, 
        resp: grpc::ServerResponseUnarySink<DeleteSessionResponse>) 
        -> grpc::Result<()> {
        let r = DeleteSessionResponse::new();
        resp.finish(r)
    }

    fn add_transfer_rule(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<AddTransferRuleRequest>, 
        resp: grpc::ServerResponseUnarySink<AddTransferRuleResponse>) 
        -> grpc::Result<()> {
        let r = AddTransferRuleResponse::new();
        resp.finish(r)
    }

    fn update_transfer_rule(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<UpdateTransferRuleRequest>, 
        resp: grpc::ServerResponseUnarySink<UpdateTransferRuleResponse>) 
        -> grpc::Result<()> {
        let r = UpdateTransferRuleResponse::new();
        resp.finish(r)
    }

    fn delete_transfer_rule(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<DeleteTransferRuleRequest>, 
        resp: grpc::ServerResponseUnarySink<DeleteTransferRuleResponse>) 
        -> grpc::Result<()> {
        let r = DeleteTransferRuleResponse::new();
        resp.finish(r)
    }

    fn query_media_session(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<QuerySessionRequest>, 
        resp: grpc::ServerResponseUnarySink<QuerySessionResponse>) 
        -> grpc::Result<()> {
        let r = QuerySessionResponse::new();
        resp.finish(r)
    }

    fn query_transfer_rule(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<QueryTransferRuleRequest>, 
        resp: grpc::ServerResponseUnarySink<QueryTransferRuleResponse>) 
        -> grpc::Result<()> {
        let r = QueryTransferRuleResponse::new();
        resp.finish(r)
    }

    fn add_media_channel(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<AddChannelRequest>, 
        resp: grpc::ServerResponseUnarySink<AddChannelResponse>) 
        -> grpc::Result<()> {
        let r = AddChannelResponse::new();
        resp.finish(r)
    }

    fn update_media_channel(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<UpdateMediaChannelRequest>, 
        resp: grpc::ServerResponseUnarySink<UpdateMediaChannelResponse>) 
        -> grpc::Result<()> {
        let r = UpdateMediaChannelResponse::new();
        resp.finish(r)
    }

    fn delete_media_channel(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<DeleteMediaChannelRequst>, 
        resp: grpc::ServerResponseUnarySink<DeleteMediaChannelResponse>) 
        -> grpc::Result<()> {
        let r = DeleteMediaChannelResponse::new();
        resp.finish(r)
    }

    fn set_transfer_rule(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<SetTransferRuleRequest>, 
        resp: grpc::ServerResponseUnarySink<Result>) 
        -> grpc::Result<()> {
        let r = Result::new();
        resp.finish(r)
    }

    fn query_channel(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<QueryChannelRequest>, 
        resp: grpc::ServerResponseUnarySink<QueryChannelResponse>) 
        -> grpc::Result<()> {
        let r = QueryChannelResponse::new();
        resp.finish(r)
    }

    fn query_src_by_consumer(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<QuerySrcByConsumerRequest>, 
        resp: grpc::ServerResponseUnarySink<QuerySrcByConsumerResponse>) 
        -> grpc::Result<()> {
        let r = QuerySrcByConsumerResponse::new();
        resp.finish(r)
    }

    fn query_consumers_by_src(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<QueryConsumersBySrcRequest>, 
        resp: grpc::ServerResponseUnarySink<QueryConsumersBySrcResponse>) 
        -> grpc::Result<()> {
        let r = QueryConsumersBySrcResponse::new();
        resp.finish(r)
    }

    fn query_all_relations(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<QueryAllRelationsRequest>, 
        resp: grpc::ServerResponseUnarySink<QueryAllRelationsResponse>) 
        -> grpc::Result<()> {
        let r = QueryAllRelationsResponse::new();
        resp.finish(r)
    }

}


pub fn start() {
	let port = args::get_port();
    info!("Server start listen at {} ", port);
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port);
    let handler: MediaTransferServer = MediaTransferServer{};
    server.add_service(MediaExchangeServiceServer::new_service_def(handler));
    let _server = server.build().expect("Could not start server");
    loop {
        thread::park();
    }
}

pub fn decode_result(s: String) -> Result {
    let obj: serde_json::Value = serde_json::from_str(&s).unwrap();
    let mut r = Result::new();
    r.set_code(
        match obj["code"].as_i64() {
            Some(expr) => expr as i32,
            None => 0,
        });
    r.set_description(
        match obj["description"].as_str() {
            Some(expr) => expr.to_string(),
            None => "".to_string(),
        });
    r
}