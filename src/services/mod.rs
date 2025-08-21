use std::{collections::BTreeMap, net::SocketAddr};

use hyper::{body::Incoming, Request, Response};
use hyper_services::{
    commons::{HandlerResult},
    response_building::{full_to_boxed_body},
    service::{stateful_service::StatefulHandler},
};

use crate::pixels::{pixelstrip::PixelStrip, pixelstripmanager::PixelStripManager};

#[derive(Clone)]
pub struct LedCommandHandler {
    pixel_strip_manager: PixelStripManager,
}

impl  LedCommandHandler {
    pub fn new(pixel_strip_manager:PixelStripManager) -> LedCommandHandler {
        LedCommandHandler {
            pixel_strip_manager,
        }
    }
}

impl  StatefulHandler for LedCommandHandler {
    async fn handle_request(self: Self, request: Request<Incoming>) -> HandlerResult {
        let method = request.method().clone();
        let path = request.uri().path().to_string();

        //return Ok(not_found);
        //return Ok(bad_request);

        return Ok(Response::new(full_to_boxed_body("Ok")));
    }
}
