use std::net::Ipv4Addr;

pub mod app;
pub mod event;
pub mod widgets;
pub mod config;
pub mod parse;

pub struct Hit {
    addr: Ipv4Addr,
    status: u64,
}
