use crate::{Path, PathExploration};

#[derive(Debug, Clone)]
pub struct DFSPathExploration {
    paths: Vec<Path>,
}

impl DFSPathExploration {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }
}

impl PathExploration for DFSPathExploration {
    fn save_path(&mut self, path: Path) {
        path.state.constraints.push();
        self.paths.push(path);
    }

    fn get_path(&mut self) -> Option<Path> {
        match self.paths.pop() {
            Some(path) => {
                path.state.constraints.pop();
                Some(path)
            }
            None => None,
        }
    }
}
