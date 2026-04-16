use std::collections::HashMap;

pub struct MacTable {
    table: HashMap<String, u32>,
}

impl MacTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn learn(&mut self, mac: String, port: u32) {
        self.table.insert(mac, port);
    }

    pub fn lookup(&self, mac: &str) -> Option<u32> {
        self.table.get(mac).copied()
    }

    pub fn remove(&mut self, mac: &str) {
        self.table.remove(mac);
    }

    pub fn size(&self) -> usize {
        self.table.len()
    }
}
