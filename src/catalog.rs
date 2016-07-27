use std::str::from_utf8;
use std::collections::HashMap;

use rustc_serialize::json;
use curl::http;
use structs::Node;

/// Catalog can be used to query the Catalog endpoints
pub struct Catalog {
    endpoint: String,
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct ServiceNode {
    Address: String,
    Node: String,
    ServiceAddress: String,
    ServiceID: String,
    ServiceName: String,
    ServicePort: u16,
    ServiceTags: Vec<String>,
}


impl Catalog {
    pub fn new(address: &str) -> Catalog {
        Catalog { endpoint: format!("{}/v1/catalog", address) }
    }

    pub fn services(&self) -> HashMap<String, Vec<String>> {
        let url = format!("{}/services", self.endpoint);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }

    pub fn get_nodes(&self, service: String) -> Vec<Node> {
        let url = format!("{}/service/{}", self.endpoint, service);
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(err) => {
                panic!("consul: Could not convert to json: {:?}. Err: {}",
                       result,
                       err)
            }
        };
        let v_nodes = json_data.as_array().unwrap();
        let mut filtered: Vec<Node> = Vec::new();
        for node in v_nodes.iter() {
            let node_value = match super::get_string(node, &["Node"]) {
                Some(val) => val,
                None => panic!("consul: Could not find 'Node' in: {:?}", &node),
            };
            let address = match super::get_string(node, &["Address"]) {
                Some(val) => val,
                None => panic!("consul: Could not find 'Address' in: {:?}", &node),
            };
            filtered.push(Node {
                Node: node_value,
                Address: address,
            });
        }
        filtered
    }
}
