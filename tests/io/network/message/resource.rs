use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::io::network::Resource;

#[test]
fn test_resource_new_parse() {
    let valid_resource_strs = vec!["none",
                                   "session",
                                   "node",
                                   "nodes",
                                   "coin",
                                   "coins",
                                   "input",
                                   "inputs",
                                   "output",
                                   "outputs",
                                   "transaction",
                                   "transactions",
                                   "blocknode",
                                   "blocknodes",
                                   "block",
                                   "blocks",
                                   "blockgraph",
                                   "blockgraphs",
                                   "custom",
                                   "error"];

    let invalid_resource_str = "wallet";

    for resource_str in valid_resource_strs.iter() {
        let res = Resource::parse(resource_str);
        assert!(res.is_ok());
    }

    let res = Resource::parse(invalid_resource_str);
    assert!(res.is_err());
}

#[test]
fn test_resource_check_resource() {}

#[test]
fn test_resource_display() {
    let resource_strs = vec!["none",
                             "session",
                             "node",
                             "nodes",
                             "coin",
                             "coins",
                             "input",
                             "inputs",
                             "output",
                             "outputs",
                             "transaction",
                             "transactions",
                             "blocknode",
                             "blocknodes",
                             "block",
                             "blocks",
                             "blockgraph",
                             "blockgraphs",
                             "custom",
                             "error"];

    for resource_str in resource_strs.iter() {
        let resource = Resource::parse(resource_str).unwrap();
        assert_eq!(format!("{}", resource), String::from(*resource_str));
    }
}

#[test]
fn test_resource_default() {
    let resource = Resource::default();
    assert_eq!(resource, Resource::None);
}

#[test]
fn test_resource_size() {
    let resource = Resource::default();
    let resource_size = (resource as u8).size();
    assert_eq!(resource.size(), resource_size)
}

#[test]
fn test_resource_serialize_json() {
    let resource_a = Resource::default();
    let res = resource_a.to_json();
    assert!(res.is_ok());

    let resource_a_json = res.unwrap();
    let res = Resource::from_json(&resource_a_json);
    assert!(res.is_ok());

    let resource_b = res.unwrap();
    assert_eq!(resource_a, resource_b);
}

#[test]
fn test_resource_serialize_bytes() {
    let resource_a = Resource::default();
    let res = resource_a.to_bytes();
    assert!(res.is_ok());

    let resource_a_bytes = res.unwrap();
    let res = Resource::from_bytes(&resource_a_bytes);
    assert!(res.is_ok());

    let resource_b = res.unwrap();
    assert_eq!(resource_a, resource_b);
}

#[test]
fn test_resource_serialize_hex() {
    let resource_a = Resource::default();
    let res = resource_a.to_hex();
    assert!(res.is_ok());

    let resource_a_hex = res.unwrap();
    let res = Resource::from_hex(&resource_a_hex);
    assert!(res.is_ok());
}