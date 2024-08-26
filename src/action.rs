use std::net::Ipv4Addr;

use serde::{Serialize, Deserialize};
use sled::Tree;


pub trait ActionExt {
    fn run(&self, data: Tree);
}


#[derive(Serialize, Deserialize)]
pub struct TcpRequestAction {
    ip: Ipv4Addr,
    port: u16,
    payload: String, // formatting?
}


impl ActionExt for TcpRequestAction {
    fn run(&self, data: Tree) {
        // TODO
    }
}


#[derive(Serialize, Deserialize)]
pub enum Action {
    NoOp,
    TcpRequest(TcpRequestAction),
}


impl ActionExt for Action {
    fn run(&self, data: Tree) {
        match self {
            Self::NoOp => (),
            Self::TcpRequest(r) => { r.run(data); }
        }
    }
}
