// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a path statement.</p>
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
pub struct PathStatement {
    /// <p>The packet header statement.</p>
    #[doc(hidden)]
    pub packet_header_statement: std::option::Option<crate::types::PacketHeaderStatement>,
    /// <p>The resource statement.</p>
    #[doc(hidden)]
    pub resource_statement: std::option::Option<crate::types::ResourceStatement>,
}
impl PathStatement {
    /// <p>The packet header statement.</p>
    pub fn packet_header_statement(
        &self,
    ) -> std::option::Option<&crate::types::PacketHeaderStatement> {
        self.packet_header_statement.as_ref()
    }
    /// <p>The resource statement.</p>
    pub fn resource_statement(&self) -> std::option::Option<&crate::types::ResourceStatement> {
        self.resource_statement.as_ref()
    }
}
impl PathStatement {
    /// Creates a new builder-style object to manufacture [`PathStatement`](crate::types::PathStatement).
    pub fn builder() -> crate::types::builders::PathStatementBuilder {
        crate::types::builders::PathStatementBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PathStatement;
/// A builder for [`PathStatement`](crate::types::PathStatement).
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
pub struct PathStatementBuilder {
    pub(crate) packet_header_statement: std::option::Option<crate::types::PacketHeaderStatement>,
    pub(crate) resource_statement: std::option::Option<crate::types::ResourceStatement>,
}
impl PathStatementBuilder {
    /// <p>The packet header statement.</p>
    pub fn packet_header_statement(mut self, input: crate::types::PacketHeaderStatement) -> Self {
        self.packet_header_statement = Some(input);
        self
    }
    /// <p>The packet header statement.</p>
    pub fn set_packet_header_statement(
        mut self,
        input: std::option::Option<crate::types::PacketHeaderStatement>,
    ) -> Self {
        self.packet_header_statement = input;
        self
    }
    /// <p>The resource statement.</p>
    pub fn resource_statement(mut self, input: crate::types::ResourceStatement) -> Self {
        self.resource_statement = Some(input);
        self
    }
    /// <p>The resource statement.</p>
    pub fn set_resource_statement(
        mut self,
        input: std::option::Option<crate::types::ResourceStatement>,
    ) -> Self {
        self.resource_statement = input;
        self
    }
    /// Consumes the builder and constructs a [`PathStatement`](crate::types::PathStatement).
    pub fn build(self) -> crate::types::PathStatement {
        crate::types::PathStatement {
            packet_header_statement: self.packet_header_statement,
            resource_statement: self.resource_statement,
        }
    }
}
