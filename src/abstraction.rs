// pub mod environment;
// pub mod file_system;

// thread_local! {
//
//      static A: RefCell<Box<dyn file_system::FileSystem<File = file_system::stub::File>>> =
//     RefCell::new(Box::new(FileSystem {}));
// }
//
// fn open() -> Box<dyn File> {
//     let a = A.with_borrow_mut(|a| a.as_mut().unwrap().open("", OpenMode::Read));
//     Box::new(a.unwrap())
//     a.overider(Box::new(Files))
// }

// fn trap(req: Request) -> Response {
//     HANDLE.with_borrow_mut(|h| {
//         let handle = h.as_mut().expect("handle used before initialization");
//         handle
//             .request
//             .send(req)
//             .expect("runtime terminated before replicas");
//     });
//
//     let resp = HANDLE.with_borrow_mut(|handle| handle.as_mut().unwrap().response.recv());
//     resp.expect("runtime terminated before replicas")
// }
