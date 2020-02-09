use crate::network::gossip::layer::Layer;
use crate::network::gossip::profile::{Profile, ProfileSet};
use rand::seq::IteratorRandom;
use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::iter::FromIterator;

const DEFAULT_VIEW_SIZE: usize = 20;
const DEFAULT_GOSSIP_SIZE: usize = 10;

pub struct Cyclon {
    view_size: usize,
    gossip_size: usize,
    view: ProfileSet,
}

impl Cyclon {
    fn new(view_size: usize, gossip_size: usize) -> Self {
        Cyclon {
            view_size,
            gossip_size,
            view: ProfileSet::default(),
        }
    }
}

impl Default for Cyclon {
    fn default() -> Self {
        return Cyclon::new(DEFAULT_VIEW_SIZE, DEFAULT_GOSSIP_SIZE);
    }
}

impl Layer for Cyclon {
    fn accept_gossips(&mut self, identity: &mut Profile, target: &Profile, gossips: &ProfileSet) {
        self.view = HashSet::from_iter(
            gossips
                .into_iter()
                .cloned()
                .choose_multiple(&mut rand::thread_rng(), self.view_size),
        );
    }

    fn collect_gossips(self, identity: &mut Profile, target: &Profile, gossips: &mut ProfileSet) {
        self.view
            .iter()
            .choose_multiple(&mut rand::thread_rng(), self.gossip_size)
            .into_iter()
            .cloned()
            .for_each(move |profile| {
                gossips.insert(profile);
            });
    }
}
