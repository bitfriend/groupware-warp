use arangors::{
    document::{
        options::{InsertOptions, RemoveOptions, UpdateOptions},
        response::DocumentResponse,
    },
    AqlQuery, Collection, Database, Document,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    convert::Infallible,
};
use uclient::reqwest::ReqwestClient;
use warp;

use crate::config::db_database;
use crate::database::{DbConn, DbPool};
use crate::user::UserResponse;

pub async fn find_users(
    pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    let conn: DbConn = pool.get().unwrap();
    let db: Database<ReqwestClient> = conn.db(&db_database()).unwrap();
    let mut terms = vec!["FOR x IN companies"];
    let mut vars: HashMap<&str, Value> = HashMap::new();
    // if params.search.is_some() {
    //     let search: String = params.search.unwrap().trim().to_string();
    //     if !search.is_empty() {
    //         terms.push("FILTER CONTAINS(x.name, @@search)");
    //         vars.insert("@search", to_value(search).unwrap());
    //     }
    // }
    // if params.sort_by.is_some() {
    //     let sort_by: String = params.sort_by.unwrap();
    //     terms.push("SORT x.@@sort_by ASC");
    //     vars.insert("@sort_by", to_value(sort_by).unwrap());
    // }
    // if params.limit.is_some() {
    //     let limit: u32 = params.limit.unwrap();
    //     terms.push("LIMIT 0, @@limit");
    //     vars.insert("@limit", to_value(limit).unwrap());
    // }
    terms.push("RETURN x");
    let q = terms.join(" ");
    let aql = AqlQuery::builder()
        .query(&q)
        .bind_vars(vars)
        .build();
    let records: Vec<UserResponse> = db.aql_query(aql).expect("Query failed");
    Ok(warp::reply::json(&records))
}