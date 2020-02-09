use crate::network::gossip::layer::Layer;
use crate::network::gossip::profile::{Profile, ProfileSet, Topic, TopicMap};
use crate::settings::Block0Info::Hash;
use rand::seq::IteratorRandom;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;

struct Rings {
    view_size: usize,
    view: ProfileSet,
}

impl Rings {}

impl Layer for Rings {
    fn accept_gossips(&mut self, identity: &mut Profile, target: &Profile, gossips: &ProfileSet) {
        // Get a node-id sorted vector of gossips.
        let mut sorted = gossips.iter().collect::<Vec<&Profile>>();
        sorted.sort_by(|left, right| left.id.cmp(&right.id));

        // Now group each profile by its subscriptions to topics (if one or more matches our own).
        let subscribed_to: HashSet<&Topic> = identity.subscriptions.keys().collect();
        let mut topic_map = TopicMap::default();
    }

    fn collect_gossips(self, identity: &mut Profile, target: &Profile, gossips: &mut ProfileSet) {}
}
