// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// server interface

pub trait MediaExchangeService {
    fn add_media_session(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::AddSessionRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::AddSessionResponse>) -> ::grpc::Result<()>;

    fn update_media_session(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::UpdateSessionRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::UpdateSessionResponse>) -> ::grpc::Result<()>;

    fn delete_media_session(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::DeleteSessionRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::DeleteSessionResponse>) -> ::grpc::Result<()>;

    fn add_transfer_rule(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::AddTransferRuleRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::AddTransferRuleResponse>) -> ::grpc::Result<()>;

    fn update_transfer_rule(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::UpdateTransferRuleRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::UpdateTransferRuleResponse>) -> ::grpc::Result<()>;

    fn delete_transfer_rule(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::DeleteTransferRuleRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::DeleteTransferRuleResponse>) -> ::grpc::Result<()>;

    fn query_media_session(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::QuerySessionRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::QuerySessionResponse>) -> ::grpc::Result<()>;

    fn query_transfer_rule(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::QueryTransferRuleRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::QueryTransferRuleResponse>) -> ::grpc::Result<()>;

    fn add_media_channel(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::AddChannelRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::AddChannelResponse>) -> ::grpc::Result<()>;

    fn update_media_channel(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::UpdateMediaChannelRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::UpdateMediaChannelResponse>) -> ::grpc::Result<()>;

    fn delete_media_channel(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::DeleteMediaChannelRequst>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::DeleteMediaChannelResponse>) -> ::grpc::Result<()>;

    fn set_transfer_rule(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::SetTransferRuleRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::Result>) -> ::grpc::Result<()>;

    fn query_channel(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::QueryChannelRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::QueryChannelResponse>) -> ::grpc::Result<()>;

    fn query_src_by_consumer(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::QuerySrcByConsumerRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::QuerySrcByConsumerResponse>) -> ::grpc::Result<()>;

    fn query_consumers_by_src(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::QueryConsumersBySrcRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::QueryConsumersBySrcResponse>) -> ::grpc::Result<()>;

    fn query_all_relations(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::MediaTransfer::QueryAllRelationsRequest>, resp: ::grpc::ServerResponseUnarySink<super::MediaTransfer::QueryAllRelationsResponse>) -> ::grpc::Result<()>;
}

// client

pub struct MediaExchangeServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for MediaExchangeServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        MediaExchangeServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl MediaExchangeServiceClient {
    pub fn add_media_session(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::AddSessionRequest) -> ::grpc::SingleResponse<super::MediaTransfer::AddSessionResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/AddMediaSession"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn update_media_session(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::UpdateSessionRequest) -> ::grpc::SingleResponse<super::MediaTransfer::UpdateSessionResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/UpdateMediaSession"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn delete_media_session(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::DeleteSessionRequest) -> ::grpc::SingleResponse<super::MediaTransfer::DeleteSessionResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/DeleteMediaSession"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn add_transfer_rule(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::AddTransferRuleRequest) -> ::grpc::SingleResponse<super::MediaTransfer::AddTransferRuleResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/AddTransferRule"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn update_transfer_rule(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::UpdateTransferRuleRequest) -> ::grpc::SingleResponse<super::MediaTransfer::UpdateTransferRuleResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/UpdateTransferRule"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn delete_transfer_rule(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::DeleteTransferRuleRequest) -> ::grpc::SingleResponse<super::MediaTransfer::DeleteTransferRuleResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/DeleteTransferRule"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn query_media_session(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::QuerySessionRequest) -> ::grpc::SingleResponse<super::MediaTransfer::QuerySessionResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryMediaSession"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn query_transfer_rule(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::QueryTransferRuleRequest) -> ::grpc::SingleResponse<super::MediaTransfer::QueryTransferRuleResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryTransferRule"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn add_media_channel(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::AddChannelRequest) -> ::grpc::SingleResponse<super::MediaTransfer::AddChannelResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/AddMediaChannel"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn update_media_channel(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::UpdateMediaChannelRequest) -> ::grpc::SingleResponse<super::MediaTransfer::UpdateMediaChannelResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/UpdateMediaChannel"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn delete_media_channel(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::DeleteMediaChannelRequst) -> ::grpc::SingleResponse<super::MediaTransfer::DeleteMediaChannelResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/DeleteMediaChannel"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn set_transfer_rule(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::SetTransferRuleRequest) -> ::grpc::SingleResponse<super::MediaTransfer::Result> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/SetTransferRule"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn query_channel(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::QueryChannelRequest) -> ::grpc::SingleResponse<super::MediaTransfer::QueryChannelResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryChannel"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn query_src_by_consumer(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::QuerySrcByConsumerRequest) -> ::grpc::SingleResponse<super::MediaTransfer::QuerySrcByConsumerResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QuerySrcByConsumer"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn query_consumers_by_src(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::QueryConsumersBySrcRequest) -> ::grpc::SingleResponse<super::MediaTransfer::QueryConsumersBySrcResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryConsumersBySrc"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn query_all_relations(&self, o: ::grpc::RequestOptions, req: super::MediaTransfer::QueryAllRelationsRequest) -> ::grpc::SingleResponse<super::MediaTransfer::QueryAllRelationsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryAllRelations"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct MediaExchangeServiceServer;


impl MediaExchangeServiceServer {
    pub fn new_service_def<H : MediaExchangeService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/exchangeclient.MediaExchangeService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/AddMediaSession"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).add_media_session(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/UpdateMediaSession"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).update_media_session(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/DeleteMediaSession"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).delete_media_session(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/AddTransferRule"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).add_transfer_rule(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/UpdateTransferRule"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).update_transfer_rule(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/DeleteTransferRule"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).delete_transfer_rule(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryMediaSession"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).query_media_session(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryTransferRule"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).query_transfer_rule(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/AddMediaChannel"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).add_media_channel(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/UpdateMediaChannel"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).update_media_channel(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/DeleteMediaChannel"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).delete_media_channel(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/SetTransferRule"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).set_transfer_rule(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryChannel"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).query_channel(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QuerySrcByConsumer"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).query_src_by_consumer(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryConsumersBySrc"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).query_consumers_by_src(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/exchangeclient.MediaExchangeService/QueryAllRelations"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).query_all_relations(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
