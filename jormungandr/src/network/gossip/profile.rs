use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::net::IpAddr;

pub type ProfileSet = HashSet<Profile>;
pub type TopicMap = HashMap<Topic, ProfileSet>;
pub type SubscriptionSet = HashMap<Topic, Priority>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeId([u8; 24]);

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct Topic(u32);

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub enum Priority {
    High = 3,
    Medium = 2,
    Low = 1,
}

#[derive(Eq, Clone)]
pub struct Profile {
    pub id: NodeId,
    pub ip: IpAddr,
    pub port: u16,
    pub subscriptions: SubscriptionSet,
}

impl Profile {
    // TODO: We should probably apply a penalty for topics that only partially overlap.
    pub fn proximity_to(&self, other: &Self) -> i32 {
        let mut score: i32 = 0;
        for (topic1, priority1) in &self.subscriptions {
            for (topic2, priority2) in &other.subscriptions {
                if topic1 == topic2 {
                    score += *priority1 as i32 * *priority2 as i32;
                }
            }
        }
        score
    }
}

impl Hash for Profile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
