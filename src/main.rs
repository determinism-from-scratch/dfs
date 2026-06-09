use dfs::simulation::runtime::{
    Runtime,
    fault_injector::perfect::FaultInjector,
    replica::environment::{Environment, file_system::memory::FileSystem},
    scheduler::fifo::Scheduler,
};

fn main() {
    let fs = FileSystem {};
    let scheduler = Scheduler::new();
    let fault_injector = FaultInjector {};

    let mut runtime: Runtime<Scheduler, FaultInjector, FileSystem> =
        Runtime::new(scheduler, fault_injector);

    let env = Environment::new(
        dfs::simulation::runtime::replica::environment::file_system::memory::FileSystem {},
    );
    runtime.spawn(env, || {
        println!("hello");
    });

    runtime.run();
}
