pub mod graph {
    type Nodes = Vec<graph_items::node::Node>;
    type Edges = Vec<graph_items::edge::Edge>;
    type Attrs = std::collections::HashMap<String, String>;

    fn attrs_from_arr(arr: &[(&str, &str)]) -> Attrs {
        arr.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Nodes,
        pub edges: Edges,
        pub attrs: Attrs,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: Attrs::new(),
            }
        }

        pub fn with_nodes(self, nodes: &Nodes) -> Self {
            Self {
                nodes: nodes.to_vec(),
                edges: self.edges,
                attrs: self.attrs,
            }
        }

        pub fn with_edges(self, edges: &Edges) -> Self {
            Self {
                nodes: self.nodes,
                edges: edges.to_vec(),
                attrs: self.attrs,
            }
        }

        pub fn with_attrs(self, attrs_arr: &[(&str, &str)]) -> Self {
            Self {
                nodes: self.nodes,
                edges: self.edges,
                attrs: attrs_from_arr(attrs_arr),
            }
        }

        pub fn get_node(&self, id: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|n| n.id == id)
        }
    }

    pub mod graph_items {
        pub mod edge {
            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Edge {
                a: String,
                b: String,
                attrs: super::super::Attrs,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Self {
                        a: a.to_string(),
                        b: b.to_string(),
                        attrs: super::super::Attrs::new(),
                    }
                }

                pub fn with_attrs(self, attrs_arr: &[(&str, &str)]) -> Self {
                    Self {
                        a: self.a,
                        b: self.b,
                        attrs: super::super::attrs_from_arr(attrs_arr),
                    }
                }
            }
        }

        pub mod node {
            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Node {
                pub id: String,
                pub attrs: super::super::Attrs,
            }

            impl Node {
                pub fn new(id: &str) -> Self {
                    Self {
                        id: id.to_owned(),
                        attrs: super::super::Attrs::new(),
                    }
                }

                pub fn with_attrs(self, attrs_arr: &[(&str, &str)]) -> Self {
                    Self {
                        id: self.id,
                        attrs: super::super::attrs_from_arr(attrs_arr),
                    }
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| &s[..])
                }
            }
        }
    }
}
