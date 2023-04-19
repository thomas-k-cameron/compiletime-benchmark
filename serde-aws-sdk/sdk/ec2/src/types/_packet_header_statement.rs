// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a packet header statement.</p>
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
pub struct PacketHeaderStatement {
    /// <p>The source addresses.</p>
    #[doc(hidden)]
    pub source_addresses: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The destination addresses.</p>
    #[doc(hidden)]
    pub destination_addresses: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The source ports.</p>
    #[doc(hidden)]
    pub source_ports: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The destination ports.</p>
    #[doc(hidden)]
    pub destination_ports: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The source prefix lists.</p>
    #[doc(hidden)]
    pub source_prefix_lists: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The destination prefix lists.</p>
    #[doc(hidden)]
    pub destination_prefix_lists: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The protocols.</p>
    #[doc(hidden)]
    pub protocols: std::option::Option<std::vec::Vec<crate::types::Protocol>>,
}
impl PacketHeaderStatement {
    /// <p>The source addresses.</p>
    pub fn source_addresses(&self) -> std::option::Option<&[std::string::String]> {
        self.source_addresses.as_deref()
    }
    /// <p>The destination addresses.</p>
    pub fn destination_addresses(&self) -> std::option::Option<&[std::string::String]> {
        self.destination_addresses.as_deref()
    }
    /// <p>The source ports.</p>
    pub fn source_ports(&self) -> std::option::Option<&[std::string::String]> {
        self.source_ports.as_deref()
    }
    /// <p>The destination ports.</p>
    pub fn destination_ports(&self) -> std::option::Option<&[std::string::String]> {
        self.destination_ports.as_deref()
    }
    /// <p>The source prefix lists.</p>
    pub fn source_prefix_lists(&self) -> std::option::Option<&[std::string::String]> {
        self.source_prefix_lists.as_deref()
    }
    /// <p>The destination prefix lists.</p>
    pub fn destination_prefix_lists(&self) -> std::option::Option<&[std::string::String]> {
        self.destination_prefix_lists.as_deref()
    }
    /// <p>The protocols.</p>
    pub fn protocols(&self) -> std::option::Option<&[crate::types::Protocol]> {
        self.protocols.as_deref()
    }
}
impl PacketHeaderStatement {
    /// Creates a new builder-style object to manufacture [`PacketHeaderStatement`](crate::types::PacketHeaderStatement).
    pub fn builder() -> crate::types::builders::PacketHeaderStatementBuilder {
        crate::types::builders::PacketHeaderStatementBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PacketHeaderStatement;
/// A builder for [`PacketHeaderStatement`](crate::types::PacketHeaderStatement).
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
pub struct PacketHeaderStatementBuilder {
    pub(crate) source_addresses: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) destination_addresses: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) source_ports: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) destination_ports: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) source_prefix_lists: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) destination_prefix_lists: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) protocols: std::option::Option<std::vec::Vec<crate::types::Protocol>>,
}
impl PacketHeaderStatementBuilder {
    /// Appends an item to `source_addresses`.
    ///
    /// To override the contents of this collection use [`set_source_addresses`](Self::set_source_addresses).
    ///
    /// <p>The source addresses.</p>
    pub fn source_addresses(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.source_addresses.unwrap_or_default();
        v.push(input.into());
        self.source_addresses = Some(v);
        self
    }
    /// <p>The source addresses.</p>
    pub fn set_source_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.source_addresses = input;
        self
    }
    /// Appends an item to `destination_addresses`.
    ///
    /// To override the contents of this collection use [`set_destination_addresses`](Self::set_destination_addresses).
    ///
    /// <p>The destination addresses.</p>
    pub fn destination_addresses(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.destination_addresses.unwrap_or_default();
        v.push(input.into());
        self.destination_addresses = Some(v);
        self
    }
    /// <p>The destination addresses.</p>
    pub fn set_destination_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.destination_addresses = input;
        self
    }
    /// Appends an item to `source_ports`.
    ///
    /// To override the contents of this collection use [`set_source_ports`](Self::set_source_ports).
    ///
    /// <p>The source ports.</p>
    pub fn source_ports(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.source_ports.unwrap_or_default();
        v.push(input.into());
        self.source_ports = Some(v);
        self
    }
    /// <p>The source ports.</p>
    pub fn set_source_ports(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.source_ports = input;
        self
    }
    /// Appends an item to `destination_ports`.
    ///
    /// To override the contents of this collection use [`set_destination_ports`](Self::set_destination_ports).
    ///
    /// <p>The destination ports.</p>
    pub fn destination_ports(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.destination_ports.unwrap_or_default();
        v.push(input.into());
        self.destination_ports = Some(v);
        self
    }
    /// <p>The destination ports.</p>
    pub fn set_destination_ports(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.destination_ports = input;
        self
    }
    /// Appends an item to `source_prefix_lists`.
    ///
    /// To override the contents of this collection use [`set_source_prefix_lists`](Self::set_source_prefix_lists).
    ///
    /// <p>The source prefix lists.</p>
    pub fn source_prefix_lists(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.source_prefix_lists.unwrap_or_default();
        v.push(input.into());
        self.source_prefix_lists = Some(v);
        self
    }
    /// <p>The source prefix lists.</p>
    pub fn set_source_prefix_lists(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.source_prefix_lists = input;
        self
    }
    /// Appends an item to `destination_prefix_lists`.
    ///
    /// To override the contents of this collection use [`set_destination_prefix_lists`](Self::set_destination_prefix_lists).
    ///
    /// <p>The destination prefix lists.</p>
    pub fn destination_prefix_lists(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.destination_prefix_lists.unwrap_or_default();
        v.push(input.into());
        self.destination_prefix_lists = Some(v);
        self
    }
    /// <p>The destination prefix lists.</p>
    pub fn set_destination_prefix_lists(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.destination_prefix_lists = input;
        self
    }
    /// Appends an item to `protocols`.
    ///
    /// To override the contents of this collection use [`set_protocols`](Self::set_protocols).
    ///
    /// <p>The protocols.</p>
    pub fn protocols(mut self, input: crate::types::Protocol) -> Self {
        let mut v = self.protocols.unwrap_or_default();
        v.push(input);
        self.protocols = Some(v);
        self
    }
    /// <p>The protocols.</p>
    pub fn set_protocols(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Protocol>>,
    ) -> Self {
        self.protocols = input;
        self
    }
    /// Consumes the builder and constructs a [`PacketHeaderStatement`](crate::types::PacketHeaderStatement).
    pub fn build(self) -> crate::types::PacketHeaderStatement {
        crate::types::PacketHeaderStatement {
            source_addresses: self.source_addresses,
            destination_addresses: self.destination_addresses,
            source_ports: self.source_ports,
            destination_ports: self.destination_ports,
            source_prefix_lists: self.source_prefix_lists,
            destination_prefix_lists: self.destination_prefix_lists,
            protocols: self.protocols,
        }
    }
}
