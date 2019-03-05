// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use radix_fmt::{radix, Radix};
use std::hash::Hasher;
use twox_hash::{XxHash, XxHash32};

use super::identifiers::*;

pub struct StoreKeyerBuilder;

pub struct StoreKeyer<'a> {
    idx: StoreKeyerIdx<'a>,
    bucket: StoreKeyerBucket<'a>,
}

enum StoreKeyerIdx<'a> {
    TermToIIDs(&'a str),
    OIDToIID(StoreObjectOID),
    IIDToOID(StoreObjectIID),
    IIDToTerms(StoreObjectIID),
}

type StoreKeyerBucket<'a> = &'a str;
type StoreKeyerBucketCompacted = Radix<u64>;
type StoreKeyerRouteCompacted = Radix<u64>;

const STORE_KEYER_BUCKET_COMPACT_BASE: u32 = 36;
const STORE_KEYER_ROUTE_COMPACT_BASE: u32 = 36;

impl<'a> StoreKeyerIdx<'a> {
    pub fn to_index(&self) -> u8 {
        match self {
            StoreKeyerIdx::TermToIIDs(_) => 0,
            StoreKeyerIdx::OIDToIID(_) => 1,
            StoreKeyerIdx::IIDToOID(_) => 2,
            StoreKeyerIdx::IIDToTerms(_) => 3,
        }
    }
}

impl StoreKeyerBuilder {
    pub fn term_to_iids<'a>(bucket: &'a str, term: &'a str) -> StoreKeyer<'a> {
        StoreKeyer {
            idx: StoreKeyerIdx::TermToIIDs(term),
            bucket: bucket,
        }
    }

    pub fn oid_to_iid<'a>(bucket: &'a str, oid: StoreObjectOID) -> StoreKeyer<'a> {
        StoreKeyer {
            idx: StoreKeyerIdx::OIDToIID(oid),
            bucket: bucket,
        }
    }

    pub fn iid_to_oid<'a>(bucket: &'a str, iid: StoreObjectIID) -> StoreKeyer<'a> {
        StoreKeyer {
            idx: StoreKeyerIdx::IIDToOID(iid),
            bucket: bucket,
        }
    }

    pub fn iid_to_terms<'a>(bucket: &'a str, iid: StoreObjectIID) -> StoreKeyer<'a> {
        StoreKeyer {
            idx: StoreKeyerIdx::IIDToTerms(iid),
            bucket: bucket,
        }
    }
}

impl<'a> StoreKeyer<'a> {
    pub fn to_string(&self) -> String {
        format!(
            "{}:{}:{}",
            self.idx.to_index(),
            self.bucket_to_compact(),
            self.route_to_compact()
        )
    }

    pub fn bucket_to_compact(&self) -> StoreKeyerBucketCompacted {
        let mut hasher = XxHash32::with_seed(0);

        hasher.write(self.bucket.as_bytes());

        radix(hasher.finish(), STORE_KEYER_BUCKET_COMPACT_BASE)
    }

    pub fn route_to_compact(&self) -> StoreKeyerRouteCompacted {
        let value = match &self.idx {
            StoreKeyerIdx::TermToIIDs(route) => Self::hash_route_text(route),
            StoreKeyerIdx::OIDToIID(route) => Self::hash_route_text(route),
            StoreKeyerIdx::IIDToOID(route) => *route,
            StoreKeyerIdx::IIDToTerms(route) => *route,
        };

        radix(value, STORE_KEYER_ROUTE_COMPACT_BASE)
    }

    fn hash_route_text(text: &str) -> u64 {
        let mut hasher = XxHash::with_seed(0);

        hasher.write(text.as_bytes());
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_keys_term_to_iids() {
        assert_eq!(
            StoreKeyerBuilder::term_to_iids("user:0dcde3a6", "hello").to_string(),
            "0:vngsgj:l8a8u0vgmher"
        );
        assert_eq!(
            StoreKeyerBuilder::term_to_iids("default", "yes").to_string(),
            "0:tlegv5:8hzoehaig16x"
        );
    }

    #[test]
    fn it_keys_oid_to_iid() {
        assert_eq!(
            StoreKeyerBuilder::oid_to_iid("user:0dcde3a6", "conversation:6501e83a".to_string())
                .to_string(),
            "1:vngsgj:330ky6g2kd34c"
        );
    }

    #[test]
    fn it_keys_iid_to_oid() {
        assert_eq!(
            StoreKeyerBuilder::iid_to_oid("user:0dcde3a6", 10292198).to_string(),
            "2:vngsgj:64lie"
        );
    }

    #[test]
    fn it_keys_iid_to_terms() {
        assert_eq!(
            StoreKeyerBuilder::iid_to_terms("user:0dcde3a6", 1).to_string(),
            "3:vngsgj:1"
        );
        assert_eq!(
            StoreKeyerBuilder::iid_to_terms("user:0dcde3a6", 20).to_string(),
            "3:vngsgj:k"
        );
    }
}
