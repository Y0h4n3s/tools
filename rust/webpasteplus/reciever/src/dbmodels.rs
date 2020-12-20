use diesel::associations::BelongsTo;
use diesel::pg::PgConnection;
use diesel::pg::types::date_and_time::PgTimestamp;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Debug, Insertable)]
pub struct RootDomain {
    pub id: i32,
    pub hostname: String,
}


#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(RootDomain, foriegn_key = rid)]
pub struct SubDomain {
    pub id: i32,
    pub hostname: String,
    pub ip: String,
    pub vhost: bool,
    pub date_added: PgTimestamp,
    pub notes: String,
    pub rid: i32,
}
