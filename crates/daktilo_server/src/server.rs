use daktilo_common::event::DaktiloEvent;
use tokio::sync::mpsc::UnboundedSender;
use tonic::{transport::Server, Request, Response};

use crate::server_proto::{
    daktilo_server::{Daktilo, DaktiloServer},
    ActivateProfileRequest, CallResponse, DeactivateProfileRequest, ReportCursorMovementRequest,
    ReportLineLengthRequest,
};

/// Server configuration.
pub mod config;

#[derive(Debug)]
struct DaktiloService {
    sender: UnboundedSender<DaktiloEvent>,
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
            let _ = self.sender.send(DaktiloEvent::ColumnChange(
                request.get_ref().column_number as usize,
            ));
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
        tracing::info!("line length: {:?}", request);

        Ok(Response::new(CallResponse {
            success: true,
            error_message: None,
        }))
    }

    async fn activate_profile(
        &self,
        request: Request<ActivateProfileRequest>,
    ) -> Result<Response<CallResponse>, tonic::Status> {
        tracing::info!("activate profile: {:?}", request);

        Ok(Response::new(CallResponse {
            success: true,
            error_message: None,
        }))
    }

    async fn deactivate_profile(
        &self,
        request: Request<DeactivateProfileRequest>,
    ) -> Result<Response<CallResponse>, tonic::Status> {
        tracing::info!("deactivate profile: {:?}", request);

        Ok(Response::new(CallResponse {
            success: true,
            error_message: None,
        }))
    }
}

pub(crate) async fn start_api_server(
    sender: UnboundedSender<DaktiloEvent>,
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
