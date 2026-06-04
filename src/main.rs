use dfs::{
    abstraction::{environment::Environment, file_system::stub::FileSystem},
    database::Replica,
};

fn main() {
    let fs = FileSystem {};
    let env = Environment::new(fs);
    let _replica = Replica::new(env);
}
