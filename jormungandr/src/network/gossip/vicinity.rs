use crate::network::gossip::layer::Layer;
use crate::network::gossip::profile::{Priority, Profile, ProfileSet, Topic};
use itertools::Itertools;
use std::collections::HashMap;

const DEFAULT_VIEW_SIZE: usize = 20;
const DEFAULT_GOSSIP_SIZE: usize = 10;

pub struct Vicinity {
    view_size: usize,
    gossip_size: usize,
}

// The Vicinity layer is responsible for maintaining interest-induced random links, that is,
// randomly chosen links between nodes that share one or more topics.
impl Vicinity {
    fn new(view_size: usize, gossip_size: usize) -> Self {
        Vicinity {
            view_size,
            gossip_size,
        }
    }
}

impl Default for Vicinity {
    fn default() -> Self {
        Vicinity::new(DEFAULT_VIEW_SIZE, DEFAULT_GOSSIP_SIZE)
    }
}

impl Layer for Vicinity {
    fn accept_gossips(
        &mut self,
        identity: &mut Profile,
        input: &ProfileSet,
        origin: &Profile,
        output: &mut ProfileSet,
    ) {
        let topic_map: HashMap<Topic, Vec<&Profile>> = input
            .into_iter()
            // 1) Filter for profiles that we share at least one common subscription with.
            .filter(|profile| {
                profile
                    .subscriptions
                    .keys()
                    .any(|topic| identity.subscriptions.contains_key(topic))
            })
            // 2) Sort using proximity function relative to the IDENTITY.
            .sorted_by(|left, right| {
                left.proximity_to(identity)
                    .cmp(&right.proximity_to(identity))
            })
            // 3) Expand to an iterator of (Topic, &Profile) tuples.
            .map(|profile| {
                profile
                    .subscriptions
                    .keys()
                    .into_iter()
                    .map(move |topic| (*topic, profile))
            })
            // 4) Merge into topics-to-profile buckets.
            .flatten()
            .into_iter()
            .into_group_map();

        // Adjust the priority of each topic that we subscribe to based on how many profiles we have
        // gathered for each topic subscription. The more profiles we have, the lower the priority
        // should be. Hence, when we gossip, we expect to get back more profiles for the topics
        // that are most under-subscribed.
        identity
            .subscriptions
            .iter_mut()
            .for_each(|(topic, priority)| {
                if let Some(topic_profile_set) = topic_map.get(topic) {
                    let fill_percentage = (topic_profile_set.len() / self.view_size) / 100;
                    if fill_percentage >= 80 {
                        *priority = Priority::Low;
                    } else if fill_percentage >= 50 {
                        *priority = Priority::Medium
                    } else {
                        *priority = Priority::High
                    }
                }
            });

        // Flatten all of the profiles into the output.
        //        for profiles in topic_map.values().into_iter().clone().into_iter().cloned() {
        //            output.extend(profiles);
        //        }
    }

    fn collect_gossips(
        &self,
        identity: &mut Profile,
        input: &ProfileSet,
        target: &Profile,
        output: &mut ProfileSet,
    ) {
        // Get a proximity-sorted vector of gossips relative to the TARGET node.
        let mut sorted = input.iter().collect::<Vec<&Profile>>();
        sorted.sort_by(|left, right| left.proximity_to(target).cmp(&right.proximity_to(target)));

        // Write the top N (where n = self.gossip_size) profiles to the resulting view.
        for gossip in sorted.into_iter().take(self.gossip_size) {
            output.insert(gossip.clone());
        }
    }
}
