use std::path::PathBuf;

use rhai;

use crate::rhai_integration::DataSourceBuilder;

pub struct Executor {
    rhai_engine: rhai::Engine,
    rhai_script: PathBuf,
}

fn executor_engine() -> rhai::Engine {
    let mut engine = rhai::Engine::new();
    // ^ TODO: Switch?
    // let mut engine = rhai::Engine::new_raw();
    engine.build_type::<DataSourceBuilder>();

    engine
}

impl Executor {
    pub fn new<P: Clone + Into<PathBuf>>(script_path: P) -> Executor {
        Executor {
            rhai_engine: executor_engine(),
            rhai_script: script_path.into(),
        }
    }

    pub fn run(&self) -> () {
        self.rhai_engine
            .run_file(self.rhai_script.clone())
            .expect("Error executing script")
    }
}
