// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the IP addresses and network interface associated with a NAT gateway.</p>
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
pub struct NatGatewayAddress {
    /// <p>[Public NAT gateway only] The allocation ID of the Elastic IP address that's associated with the NAT gateway.</p>
    #[doc(hidden)]
    pub allocation_id: std::option::Option<std::string::String>,
    /// <p>The ID of the network interface associated with the NAT gateway.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>The private IP address associated with the NAT gateway.</p>
    #[doc(hidden)]
    pub private_ip: std::option::Option<std::string::String>,
    /// <p>[Public NAT gateway only] The Elastic IP address associated with the NAT gateway.</p>
    #[doc(hidden)]
    pub public_ip: std::option::Option<std::string::String>,
    /// <p>[Public NAT gateway only] The association ID of the Elastic IP address that's associated with the NAT gateway.</p>
    #[doc(hidden)]
    pub association_id: std::option::Option<std::string::String>,
    /// <p>Defines if the IP address is the primary address.</p>
    #[doc(hidden)]
    pub is_primary: std::option::Option<bool>,
    /// <p>The address failure message.</p>
    #[doc(hidden)]
    pub failure_message: std::option::Option<std::string::String>,
    /// <p>The address status.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::NatGatewayAddressStatus>,
}
impl NatGatewayAddress {
    /// <p>[Public NAT gateway only] The allocation ID of the Elastic IP address that's associated with the NAT gateway.</p>
    pub fn allocation_id(&self) -> std::option::Option<&str> {
        self.allocation_id.as_deref()
    }
    /// <p>The ID of the network interface associated with the NAT gateway.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The private IP address associated with the NAT gateway.</p>
    pub fn private_ip(&self) -> std::option::Option<&str> {
        self.private_ip.as_deref()
    }
    /// <p>[Public NAT gateway only] The Elastic IP address associated with the NAT gateway.</p>
    pub fn public_ip(&self) -> std::option::Option<&str> {
        self.public_ip.as_deref()
    }
    /// <p>[Public NAT gateway only] The association ID of the Elastic IP address that's associated with the NAT gateway.</p>
    pub fn association_id(&self) -> std::option::Option<&str> {
        self.association_id.as_deref()
    }
    /// <p>Defines if the IP address is the primary address.</p>
    pub fn is_primary(&self) -> std::option::Option<bool> {
        self.is_primary
    }
    /// <p>The address failure message.</p>
    pub fn failure_message(&self) -> std::option::Option<&str> {
        self.failure_message.as_deref()
    }
    /// <p>The address status.</p>
    pub fn status(&self) -> std::option::Option<&crate::types::NatGatewayAddressStatus> {
        self.status.as_ref()
    }
}
impl NatGatewayAddress {
    /// Creates a new builder-style object to manufacture [`NatGatewayAddress`](crate::types::NatGatewayAddress).
    pub fn builder() -> crate::types::builders::NatGatewayAddressBuilder {
        crate::types::builders::NatGatewayAddressBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::NatGatewayAddress;
/// A builder for [`NatGatewayAddress`](crate::types::NatGatewayAddress).
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
pub struct NatGatewayAddressBuilder {
    pub(crate) allocation_id: std::option::Option<std::string::String>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) private_ip: std::option::Option<std::string::String>,
    pub(crate) public_ip: std::option::Option<std::string::String>,
    pub(crate) association_id: std::option::Option<std::string::String>,
    pub(crate) is_primary: std::option::Option<bool>,
    pub(crate) failure_message: std::option::Option<std::string::String>,
    pub(crate) status: std::option::Option<crate::types::NatGatewayAddressStatus>,
}
impl NatGatewayAddressBuilder {
    /// <p>[Public NAT gateway only] The allocation ID of the Elastic IP address that's associated with the NAT gateway.</p>
    pub fn allocation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.allocation_id = Some(input.into());
        self
    }
    /// <p>[Public NAT gateway only] The allocation ID of the Elastic IP address that's associated with the NAT gateway.</p>
    pub fn set_allocation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.allocation_id = input;
        self
    }
    /// <p>The ID of the network interface associated with the NAT gateway.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_id = Some(input.into());
        self
    }
    /// <p>The ID of the network interface associated with the NAT gateway.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>The private IP address associated with the NAT gateway.</p>
    pub fn private_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.private_ip = Some(input.into());
        self
    }
    /// <p>The private IP address associated with the NAT gateway.</p>
    pub fn set_private_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.private_ip = input;
        self
    }
    /// <p>[Public NAT gateway only] The Elastic IP address associated with the NAT gateway.</p>
    pub fn public_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.public_ip = Some(input.into());
        self
    }
    /// <p>[Public NAT gateway only] The Elastic IP address associated with the NAT gateway.</p>
    pub fn set_public_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.public_ip = input;
        self
    }
    /// <p>[Public NAT gateway only] The association ID of the Elastic IP address that's associated with the NAT gateway.</p>
    pub fn association_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.association_id = Some(input.into());
        self
    }
    /// <p>[Public NAT gateway only] The association ID of the Elastic IP address that's associated with the NAT gateway.</p>
    pub fn set_association_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.association_id = input;
        self
    }
    /// <p>Defines if the IP address is the primary address.</p>
    pub fn is_primary(mut self, input: bool) -> Self {
        self.is_primary = Some(input);
        self
    }
    /// <p>Defines if the IP address is the primary address.</p>
    pub fn set_is_primary(mut self, input: std::option::Option<bool>) -> Self {
        self.is_primary = input;
        self
    }
    /// <p>The address failure message.</p>
    pub fn failure_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.failure_message = Some(input.into());
        self
    }
    /// <p>The address failure message.</p>
    pub fn set_failure_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.failure_message = input;
        self
    }
    /// <p>The address status.</p>
    pub fn status(mut self, input: crate::types::NatGatewayAddressStatus) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The address status.</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<crate::types::NatGatewayAddressStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`NatGatewayAddress`](crate::types::NatGatewayAddress).
    pub fn build(self) -> crate::types::NatGatewayAddress {
        crate::types::NatGatewayAddress {
            allocation_id: self.allocation_id,
            network_interface_id: self.network_interface_id,
            private_ip: self.private_ip,
            public_ip: self.public_ip,
            association_id: self.association_id,
            is_primary: self.is_primary,
            failure_message: self.failure_message,
            status: self.status,
        }
    }
}
