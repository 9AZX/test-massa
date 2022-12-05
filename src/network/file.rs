use std::{fs, io};
use crate::network::peer::{Peer};

pub struct PeersFileController {
    file_path: String
}

impl PeersFileController {
    pub fn new(file: &str) -> PeersFileController {
        PeersFileController { file_path: file.to_string() }
    }

    pub fn read_file(&self) -> Vec<Peer> {
        let json = fs::read_to_string(&self.file_path)
            .expect("Unable to read peers json file");

        let data: Vec<String> = serde_json::from_str(&json)
            .expect("JSON does not have correct format.");

        data.iter().map(Peer::new).collect()
    }

    pub fn write_file(&self, peers: &Vec<Peer>) -> Result<(), io::Error> {
        let ips: Vec<String> = peers.iter().map(|peer| peer.ip().to_string()).collect();

        let json = serde_json::to_string(&ips)?;

        fs::write(&self.file_path, &json)
    }
}