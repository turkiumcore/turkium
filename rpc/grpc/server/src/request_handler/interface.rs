use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use Turkium_grpc_core::{
    ops::TurkiumdPayloadOps,
    protowire::{TurkiumdRequest, TurkiumdResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type TurkiumdMethod = Method<ServerContext, Connection, TurkiumdRequest, TurkiumdResponse>;
pub type DynTurkiumdMethod = Arc<dyn MethodTrait<ServerContext, Connection, TurkiumdRequest, TurkiumdResponse>>;
pub type TurkiumdDropFn = DropFn<TurkiumdRequest, TurkiumdResponse>;
pub type TurkiumdRoutingPolicy = RoutingPolicy<TurkiumdRequest, TurkiumdResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`TurkiumdPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<TurkiumdPayloadOps, DynTurkiumdMethod>,
    method_not_implemented: DynTurkiumdMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, turkiumd_request: TurkiumdRequest| {
            Box::pin(async move {
                match turkiumd_request.payload {
                    Some(ref request) => Ok(TurkiumdResponse {
                        id: turkiumd_request.id,
                        payload: Some(
                            TurkiumdPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into()),
                        ),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: TurkiumdPayloadOps, method: TurkiumdMethod) {
        let method: DynTurkiumdMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: TurkiumdPayloadOps, method: TurkiumdMethod) {
        let method: DynTurkiumdMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: TurkiumdPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: TurkiumdRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, TurkiumdRequest, TurkiumdResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, TurkiumdRequest, TurkiumdResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &TurkiumdPayloadOps,
        connection: Connection,
        request: TurkiumdRequest,
    ) -> GrpcServerResult<TurkiumdResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &TurkiumdPayloadOps) -> DynTurkiumdMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
