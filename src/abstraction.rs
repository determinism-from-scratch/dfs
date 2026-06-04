use crate::simulation::runtime::handles::{HANDLE, Request, Response};

pub mod environment;
pub mod file_system;

fn trap(req: Request) -> Response {
    HANDLE.with_borrow_mut(|h| {
        let handle = h.as_mut().expect("handle used before initialization");
        handle
            .request
            .send(req)
            .expect("runtime terminated before replicas");
    });

    let resp = HANDLE.with_borrow_mut(|handle| handle.as_mut().unwrap().response.recv());
    resp.expect("runtime terminated before replicas")
}
