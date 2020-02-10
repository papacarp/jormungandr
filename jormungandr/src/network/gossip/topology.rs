use crate::network::gossip::layer::Layer;
use crate::network::gossip::profile::{Profile, ProfileSet};
use crate::network::gossip::vicinity::Vicinity;
use crate::network::p2p::Gossips;

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
    fn new(view: ProfileSet) -> Self {
        // Initialise PolderCast layers (in bottom-first order).
        let mut layers: Vec<Box<dyn Layer>> = Vec::default();
        //        layers.push(Box::new(Cyclon::default()));
        layers.push(Box::new(Vicinity::default()));
        //        layers.push(Box::new(Rings::default()));
        Topology { layers, view }
    }

    fn accept_profiles(
        &mut self,
        identity: &mut Profile,
        origin: &Profile,
        origin_profiles: &ProfileSet,
    ) {
        // TODO: write lock?
        for layer in &mut self.layers {
            // Create a new merged view of the origin's profiles and our current view of our
            // profiles for each successive iteration of this loop. It may even be valid and
            // beneficial to perform each iteration in parallel.
            let merged_profiles = self
                .view
                .union(origin_profiles)
                .into_iter()
                .cloned()
                .collect();
            layer.accept_gossips(identity, &merged_profiles, origin, &mut self.view);
        }
    }

    fn collect_profiles(
        &self,
        identity: &mut Profile,
        target: &Profile,
        target_profiles: &mut ProfileSet,
    ) {
        // TODO: read lock.
        //        for layer in self.layers {
        //            layer.collect_gossips(identity, &self.view, target, gossips);
        //        }
    }
}
