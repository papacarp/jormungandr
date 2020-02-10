use crate::network::gossip::layer::Layer;
use crate::network::gossip::profile::{Profile, ProfileSet, Topic, TopicMap};
use crate::settings::Block0Info::Hash;
use rand::seq::IteratorRandom;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

// Default number of neighbours to collect per topic (either side or origin profile).
const DEFAULT_RING_NEIGHBOURS: usize = 2;
const DEFAULT_VIEW_SIZE: usize = 20;
const DEFAULT_GOSSIP_SIZE: usize = 10;

struct Rings {
    view_size: usize,
    view: ProfileSet,
}

impl Rings {}

impl Layer for Rings {
    fn accept_gossips(&mut self, identity: &mut Profile, target: &Profile, gossips: &ProfileSet) {
        // First, p collects all subscribers of topics which p and q have in common
        let mut interesting_profiles: Vec<&Profile> = Vec::new();
        for topic in identity.subscriptions.keys() {
            for profile in gossips {
                if profile.subscriptions.contains_key(topic) {
                    interesting_profiles.push(profile);
                    break; // skip multiples
                }
            }
        }

        // Sort the interesting topics id.
        interesting_profiles.sort_by(|left, right| left.id.cmp(&right.id));

        // Group all profiles by topics -- retaining sort order (albeit in reverse).
        let mut topic_map: HashMap<Topic, Vec<&Profile>> = HashMap::new();
        for topic in identity.subscriptions.keys() {
            for profile in &interesting_profiles {
                topic_map.entry(*topic).or_default().push(*profile);
            }
        }

        // Split profiles into left and right of the IDENTITY profile with no more than
        // DEFAULT_RING_NEIGHBOURS per group.
        let mut neighbours = ProfileSet::new();
        for profiles in topic_map.values() {
            for groups in profiles.splitn(DEFAULT_RING_NEIGHBOURS, |profile| {
                identity.id <= identity.id
            }) {
                for profile in groups {
                    neighbours.insert((*profile).clone());
                }
            }
        }

        // Overwrite our current view.
        self.view = neighbours;
    }

    fn collect_gossips(self, identity: &mut Profile, target: &Profile, gossips: &mut ProfileSet) {}
}
