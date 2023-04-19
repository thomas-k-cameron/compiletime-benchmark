// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the private IPv4 address of a network interface.</p>
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
pub struct NetworkInterfacePrivateIpAddress {
    /// <p>The association information for an Elastic IP address (IPv4) associated with the network interface.</p>
    #[doc(hidden)]
    pub association: std::option::Option<crate::types::NetworkInterfaceAssociation>,
    /// <p>Indicates whether this IPv4 address is the primary private IPv4 address of the network interface.</p>
    #[doc(hidden)]
    pub primary: std::option::Option<bool>,
    /// <p>The private DNS name.</p>
    #[doc(hidden)]
    pub private_dns_name: std::option::Option<std::string::String>,
    /// <p>The private IPv4 address.</p>
    #[doc(hidden)]
    pub private_ip_address: std::option::Option<std::string::String>,
}
impl NetworkInterfacePrivateIpAddress {
    /// <p>The association information for an Elastic IP address (IPv4) associated with the network interface.</p>
    pub fn association(&self) -> std::option::Option<&crate::types::NetworkInterfaceAssociation> {
        self.association.as_ref()
    }
    /// <p>Indicates whether this IPv4 address is the primary private IPv4 address of the network interface.</p>
    pub fn primary(&self) -> std::option::Option<bool> {
        self.primary
    }
    /// <p>The private DNS name.</p>
    pub fn private_dns_name(&self) -> std::option::Option<&str> {
        self.private_dns_name.as_deref()
    }
    /// <p>The private IPv4 address.</p>
    pub fn private_ip_address(&self) -> std::option::Option<&str> {
        self.private_ip_address.as_deref()
    }
}
impl NetworkInterfacePrivateIpAddress {
    /// Creates a new builder-style object to manufacture [`NetworkInterfacePrivateIpAddress`](crate::types::NetworkInterfacePrivateIpAddress).
    pub fn builder() -> crate::types::builders::NetworkInterfacePrivateIpAddressBuilder {
        crate::types::builders::NetworkInterfacePrivateIpAddressBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::NetworkInterfacePrivateIpAddress;
/// A builder for [`NetworkInterfacePrivateIpAddress`](crate::types::NetworkInterfacePrivateIpAddress).
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
pub struct NetworkInterfacePrivateIpAddressBuilder {
    pub(crate) association: std::option::Option<crate::types::NetworkInterfaceAssociation>,
    pub(crate) primary: std::option::Option<bool>,
    pub(crate) private_dns_name: std::option::Option<std::string::String>,
    pub(crate) private_ip_address: std::option::Option<std::string::String>,
}
impl NetworkInterfacePrivateIpAddressBuilder {
    /// <p>The association information for an Elastic IP address (IPv4) associated with the network interface.</p>
    pub fn association(mut self, input: crate::types::NetworkInterfaceAssociation) -> Self {
        self.association = Some(input);
        self
    }
    /// <p>The association information for an Elastic IP address (IPv4) associated with the network interface.</p>
    pub fn set_association(
        mut self,
        input: std::option::Option<crate::types::NetworkInterfaceAssociation>,
    ) -> Self {
        self.association = input;
        self
    }
    /// <p>Indicates whether this IPv4 address is the primary private IPv4 address of the network interface.</p>
    pub fn primary(mut self, input: bool) -> Self {
        self.primary = Some(input);
        self
    }
    /// <p>Indicates whether this IPv4 address is the primary private IPv4 address of the network interface.</p>
    pub fn set_primary(mut self, input: std::option::Option<bool>) -> Self {
        self.primary = input;
        self
    }
    /// <p>The private DNS name.</p>
    pub fn private_dns_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.private_dns_name = Some(input.into());
        self
    }
    /// <p>The private DNS name.</p>
    pub fn set_private_dns_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.private_dns_name = input;
        self
    }
    /// <p>The private IPv4 address.</p>
    pub fn private_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.private_ip_address = Some(input.into());
        self
    }
    /// <p>The private IPv4 address.</p>
    pub fn set_private_ip_address(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.private_ip_address = input;
        self
    }
    /// Consumes the builder and constructs a [`NetworkInterfacePrivateIpAddress`](crate::types::NetworkInterfacePrivateIpAddress).
    pub fn build(self) -> crate::types::NetworkInterfacePrivateIpAddress {
        crate::types::NetworkInterfacePrivateIpAddress {
            association: self.association,
            primary: self.primary,
            private_dns_name: self.private_dns_name,
            private_ip_address: self.private_ip_address,
        }
    }
}
