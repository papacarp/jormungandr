use crate::network::gossip::profile::{Profile, ProfileSet};

pub trait Layer {
    fn accept_gossips(
        &mut self,
        identity: &mut Profile,
        input: &ProfileSet,
        origin: &Profile,
        output: &mut ProfileSet,
    );

    fn collect_gossips(
        &self,
        identity: &mut Profile,
        input: &ProfileSet,
        target: &Profile,
        output: &mut ProfileSet,
    );
}
