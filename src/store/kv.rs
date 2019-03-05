// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocksdb::{DBCompactionStyle, DBCompressionType, Error as DBError, Options as DBOptions, DB};

use super::identifiers::*;
use super::item::StoreItemPart;
use super::keyer::StoreKeyerBuilder;
use crate::APP_CONF;

pub struct StoreKVPool;
pub struct StoreKVBuilder;

pub struct StoreKV {
    database: DB,
}

pub struct StoreKVActionBuilder;

pub struct StoreKVAction<'a> {
    store: StoreKV,
    bucket: StoreItemPart<'a>,
}

impl StoreKVPool {
    pub fn acquire(target: &str) -> Result<StoreKV, DBError> {
        // TODO: pool and auto-close or auto-open if needed
        // TODO: keep it in a LAZY_STATIC global object
        StoreKVBuilder::new()
    }
}

impl StoreKVBuilder {
    pub fn new() -> Result<StoreKV, DBError> {
        Self::open().map(|db| StoreKV { database: db })
    }

    fn open() -> Result<DB, DBError> {
        debug!("opening key-value database");

        // Configure database options
        let db_options = Self::configure();

        // Acquire path to database
        // TODO: 1 database per collection
        // TODO: auto-close file descriptor if not used in a while, and re-open whenever needed
        let db_path = APP_CONF.store.kv.path.join("./collection");

        DB::open(&db_options, db_path)
    }

    fn configure() -> DBOptions {
        debug!("configuring key-value database");

        // Make database options
        let mut db_options = DBOptions::default();

        db_options.create_if_missing(true);
        db_options.set_use_fsync(false);
        db_options.set_compaction_style(DBCompactionStyle::Level);

        db_options.set_compression_type(if APP_CONF.store.kv.database.compress == true {
            DBCompressionType::Lz4
        } else {
            DBCompressionType::None
        });

        db_options.increase_parallelism(APP_CONF.store.kv.database.parallelism as i32);
        db_options.set_max_open_files(APP_CONF.store.kv.database.max_files as i32);
        db_options
            .set_max_background_compactions(APP_CONF.store.kv.database.max_compactions as i32);
        db_options.set_max_background_flushes(APP_CONF.store.kv.database.max_flushes as i32);

        db_options
    }
}

impl StoreKVActionBuilder {
    pub fn new<'a>(bucket: StoreItemPart<'a>, store: StoreKV) -> StoreKVAction<'a> {
        StoreKVAction {
            store: store,
            bucket: bucket,
        }
    }
}

impl<'a> StoreKVAction<'a> {
    /// Term-to-IIDs mapper
    ///
    /// [IDX=0] ((term)) ~> [((iid))]
    pub fn get_term_to_iids(&self, term: &str) -> Option<Vec<StoreObjectIID>> {
        let keyer = StoreKeyerBuilder::term_to_iids(self.bucket.as_str(), term);

        // TODO
        None
    }

    pub fn set_term_to_iids(&self, term: &str, iids: Vec<StoreObjectIID>) -> Result<(), ()> {
        let keyer = StoreKeyerBuilder::term_to_iids(self.bucket.as_str(), term);

        // TODO
        Err(())
    }

    pub fn delete_term_to_iids(&self, term: &str) -> Result<(), ()> {
        let keyer = StoreKeyerBuilder::term_to_iids(self.bucket.as_str(), term);

        // TODO
        Err(())
    }

    /// OID-to-IID mapper
    ///
    /// [IDX=1] ((oid)) ~> ((iid))
    pub fn get_oid_to_iid(&self, oid: StoreObjectOID) -> Option<StoreObjectIID> {
        let keyer = StoreKeyerBuilder::oid_to_iid(self.bucket.as_str(), oid);

        // TODO
        None
    }

    pub fn set_oid_to_iid(&self, oid: StoreObjectOID, iid: StoreObjectIID) -> Result<(), ()> {
        let keyer = StoreKeyerBuilder::oid_to_iid(self.bucket.as_str(), oid);

        // TODO
        Err(())
    }

    pub fn delete_oid_to_iid(&self, oid: StoreObjectOID) -> Result<(), ()> {
        let keyer = StoreKeyerBuilder::oid_to_iid(self.bucket.as_str(), oid);

        // TODO
        Err(())
    }

    /// IID-to-OID mapper
    ///
    /// [IDX=2] ((iid)) ~> ((oid))
    pub fn get_iid_to_oid(&self, iid: StoreObjectIID) -> Option<StoreObjectOID> {
        let keyer = StoreKeyerBuilder::iid_to_oid(self.bucket.as_str(), iid);

        // TODO
        None
    }

    pub fn set_iid_to_oid(&self, iid: StoreObjectIID, oid: StoreObjectOID) -> Result<(), ()> {
        let keyer = StoreKeyerBuilder::iid_to_oid(self.bucket.as_str(), iid);

        // TODO
        Err(())
    }

    pub fn delete_iid_to_oid(&self, iid: StoreObjectIID) -> Result<(), ()> {
        let keyer = StoreKeyerBuilder::iid_to_oid(self.bucket.as_str(), iid);

        // TODO
        Err(())
    }

    /// IID-to-Terms mapper
    ///
    /// [IDX=3] ((iid)) ~> [((term))]
    pub fn get_iid_to_terms(&self, iid: StoreObjectIID) -> Option<Vec<String>> {
        let keyer = StoreKeyerBuilder::iid_to_terms(self.bucket.as_str(), iid);

        // TODO
        None
    }

    pub fn set_iid_to_terms(&self, iid: StoreObjectIID, terms: &[&'a str]) -> Result<(), ()> {
        let keyer = StoreKeyerBuilder::iid_to_terms(self.bucket.as_str(), iid);

        // TODO
        Err(())
    }

    pub fn delete_iid_to_terms(&self, iid: StoreObjectIID) -> Result<(), ()> {
        let keyer = StoreKeyerBuilder::iid_to_terms(self.bucket.as_str(), iid);

        // TODO
        Err(())
    }
}
