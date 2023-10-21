use std::time::SystemTime;

use rdev::{Event, EventType};
use tokio::sync::mpsc::UnboundedSender;
use tonic::{transport::Server, Request, Response};

use self::daktilo_api::{
    daktilo_server::{Daktilo, DaktiloServer},
    CallResponse, ReportCursorMovementRequest, ReportLineLengthRequest,
};

mod daktilo_api {
    tonic::include_proto!("daktilo");
}

#[derive(Debug)]
struct DaktiloService {
    sender: UnboundedSender<Event>,
}

#[tonic::async_trait]
impl Daktilo for DaktiloService {
    async fn report_cursor_movement(
        &self,
        request: Request<ReportCursorMovementRequest>,
    ) -> Result<Response<CallResponse>, tonic::Status> {
        tracing::info!("cursor movement: {:?}", request);

        if request.get_ref().column_number > 0 && request.get_ref().column_number % 20 == 0 {
            tracing::info!("column number: {}", request.get_ref().column_number);
            let _ = self.sender.send(Event {
                time: SystemTime::now(),
                name: None,
                event_type: EventType::ButtonPress(rdev::Button::Unknown(255)),
            });
        }

        Ok(Response::new(CallResponse {
            success: true,
            error_message: None,
        }))
    }

    async fn report_line_length(
        &self,
        request: Request<ReportLineLengthRequest>,
    ) -> Result<Response<CallResponse>, tonic::Status> {
        Ok(Response::new(CallResponse {
            success: true,
            error_message: None,
        }))
    }
}

pub(crate) async fn start_api_server(
    sender: UnboundedSender<Event>,
) -> Result<(), tonic::transport::Error> {
    let addr = "[::1]:50051".parse().unwrap();
    let daktilo_service = DaktiloService { sender };

    tracing::info!("starting api server on {}", addr);

    let server = Server::builder()
        .add_service(DaktiloServer::new(daktilo_service))
        .serve(addr)
        .await;

    tracing::info!("server stopped: {:?}", server);
    server
}
