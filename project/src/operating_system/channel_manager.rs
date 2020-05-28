use super::process::Process;
use crate::consts::*;
use super::resource::Resource;

pub struct ChannelManager {
    source_id : usize,
    destination_id usize,
    resource: Resource


}

impl ChannelManager {

    pub fn new(source: usize, destination: usize, resource: Resource) -> ChannelManager {

        self.source_id =  source;
        self.destination = destination;
        self.resource = resource;
    }
}