// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for CreateVpnConnectionRoute.</p>
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
pub struct CreateVpnConnectionRouteInput {
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    #[doc(hidden)]
    pub destination_cidr_block: std::option::Option<std::string::String>,
    /// <p>The ID of the VPN connection.</p>
    #[doc(hidden)]
    pub vpn_connection_id: std::option::Option<std::string::String>,
}
impl CreateVpnConnectionRouteInput {
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    pub fn destination_cidr_block(&self) -> std::option::Option<&str> {
        self.destination_cidr_block.as_deref()
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn vpn_connection_id(&self) -> std::option::Option<&str> {
        self.vpn_connection_id.as_deref()
    }
}
impl CreateVpnConnectionRouteInput {
    /// Creates a new builder-style object to manufacture [`CreateVpnConnectionRouteInput`](crate::operation::create_vpn_connection_route::CreateVpnConnectionRouteInput).
    pub fn builder(
    ) -> crate::operation::create_vpn_connection_route::builders::CreateVpnConnectionRouteInputBuilder
    {
        crate::operation::create_vpn_connection_route::builders::CreateVpnConnectionRouteInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_vpn_connection_route::CreateVpnConnectionRouteInput;
/// A builder for [`CreateVpnConnectionRouteInput`](crate::operation::create_vpn_connection_route::CreateVpnConnectionRouteInput).
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
pub struct CreateVpnConnectionRouteInputBuilder {
    pub(crate) destination_cidr_block: std::option::Option<std::string::String>,
    pub(crate) vpn_connection_id: std::option::Option<std::string::String>,
}
impl CreateVpnConnectionRouteInputBuilder {
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    pub fn destination_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination_cidr_block = Some(input.into());
        self
    }
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    pub fn set_destination_cidr_block(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.destination_cidr_block = input;
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn vpn_connection_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpn_connection_id = Some(input.into());
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn set_vpn_connection_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.vpn_connection_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateVpnConnectionRouteInput`](crate::operation::create_vpn_connection_route::CreateVpnConnectionRouteInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_vpn_connection_route::CreateVpnConnectionRouteInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_vpn_connection_route::CreateVpnConnectionRouteInput {
                destination_cidr_block: self.destination_cidr_block,
                vpn_connection_id: self.vpn_connection_id,
            },
        )
    }
}
