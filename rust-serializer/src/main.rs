mod serialize_stcut {
    use serde::ser::{Serialize, SerializeStruct};
    struct Person {
        name: String,
        age: u8,
        phones: Vec<String>,
    }

    impl Serialize for Person {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut s = serializer.serialize_struct("person", 3)?;
            s.serialize_field("name", &self.name);
            s.serialize_field("age", &self.age);
            s.serialize_field("phones", &self.phones);
            s.end()
        }
    }
}

mod serialize_yaml {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Address {
        pub uid: u32,
        pub addr_id: u32,
        pub addr: String,
        pub addr_detail: String,
    }

    impl Clone for Address {
        fn clone(&self) -> Self {
            Self {
                uid: self.uid.clone(),
                addr_id: self.addr_id.clone(),
                addr: self.addr.clone(),
                addr_detail: self.addr_detail.clone(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        pub uid: u32,
        pub nickname: String,
        pub realname: String,
        pub address: Vec<Address>,
    }
}

use serialize_yaml::{Address, User};
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let user = User {
        uid: 100,
        nickname: "dasdasdas".to_string(),
        realname: "12213".to_string(),
        address: vec![Address {
            uid: 100,
            addr_id: 001,
            addr: "Asia/Shanghai".to_string(),
            addr_detail: "Baoshan".to_string(),
        }],
    };

    let json_str = serde_json::to_string_pretty(&user).unwrap();
    let yaml_str = serde_yaml::to_string(&user).unwrap();

    println!("json_str: {}", json_str);
    println!("yaml_str: {}", yaml_str);

    let user_json: Vec<User> = serde_json::from_str(json_str.as_str()).unwrap();
    let user_yaml: Vec<User> = serde_yaml::from_str(yaml_str.as_str()).unwrap();

    println!("user_json: {:?}", user_json);
    println!("user_yaml: {:?}", user_yaml)
}
