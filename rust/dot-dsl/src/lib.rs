pub mod graph {
    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    use std::collections::HashMap;

    macro_rules! attributes {
        () => {
            pub fn attr(&self, key: &str) -> Option<&str> {
                self.attrs.get(key).map(|val| val.as_str())
            }

            pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                Self {
                    attrs: attrs
                        .iter()
                        .map(|(key, val)| (key.to_string(), val.to_string()))
                        .collect(),
                    ..self
                }
            }
        };
    }

    pub mod graph_items {
        use super::*;
        pub mod node {
            use super::*;

            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                attributes!();
            }
        }

        pub mod edge {
            use super::*;

            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct Edge {
                pub src: String,
                pub dst: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(src: &str, dst: &str) -> Self {
                    Self {
                        src: src.to_string(),
                        dst: dst.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                attributes!();
            }
        }
    }

    #[derive(Default, Eq, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Self {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }

        attributes!();
    }
}
