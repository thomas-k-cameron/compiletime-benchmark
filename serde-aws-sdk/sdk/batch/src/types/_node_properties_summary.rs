// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the properties of a node that's associated with a multi-node parallel job.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct NodePropertiesSummary {
    /// <p>Specifies whether the current node is the main node for a multi-node parallel job.</p>
    #[doc(hidden)]
    pub is_main_node: std::option::Option<bool>,
    /// <p>The number of nodes that are associated with a multi-node parallel job.</p>
    #[doc(hidden)]
    pub num_nodes: std::option::Option<i32>,
    /// <p>The node index for the node. Node index numbering begins at zero. This index is also available on the node with the <code>AWS_BATCH_JOB_NODE_INDEX</code> environment variable.</p>
    #[doc(hidden)]
    pub node_index: std::option::Option<i32>,
}
impl NodePropertiesSummary {
    /// <p>Specifies whether the current node is the main node for a multi-node parallel job.</p>
    pub fn is_main_node(&self) -> std::option::Option<bool> {
        self.is_main_node
    }
    /// <p>The number of nodes that are associated with a multi-node parallel job.</p>
    pub fn num_nodes(&self) -> std::option::Option<i32> {
        self.num_nodes
    }
    /// <p>The node index for the node. Node index numbering begins at zero. This index is also available on the node with the <code>AWS_BATCH_JOB_NODE_INDEX</code> environment variable.</p>
    pub fn node_index(&self) -> std::option::Option<i32> {
        self.node_index
    }
}
impl NodePropertiesSummary {
    /// Creates a new builder-style object to manufacture [`NodePropertiesSummary`](crate::types::NodePropertiesSummary).
    pub fn builder() -> crate::types::builders::NodePropertiesSummaryBuilder {
        crate::types::builders::NodePropertiesSummaryBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::NodePropertiesSummary;
/// A builder for [`NodePropertiesSummary`](crate::types::NodePropertiesSummary).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct NodePropertiesSummaryBuilder {
    pub(crate) is_main_node: std::option::Option<bool>,
    pub(crate) num_nodes: std::option::Option<i32>,
    pub(crate) node_index: std::option::Option<i32>,
}
impl NodePropertiesSummaryBuilder {
    /// <p>Specifies whether the current node is the main node for a multi-node parallel job.</p>
    pub fn is_main_node(mut self, input: bool) -> Self {
        self.is_main_node = Some(input);
        self
    }
    /// <p>Specifies whether the current node is the main node for a multi-node parallel job.</p>
    pub fn set_is_main_node(mut self, input: std::option::Option<bool>) -> Self {
        self.is_main_node = input;
        self
    }
    /// <p>The number of nodes that are associated with a multi-node parallel job.</p>
    pub fn num_nodes(mut self, input: i32) -> Self {
        self.num_nodes = Some(input);
        self
    }
    /// <p>The number of nodes that are associated with a multi-node parallel job.</p>
    pub fn set_num_nodes(mut self, input: std::option::Option<i32>) -> Self {
        self.num_nodes = input;
        self
    }
    /// <p>The node index for the node. Node index numbering begins at zero. This index is also available on the node with the <code>AWS_BATCH_JOB_NODE_INDEX</code> environment variable.</p>
    pub fn node_index(mut self, input: i32) -> Self {
        self.node_index = Some(input);
        self
    }
    /// <p>The node index for the node. Node index numbering begins at zero. This index is also available on the node with the <code>AWS_BATCH_JOB_NODE_INDEX</code> environment variable.</p>
    pub fn set_node_index(mut self, input: std::option::Option<i32>) -> Self {
        self.node_index = input;
        self
    }
    /// Consumes the builder and constructs a [`NodePropertiesSummary`](crate::types::NodePropertiesSummary).
    pub fn build(self) -> crate::types::NodePropertiesSummary {
        crate::types::NodePropertiesSummary {
            is_main_node: self.is_main_node,
            num_nodes: self.num_nodes,
            node_index: self.node_index,
        }
    }
}