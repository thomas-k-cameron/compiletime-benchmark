// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the elastic network interface for a multi-node parallel job node.</p>
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
pub struct NetworkInterface {
    /// <p>The attachment ID for the network interface.</p>
    #[doc(hidden)]
    pub attachment_id: std::option::Option<std::string::String>,
    /// <p>The private IPv6 address for the network interface.</p>
    #[doc(hidden)]
    pub ipv6_address: std::option::Option<std::string::String>,
    /// <p>The private IPv4 address for the network interface.</p>
    #[doc(hidden)]
    pub private_ipv4_address: std::option::Option<std::string::String>,
}
impl NetworkInterface {
    /// <p>The attachment ID for the network interface.</p>
    pub fn attachment_id(&self) -> std::option::Option<&str> {
        self.attachment_id.as_deref()
    }
    /// <p>The private IPv6 address for the network interface.</p>
    pub fn ipv6_address(&self) -> std::option::Option<&str> {
        self.ipv6_address.as_deref()
    }
    /// <p>The private IPv4 address for the network interface.</p>
    pub fn private_ipv4_address(&self) -> std::option::Option<&str> {
        self.private_ipv4_address.as_deref()
    }
}
impl NetworkInterface {
    /// Creates a new builder-style object to manufacture [`NetworkInterface`](crate::types::NetworkInterface).
    pub fn builder() -> crate::types::builders::NetworkInterfaceBuilder {
        crate::types::builders::NetworkInterfaceBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::NetworkInterface;
/// A builder for [`NetworkInterface`](crate::types::NetworkInterface).
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
pub struct NetworkInterfaceBuilder {
    pub(crate) attachment_id: std::option::Option<std::string::String>,
    pub(crate) ipv6_address: std::option::Option<std::string::String>,
    pub(crate) private_ipv4_address: std::option::Option<std::string::String>,
}
impl NetworkInterfaceBuilder {
    /// <p>The attachment ID for the network interface.</p>
    pub fn attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.attachment_id = Some(input.into());
        self
    }
    /// <p>The attachment ID for the network interface.</p>
    pub fn set_attachment_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.attachment_id = input;
        self
    }
    /// <p>The private IPv6 address for the network interface.</p>
    pub fn ipv6_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipv6_address = Some(input.into());
        self
    }
    /// <p>The private IPv6 address for the network interface.</p>
    pub fn set_ipv6_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ipv6_address = input;
        self
    }
    /// <p>The private IPv4 address for the network interface.</p>
    pub fn private_ipv4_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.private_ipv4_address = Some(input.into());
        self
    }
    /// <p>The private IPv4 address for the network interface.</p>
    pub fn set_private_ipv4_address(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.private_ipv4_address = input;
        self
    }
    /// Consumes the builder and constructs a [`NetworkInterface`](crate::types::NetworkInterface).
    pub fn build(self) -> crate::types::NetworkInterface {
        crate::types::NetworkInterface {
            attachment_id: self.attachment_id,
            ipv6_address: self.ipv6_address,
            private_ipv4_address: self.private_ipv4_address,
        }
    }
}
