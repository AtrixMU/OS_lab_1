#[derive(Debug, Clone)]
pub struct Resource {
    resource_type: usize,
    message: String,
    recipient: Option<usize>
}

impl Resource {
    pub fn new(resource_type: usize) -> Resource {
        Resource {
            resource_type: resource_type,
            message: String::new(),
            recipient: None,
        }
    }
    pub fn get_type(&self) -> usize {
        self.resource_type
    }
    pub fn get_msg(&self) -> String {
        self.message.clone()
    }
    pub fn get_recipient(&self) -> Option<usize> {
        self.recipient
    }
    pub fn set_msg(&mut self, msg: String) {
        self.message = msg;
    }
    pub fn set_recipient(&mut self, rec: usize) {
        self.recipient = Some(rec);
    }
}