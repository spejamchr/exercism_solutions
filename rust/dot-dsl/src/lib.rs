pub mod graph {
    type Attrs = std::collections::HashMap<String, String>;

    fn attrs_from_arr(arr: &[(&str, &str)]) -> Attrs {
        arr.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: Attrs,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: Attrs::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs_arr: &[(&str, &str)]) -> Self {
            self.attrs = attrs_from_arr(attrs_arr);
            self
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

                pub fn with_attrs(mut self, attrs_arr: &[(&str, &str)]) -> Self {
                    self.attrs = super::super::attrs_from_arr(attrs_arr);
                    self
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

                pub fn with_attrs(mut self, attrs_arr: &[(&str, &str)]) -> Self {
                    self.attrs = super::super::attrs_from_arr(attrs_arr);
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| &s[..])
                }
            }
        }
    }
}
