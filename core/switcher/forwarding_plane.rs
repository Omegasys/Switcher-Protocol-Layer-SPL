use crate::switcher::l2_switch_engine::{L2SwitchEngine, EthernetFrame};

pub struct ForwardingPlane {
    pub switch: L2SwitchEngine,
}

impl ForwardingPlane {
    pub fn new(switch: L2SwitchEngine) -> Self {
        Self { switch }
    }

    pub fn handle_packet(&mut self, frame: EthernetFrame, ingress_port: u32) {
        match self.switch.process_frame(frame, ingress_port) {
            Some(out_port) => {
                self.send(out_port);
            }
            None => {
                self.drop();
            }
        }
    }

    fn send(&self, port: u32) {
        println!("Forwarding packet to port {}", port);
    }

    fn drop(&self) {
        println!("Packet dropped");
    }
}
