#[derive(Debug)]
pub struct Workspace {
    id: i32,
    name: String,
}

impl Workspace {
    pub fn new(id: i32, name: String) -> Self {
        Workspace { id, name }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
