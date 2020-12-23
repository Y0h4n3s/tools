use crate::models::dbmodels::*;
use crate::models::request_models::MuchData;
pub mod parsers {
    use super::*;

    pub fn much_data_parse(
        data: &MuchData,
    ) -> (
        SubDomainsInsert,
        EndPointsInsert,
        EndPointInsert,
        Vec<ParamsInsert>,
    ) {
        let sub: SubDomainsInsert = SubDomainsInsert {
            hostname: data.hostname.clone(),
            is_root: None,
            ip: None,
            protocol: None,
            port: None,
            vhost: None,
            notes: None,
            
        };

        let endps = EndPointsInsert {
            list_type: "n".to_string(),
            href: data.link_only.clone(),
            sid: 0,
        };

        let endp = EndPointInsert {
            value: data.path_only.clone(),
            href: Option::from(data.full_link.clone()),
            path_href: Option::from(data.full_path.clone()),
            link_from: Option::from(data.page_from.clone()),
            hitcount: 0,
            eid: 0,
        };

        let params = parse_params(&data.params);

        (sub, endps, endp, params)
    }

    pub fn parse_params(params: &String) -> Vec<ParamsInsert> {
        let split_params: Vec<&str> = params.split('&').collect();
        let mut params_container: Vec<ParamsInsert> = Vec::new();
        for param in split_params {

            let p: Vec<&str> = param.split('=').collect();
            let (key, value) = match p.len() {
                1 => (p[0], ""),
                other=> (p[0], p[1])
            };
            let p = ParamsInsert {
                key: key.to_string(),
                value: Option::from(value.to_string()),
                epid: 0,
            };
            params_container.push(p);
        }

        params_container
    }
}
