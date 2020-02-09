use crate::network::gossip::profile::{Profile, ProfileSet};
use std::collections::HashSet;

pub trait Layer {
    fn accept_gossips(&mut self, identity: &mut Profile, target: &Profile, gossips: &ProfileSet);

    fn collect_gossips(self, identity: &mut Profile, target: &Profile, gossips: &mut ProfileSet);
}
