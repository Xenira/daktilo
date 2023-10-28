#[cfg(feature = "client_proto")]
pub mod client_proto {
    tonic::include_proto!("daktilo");
}

#[cfg(feature = "server_proto")]
pub mod server_proto {
    tonic::include_proto!("daktilo");
}

#[cfg(feature = "server")]
pub mod server;
