use vector4::Vector4;
use std::cmp::Ord;
use std::cmp::Ordering;

struct Photon {
    position: Vector4,
    direction: Vector4,
}

struct KDTreeNode {
    photon: Photon,
    split_axis: u32,
}

struct PhotonMap {
    count: u32,
    mins: Vector4,
    maxs: Vector4,
    begin: Box<KDTreeNode>,
}

struct NodeWithKnownBoundsAndMinDistance {
    mins: Vector4,
    maxs: Vector4,
    node: Box<KDTreeNode>,
    min_distance: f32,
    tree_size: u32,
}

impl NodeWithKnownBoundsAndMinDistance {
    fn new(mins: Vector4,
           maxs: Vector4,
           node: Box<KDTreeNode>,
           min_distance: f32,
           tree_size: u32)
           -> Self {
        debug_assert!(min_distance >= 0.0);
        debug_assert!(tree_size >= 1);
        debug_assert!(Vector4::min(mins, maxs) == mins);
        debug_assert!(Vector4::max(mins, maxs) == maxs);
        debug_assert!(Vector4::min(mins, node.photon.position) == mins);
        debug_assert!(Vector4::max(maxs, node.photon.position) == maxs);

        NodeWithKnownBoundsAndMinDistance {
            mins: mins,
            maxs: maxs,
            node: node,
            min_distance: min_distance,
            tree_size: tree_size
        }
    }
}

impl PartialOrd for NodeWithKnownBoundsAndMinDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NodeWithKnownBoundsAndMinDistance {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for NodeWithKnownBoundsAndMinDistance {}

impl Ord for NodeWithKnownBoundsAndMinDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        // Ignore floating point values that don't compare nicely (I'm looking at you, NaN)
        self.min_distance.partial_cmp(&other.min_distance).unwrap_or(Ordering::Equal)
    }
}