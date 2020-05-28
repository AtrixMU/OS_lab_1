use super::resource::Resource;

#[derive(Debug)]
pub struct ResourceList {
    free: Vec<Resource>
}

impl ResourceList {
    pub fn new() -> ResourceList {
        ResourceList {
            free: Vec::new(),
        }
    }

    pub fn add(&mut self, resource: Resource) {
        self.free.push(resource);

    }

    pub fn take(&mut self, resource_type: usize ) ->  Option<Resource> {
        let mut res_index = 0;
        let mut found = false;
        for (index, res) in self.free.iter().enumerate() {
            if res.get_type() == resource_type && res.get_recipient().is_none() {
              res_index = index;
              found = true;
              break;
            }
        }
        if !found {
            return None;
        }
        Some(self.free.remove(res_index))
    }

    pub fn take_specific(&mut self, resource_type: usize, recipient: usize ) -> Option<Resource> {
        let mut res_index = 0;
        let mut found = false;
        for (index, res) in self.free.iter().enumerate() {
            if res.get_type() == resource_type && res.get_recipient() == Some(recipient) {
                res_index = index;
                found = true;
                break;
            }
        }
        if !found {
            return None;
        }
        Some(self.free.remove(res_index))
    }
}