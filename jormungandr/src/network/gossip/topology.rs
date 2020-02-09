use crate::network::gossip::layer::Layer;
use crate::network::gossip::profile::{Profile, ProfileSet};
use crate::network::p2p::Gossips;
use poldercast::NodeProfile;

pub struct Topology {
    layers: Vec<Box<dyn Layer>>,
    view: ProfileSet,
}

impl Default for Topology {
    fn default() -> Self {
        Topology {
            layers: Vec::default(),
            view: ProfileSet::default(),
        }
    }
}

impl Topology {
    fn compute_gossips_for_node(&self, node: Profile) {}
}
