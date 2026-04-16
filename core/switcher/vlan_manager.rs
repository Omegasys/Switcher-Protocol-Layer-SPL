use std::collections::HashMap;

pub struct VlanManager {
    vlan_ports: HashMap<u16, Vec<u32>>,
}

impl VlanManager {
    pub fn new() -> Self {
        Self {
            vlan_ports: HashMap::new(),
        }
    }

    pub fn add_port(&mut self, vlan: u16, port: u32) {
        self.vlan_ports.entry(vlan).or_default().push(port);
    }

    pub fn is_allowed(&self, vlan: u16, port: u32) -> bool {
        match self.vlan_ports.get(&vlan) {
            Some(ports) => ports.contains(&port),
            None => true, // default allow if VLAN not defined
        }
    }

    pub fn remove_vlan(&mut self, vlan: u16) {
        self.vlan_ports.remove(&vlan);
    }
}
