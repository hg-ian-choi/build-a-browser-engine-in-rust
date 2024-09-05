use std::collections::{HashMap, HashSet};
use std::mem::offset_of;
/*
    The DOM is a tree of nodes. A node has zero or more children. (It also has
    various other attributes and methods, but we can ignore most of those for now.)
 */

pub struct Node {
    // data common to all nodes
    children: Vec<Node>,

    // data specific to each node type
    node_type: NodeType,
}


/*
    There are several node types, but for now we will ignore most of them and say
    that a node is either an Element or a Text node. In a language with inheritance
    these would be subtypes of Node. In Rust, they can be enum (Rust's keyword for
    a "tagged union" or "sum type").
 */

pub enum NodeType {
    Text(String),
    Element(ElementData),
}


/*
    An element includes a tag name and any number of attributes, which can be stored
    as a map from names to values. This engine doesn't support namespaces, so it just
    stores tag and attribute names as simple strings.
 */

pub struct ElementData {
    tag_name: String,
    attrs: AttrMap,
}

pub type AttrMap = HashMap<String, String>;


/*
    Some constructor functions to make it easy to create new codes:
 */

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData { tag_name, attrs }),
    }
}


/*
    To help, we'll add some convenient ID and class accessors to out [DOM element type](dom.rs).
    The class attribute can contain multiple class names separated by spaces, which we return
    in a hash table.
 */

// Element methods

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attrs.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attrs.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            Node => HashSet::new()
        }
    }
}
