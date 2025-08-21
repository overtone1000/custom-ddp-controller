use std::{collections::BTreeMap, net::SocketAddr, ops::Index, process::ExitStatus};

use hyper::{body::Incoming, Request, Response};
use hyper_services::{
    commons::{HandlerError, HandlerResult},
    response_building::{bad_request, full_to_boxed_body},
    service::{stateful_service::StatefulHandler, stateless_service::StatelessHandler},
};
use tokio::process::Command;

use crate::pixels::pixelstrip::PixelStrip;

#[derive(Clone)]
pub struct LedCommandHandler {
    pixel_strips: BTreeMap<SocketAddr, PixelStrip>,
}

impl LedCommandHandler {
    pub fn new() -> LedCommandHandler {
        LedCommandHandler {
            pixel_strips: BTreeMap::new(),
        }
    }
}

impl StatefulHandler for LedCommandHandler {
    async fn handle_request(self: Self, request: Request<Incoming>) -> HandlerResult {
        let method = request.method().clone();
        let path = request.uri().path().to_string();

        //return Ok(not_found);
        //return Ok(bad_request);

        return Ok(Response::new(full_to_boxed_body("Ok")));
    }
}
