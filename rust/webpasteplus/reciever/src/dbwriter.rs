use std::collections::HashMap;

use diesel::{debug_query, RunQueryDsl, ExpressionMethods, QueryResult};
use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::{self, ConnectionManager, Pool};

use crate::dbmodels::*;
use diesel::result::Error;
use diesel::sql_types::Integer;
use diesel::dsl::exists;

//TODO improve with traits
pub struct DbWriter {
    db_url: String,
}

impl DbWriter {
    pub fn new() -> DbWriter {
        let db_url = "".to_string();
        DbWriter { db_url }
    }
    pub fn connect(&mut self, db_url: String) -> Pool<ConnectionManager<PgConnection>> {
        let manager =
            Option::from(ConnectionManager::<PgConnection>::new(
                &db_url
            ));
        let pool = Option::from(r2d2::Pool::builder()
            .build(manager.unwrap())
            .expect("Failed to create pool."));
        pool.expect("Something Went Wrong With The Database Connection")
    }

    pub fn insert_raw(
        data: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
        conn: &PgConnection
    ) -> Result<SubDomainInsert, Error> {

        use crate::schema::sub_domains::dsl::*;
        use crate::schema;
        use diesel::*;
        let valid_inputs = data.get("valids").unwrap();

        //println!("inserting all: {:?}", data);
        for (domain, hrefs) in valid_inputs {
            let mut next_id = "".to_string();
            let domain_exists: Vec<Option<String>> =
                sub_domains
                    .filter(hostname.eq(domain))
                    .select(hostname)
                    .load::<Option<String>>(conn)
                    .unwrap();

            //println!("[*] checking value: {:?}", exists);
            if !domain_exists.is_empty() {

                use crate::schema::end_points::dsl::*;
                let mut eps_id = end_points
                    .filter(sid.eq(domain))
                    .select(id).get_result::<i32>(conn)
                    .unwrap();

                println!("eps_id: {}", eps_id);
                {
                    use crate::schema::end_point::dsl::*;
                    for (href, endpoints) in hrefs {
                        for endpoint in endpoints {
                            let record_exists: QueryResult<Option<String>> =
                                end_point.filter((value.eq(endpoint)))
                                    .filter(eid.eq(eps_id))
                                    .select(value)
                                    .get_result::<Option<String>>(conn);

                            if record_exists.is_err() {
                                let inserted_endpoint = EndPointInsert {
                                    value: Option::from(endpoint.to_string()),
                                    params: None,
                                    eid: eps_id.clone()
                                }.jsonify(eps_id.clone()).unwrap();
                                let query =
                                    insert_into(end_point).values(&inserted_endpoint);
                                //println!("[*] trying to insert into endpoint data {} ", debug_query::<Pg, _>(&query));
                                let result =
                                    query.returning(id).get_result::<i32>(conn).unwrap();
                                //println!("[+] insert results: {:?}", result);
                            }
                        }
                    }
                }
            } else {
                let inserted_subdomain = SubDomainInsert {
                    hostname: domain.clone()
                }.jsonify().unwrap();
                let query =
                    insert_into(sub_domains).values(&inserted_subdomain);
                //println!("[*] trying to insert into sub_domains data {} ", debug_query::<Pg, _>(&query));
                next_id =
                    query
                        .returning(hostname)
                        .get_result::<Option<String>>(conn)
                        .unwrap()
                        .unwrap();

                //println!("[+] insert results: {:?}", next_id);

                // insert endpoint chunks
                let mut next_entry_id = 0;
                {
                    use crate::schema::end_points::dsl::*;
                    let inserted_endpoints =
                        EndPointsInsert {
                        list_type: "n".to_string(),
                        sid: next_id.clone()
                    }.jsonify(next_id.clone()).unwrap();
                    let query =
                        insert_into(end_points).values(&inserted_endpoints);
                    //println!("[*] trying to insert into endpoints data {} ", debug_query::<Pg, _>(&query));
                    next_entry_id =
                        query.returning(id).get_result::<i32>(conn).unwrap();
                    //println!("[+] insert results: {:?}", next_id);
                }

                //insert individual domain endpoints
                {
                    use crate::schema::end_point::dsl::*;
                    for (href, endpoints) in hrefs {
                        for endpoint in endpoints {
                            let inserted_endpoint = EndPointInsert {
                                value: Option::from(endpoint.to_string()),
                                params: None,
                                eid: next_entry_id.clone()
                            }.jsonify(next_entry_id.clone()).unwrap();
                            let query =
                                insert_into(end_point).values(&inserted_endpoint);
                            //println!("[*] trying to insert into endpoint data {} ", debug_query::<Pg, _>(&query));
                            let result =
                                query.returning(id).get_result::<i32>(conn).unwrap();
                            //println!("[+] insert results: {:?}", result);
                        }
                    }
                }
            }
        }
        //println!("db writer is in the house");
        let host: String = "blablatest.com".to_string();

        let s =  SubDomainInsert {
            hostname: "dasf".to_string()
        };
        // /println!("{:?}", root_domain.jsonify().unwrap());
        //println!("db writer was successful?");
        //let iput = //SubDomainInsert{hostname:"test".to_string()}.jsonify().unwrap();
        //let query = insert_into(sub_domains)
          //  .values(( &iput));
        //let result: i32 = query.returning(id).get_result(conn).unwrap();

        Ok(s)
    }
}