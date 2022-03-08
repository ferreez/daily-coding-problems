#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

fn main() {
    println!("Hello, World!");
    let node = Node {
        val: "root".to_string(),
        left: Some(Box::new(Node {
            val: "left".to_string(),
            left: Some(Box::new(Node {
                val: "left.left".to_string(),
                left: None,
                right: None
            })),
            right: None,1
        })),
        right: None,
    };
    let ser_node = serialize(node);
    println!("{}", ser_node);
    deserialize(ser_node);
}

#[derive(Serialize, Deserialize)]
struct Node
{
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

fn serialize(root: Node) -> String {
    return serde_json::to_string(&root).unwrap();
}

fn deserialize(s: String) -> Node {
    return serde_json::from_str(s.as_str()).unwrap();
}

#[test]
fn test () {
    let node = Node {
        val: "root".to_string(),
        left: Some(Box::new(Node {
            val: "left".to_string(),
            left: Some(Box::new(Node {
                val: "left.left".to_string(),
                left: None,
                right: None
            })),
            right: None,
        })),
        right: None,
    };
    assert_eq!(deserialize(serialize(node)).left.unwrap().left.unwrap().val, "left.left");
}
