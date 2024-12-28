use serde::Serialize;
use z157::Tree;

#[derive(Serialize)]
struct User {
    name: String,
    bio: Bio,
    last_seen: String,
}
#[derive(Serialize)]
struct Bio {
    year_of_birth: u16,
    height_cm: u16,
}

fn main() {
    let user = User {
        name: String::from("Ford Prefect"),
        bio: Bio {
            height_cm: 180,
            year_of_birth: 1779,
        },
        last_seen: String::from("1979-12-28"),
    };
    let fields = std::env::args()
        .nth(1)
        .map(|fields| Tree::parse(fields).unwrap());
    if let Some(fields) = fields {
        let mut user = serde_json::to_value(user).unwrap();
        if fields.negation() {
            for field in fields.leaves() {
                let mut remove = user.as_object_mut().unwrap();
                let path = field.path();
                for &part in path.iter().take(path.len() - 1) {
                    remove = remove.get_mut(part).unwrap().as_object_mut().unwrap();
                }
                remove.remove(*path.last().unwrap()).unwrap();
            }
        } else {
            let mut out = serde_json::Map::new();
            for field in fields.leaves() {
                let mut insert = &mut out;
                let mut user_field_container = user.as_object_mut().unwrap();
                let path = field.path();
                for &part in path.iter().take(path.len() - 1) {
                    insert = insert
                        .entry(part)
                        .or_insert(serde_json::Value::Object(serde_json::Map::new()))
                        .as_object_mut()
                        .unwrap();
                    user_field_container = user_field_container
                        .get_mut(part)
                        .unwrap()
                        .as_object_mut()
                        .unwrap();
                }
                insert.insert(
                    (*path.last().unwrap()).to_string(),
                    user_field_container
                        .get(*path.last().unwrap())
                        .unwrap()
                        .clone(),
                );
            }
            user = serde_json::Value::Object(out);
        }
        println!("{}", serde_json::to_string_pretty(&user).unwrap());
    } else {
        println!("{}", serde_json::to_string_pretty(&user).unwrap());
    }
}
