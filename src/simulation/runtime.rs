// mod clock;
// mod file_system;
// mod scheduler;
//
// use std::collections::VecDeque;
//
// use crate::simulation::runtime::{clock::Clock, scheduler::Scheduler};
//
// type TimerId = u64;
// type ReplicaId = u64;
//
// struct Runtime<S: Scheduler> {
//     scheduler: S,
//     context: Context,
// }
//
// struct Context {
//     clock: Clock,
//     ready: VecDeque<ReplicaId>,
//     timers: Vec<TimerId>, // This will be the wakers
//     file_system:
// }
//
// enum State {
//     Running,
//     Done,
// }
//
// enum StepError {
//     InvalidAction,
// }
//
// pub enum Action {
//     Start(ReplicaId),
//     Wake(TimerId),
//     Done,
// }
//
// impl<S: Scheduler, FS: FileSystem> Runtime<S, FS> {
//     pub fn run(&mut self) -> Result<(), StepError> {
//         while let State::Running = self.step()? {}
//         Ok(())
//     }
//     pub fn step(&mut self) -> Result<State, StepError> {
//         let action = self.scheduler.decide(&self.context);
//         match self.is_valid(&action) {
//             true => self.execute(action),
//             false => Err(StepError::InvalidAction),
//         }
//     }
//
//     fn execute(&mut self, action: Action) -> Result<State, StepError> {
//         unimplemented!()
//     }
//
//     fn is_valid(&self, action: &Action) -> bool {
//         unimplemented!()
//     }
// }
