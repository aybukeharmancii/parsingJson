extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_female: bool
}

fn main() {
    let json_str = r#"
        {
            "name": "Aybuke",
            "age": 23,
            "is_female": true
        }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let p: Person = res.unwrap(); 
        println!("The name is {}", p.name);
        println!("The age is {}", p.age);
        println!("Are they female? {}", p.is_female);
        // let p: JsonValue = res.unwrap();    -- we used this when we didn't use struct
        //println!("The name is {}", p["name"].as_str().unwrap());
    }
    else {
        println!("Sorry! Could not parse JSON :(");
    }

    // .as_str().unwrap() is risky
    
}
