use crate::network::gossip::layer::Layer;
use crate::network::gossip::profile::Priority::High;
use crate::network::gossip::profile::{Priority, Profile, ProfileSet, Topic, TopicMap};
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::iter::FromIterator;

const DEFAULT_VIEW_SIZE: usize = 20;
const DEFAULT_GOSSIP_SIZE: usize = 10;

pub struct Vicinity {
    view_size: usize,
    gossip_size: usize,
    view: ProfileSet,
}

impl Vicinity {
    fn new(view_size: usize, gossip_size: usize) -> Self {
        Vicinity {
            view_size,
            gossip_size,
            view: ProfileSet::default(),
        }
    }
}

impl Default for Vicinity {
    fn default() -> Self {
        Vicinity::new(DEFAULT_VIEW_SIZE, DEFAULT_GOSSIP_SIZE)
    }
}

impl Layer for Vicinity {
    fn accept_gossips(&mut self, identity: &mut Profile, target: &Profile, gossips: &ProfileSet) {
        // Get a proximity-sorted vector of gossips relative to the IDENTITY node.
        let mut sorted = gossips.iter().collect::<Vec<&Profile>>();
        sorted.sort_by(|left, right| {
            left.proximity_to(identity)
                .cmp(&right.proximity_to(identity))
        });

        // Take the top N profiles for each of the topics we are interested in (where N = self.view_size).
        let subscribed_to: HashSet<&Topic> = identity.subscriptions.keys().collect();
        let mut topic_map = TopicMap::default();
        for profile in sorted {
            for (topic, _) in &profile.subscriptions {
                if subscribed_to.contains(&topic) {
                    let profiles = topic_map.entry(*topic).or_default();
                    if profiles.len() < self.view_size {
                        profiles.insert(profile.clone());
                    }
                }
            }
        }

        // Re-calculate the priority of each topic subscription we are subscribed to based on how
        // many profiles we have gathered for each topic subscription. The more profiles we have,
        // the lower the priority should be. Hence, when we gossip, we expect to get back more
        // profiles for the topics that are under-subscribed.
        for (topic, priority) in &mut identity.subscriptions {
            if let Some(topic_profile_set) = topic_map.get(&topic) {
                let fill_percentage = (topic_profile_set.len() / self.view_size) / 100;
                if fill_percentage >= 80 {
                    *priority = Priority::Low;
                } else if fill_percentage >= 50 {
                    *priority = Priority::Medium
                } else {
                    *priority = Priority::High
                }
            }
        }

        // Overwrite out current view.
        self.view = topic_map.values().flatten().cloned().collect()
    }

    fn collect_gossips(self, identity: &mut Profile, target: &Profile, gossips: &mut ProfileSet) {
        // Get a proximity-sorted vector of gossips relative to the TARGET node.
        let mut sorted = self.view.iter().collect::<Vec<&Profile>>();
        sorted.sort_by(|left, right| left.proximity_to(target).cmp(&right.proximity_to(target)));

        // Copy only the top N gossips into the returned profile set.
        for gossip in sorted.into_iter().take(self.gossip_size) {
            gossips.insert(gossip.clone());
        }
    }
}
