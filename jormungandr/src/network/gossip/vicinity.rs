use crate::network::gossip::layer::Layer;
use crate::network::gossip::profile::{Priority, Profile, ProfileSet, Topic};
use itertools::Itertools;
use rand::seq::IteratorRandom;
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
        _origin: &Profile,
        output: &mut ProfileSet,
    ) {
        let topic_map: HashMap<Topic, Vec<&Profile>> = input
            .into_iter()
            // 1) Sort using proximity function relative to the IDENTITY.
            .sorted_by(|left, right| {
                left.proximity_to(identity)
                    .cmp(&right.proximity_to(identity))
            })
            // 2) Expand to an iterator of (Topic, &Profile) tuples.
            .map(|profile| {
                profile
                    .subscriptions
                    .keys()
                    .into_iter()
                    .map(move |topic| (*topic, profile))
            })
            // 3) Merge into topics-to-profile buckets.
            .flatten()
            .into_iter()
            .into_group_map();

        // 5) Adjust the priority of each topic that we subscribe to based on how many profiles we
        // have gathered for each topic subscription. The more profiles we have, the lower the
        // priority should be. Hence, when we gossip, we expect to get back more profiles for the
        // topics that are most under-subscribed.
        identity
            .subscriptions
            .iter_mut()
            .for_each(|(topic, priority)| {
                if let Some(topic_profile_set) = topic_map.get(topic) {
                    // NB: These breakpoint values are relatively arbitrary. Ideally, priority
                    // would be represented by a greater value set (e.g. 0-100). This would be
                    // particularly beneficial if there were many subscribed topics as it would
                    // result in an a fairer priority allocation.
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

        // 6) Take the top N profiles (where N = self.view_size) for each subscribed topic and
        //    collect them into a single vector.
        let trimmed = topic_map
            .values()
            .into_iter()
            .take(self.view_size)
            .flatten()
            // 7) Clone each profile, and then filter out any duplicates (because a single profile
            //    may have been mapped to multiple topics previously -- see steps 2 & 4 above).
            .into_iter()
            .cloned()
            .unique()
            .collect::<Vec<&Profile>>(); // 'uniqueness' is based on node ID (see Profile equality function).

        // 7) Now we just need to make sure that we have the amount optimum amount of selected
        //    profiles (i.e. self.gossip_size). If not, we combine with as many unique randomly
        //    selected profiles as possible to make up the numbers.
        let selected: ProfileSet = trimmed.into_iter().cloned().collect();
        let random: ProfileSet = selected
            .difference(input)
            .choose_multiple(&mut rand::thread_rng(), self.gossip_size - selected.len())
            .into_iter()
            .cloned()
            .collect();

        // 8) Finally we update the output by inserting the nodes selected in step 7.
        output.extend(selected);
        output.extend(random);
    }

    fn collect_gossips(
        &self,
        _identity: &mut Profile,
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
