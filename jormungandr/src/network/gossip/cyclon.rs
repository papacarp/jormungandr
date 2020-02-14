use crate::network::gossip::layer::Layer;
use crate::network::gossip::profile::{Profile, ProfileSet};
use rand::seq::IteratorRandom;

const DEFAULT_VIEW_SIZE: usize = 20;
const DEFAULT_GOSSIP_SIZE: usize = 10;

pub struct Cyclon {
    view_size: usize,
    gossip_size: usize,
}

impl Cyclon {
    fn new(view_size: usize, gossip_size: usize) -> Self {
        Cyclon {
            view_size,
            gossip_size,
        }
    }
}

impl Default for Cyclon {
    fn default() -> Self {
        return Cyclon::new(DEFAULT_VIEW_SIZE, DEFAULT_GOSSIP_SIZE);
    }
}

impl Layer for Cyclon {
    fn accept_gossips(
        &mut self,
        _identity: &mut Profile,
        input: &ProfileSet,
        _origin: &Profile,
        output: &mut ProfileSet,
    ) {
        output.extend(
            input
                .into_iter()
                .cloned()
                .choose_multiple(&mut rand::thread_rng(), self.view_size),
        );
    }

    fn collect_gossips(
        &self,
        _identity: &mut Profile,
        input: &ProfileSet,
        _target: &Profile,
        output: &mut ProfileSet,
    ) {
        output.extend(
            input
                .into_iter()
                .cloned()
                .choose_multiple(&mut rand::thread_rng(), self.gossip_size),
        );
    }
}
