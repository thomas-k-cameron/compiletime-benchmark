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
pub struct PathStatementRequest {
    /// <p>The packet header statement.</p>
    #[doc(hidden)]
    pub packet_header_statement: std::option::Option<crate::types::PacketHeaderStatementRequest>,
    /// <p>The resource statement.</p>
    #[doc(hidden)]
    pub resource_statement: std::option::Option<crate::types::ResourceStatementRequest>,
}
impl PathStatementRequest {
    /// <p>The packet header statement.</p>
    pub fn packet_header_statement(
        &self,
    ) -> std::option::Option<&crate::types::PacketHeaderStatementRequest> {
        self.packet_header_statement.as_ref()
    }
    /// <p>The resource statement.</p>
    pub fn resource_statement(
        &self,
    ) -> std::option::Option<&crate::types::ResourceStatementRequest> {
        self.resource_statement.as_ref()
    }
}
impl PathStatementRequest {
    /// Creates a new builder-style object to manufacture [`PathStatementRequest`](crate::types::PathStatementRequest).
    pub fn builder() -> crate::types::builders::PathStatementRequestBuilder {
        crate::types::builders::PathStatementRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PathStatementRequest;
/// A builder for [`PathStatementRequest`](crate::types::PathStatementRequest).
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
pub struct PathStatementRequestBuilder {
    pub(crate) packet_header_statement:
        std::option::Option<crate::types::PacketHeaderStatementRequest>,
    pub(crate) resource_statement: std::option::Option<crate::types::ResourceStatementRequest>,
}
impl PathStatementRequestBuilder {
    /// <p>The packet header statement.</p>
    pub fn packet_header_statement(
        mut self,
        input: crate::types::PacketHeaderStatementRequest,
    ) -> Self {
        self.packet_header_statement = Some(input);
        self
    }
    /// <p>The packet header statement.</p>
    pub fn set_packet_header_statement(
        mut self,
        input: std::option::Option<crate::types::PacketHeaderStatementRequest>,
    ) -> Self {
        self.packet_header_statement = input;
        self
    }
    /// <p>The resource statement.</p>
    pub fn resource_statement(mut self, input: crate::types::ResourceStatementRequest) -> Self {
        self.resource_statement = Some(input);
        self
    }
    /// <p>The resource statement.</p>
    pub fn set_resource_statement(
        mut self,
        input: std::option::Option<crate::types::ResourceStatementRequest>,
    ) -> Self {
        self.resource_statement = input;
        self
    }
    /// Consumes the builder and constructs a [`PathStatementRequest`](crate::types::PathStatementRequest).
    pub fn build(self) -> crate::types::PathStatementRequest {
        crate::types::PathStatementRequest {
            packet_header_statement: self.packet_header_statement,
            resource_statement: self.resource_statement,
        }
    }
}
