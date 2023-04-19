// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the properties of a global table.</p>
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
pub struct GlobalTable {
    /// <p>The global table name.</p>
    #[doc(hidden)]
    pub global_table_name: std::option::Option<std::string::String>,
    /// <p>The Regions where the global table has replicas.</p>
    #[doc(hidden)]
    pub replication_group: std::option::Option<std::vec::Vec<crate::types::Replica>>,
}
impl GlobalTable {
    /// <p>The global table name.</p>
    pub fn global_table_name(&self) -> std::option::Option<&str> {
        self.global_table_name.as_deref()
    }
    /// <p>The Regions where the global table has replicas.</p>
    pub fn replication_group(&self) -> std::option::Option<&[crate::types::Replica]> {
        self.replication_group.as_deref()
    }
}
impl GlobalTable {
    /// Creates a new builder-style object to manufacture [`GlobalTable`](crate::types::GlobalTable).
    pub fn builder() -> crate::types::builders::GlobalTableBuilder {
        crate::types::builders::GlobalTableBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::GlobalTable;
/// A builder for [`GlobalTable`](crate::types::GlobalTable).
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
pub struct GlobalTableBuilder {
    pub(crate) global_table_name: std::option::Option<std::string::String>,
    pub(crate) replication_group: std::option::Option<std::vec::Vec<crate::types::Replica>>,
}
impl GlobalTableBuilder {
    /// <p>The global table name.</p>
    pub fn global_table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.global_table_name = Some(input.into());
        self
    }
    /// <p>The global table name.</p>
    pub fn set_global_table_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.global_table_name = input;
        self
    }
    /// Appends an item to `replication_group`.
    ///
    /// To override the contents of this collection use [`set_replication_group`](Self::set_replication_group).
    ///
    /// <p>The Regions where the global table has replicas.</p>
    pub fn replication_group(mut self, input: crate::types::Replica) -> Self {
        let mut v = self.replication_group.unwrap_or_default();
        v.push(input);
        self.replication_group = Some(v);
        self
    }
    /// <p>The Regions where the global table has replicas.</p>
    pub fn set_replication_group(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Replica>>,
    ) -> Self {
        self.replication_group = input;
        self
    }
    /// Consumes the builder and constructs a [`GlobalTable`](crate::types::GlobalTable).
    pub fn build(self) -> crate::types::GlobalTable {
        crate::types::GlobalTable {
            global_table_name: self.global_table_name,
            replication_group: self.replication_group,
        }
    }
}
