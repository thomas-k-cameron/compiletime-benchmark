// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct DeregisterTransitGatewayMulticastGroupMembersInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    #[doc(hidden)]
    pub group_ip_address: std::option::Option<std::string::String>,
    /// <p>The IDs of the group members' network interfaces.</p>
    #[doc(hidden)]
    pub network_interface_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeregisterTransitGatewayMulticastGroupMembersInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_multicast_domain_id.as_deref()
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn group_ip_address(&self) -> std::option::Option<&str> {
        self.group_ip_address.as_deref()
    }
    /// <p>The IDs of the group members' network interfaces.</p>
    pub fn network_interface_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.network_interface_ids.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeregisterTransitGatewayMulticastGroupMembersInput {
    /// Creates a new builder-style object to manufacture [`DeregisterTransitGatewayMulticastGroupMembersInput`](crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersInput).
    pub fn builder() -> crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersInputBuilder{
        crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersInput;
/// A builder for [`DeregisterTransitGatewayMulticastGroupMembersInput`](crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersInput).
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
pub struct DeregisterTransitGatewayMulticastGroupMembersInputBuilder {
    pub(crate) transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    pub(crate) group_ip_address: std::option::Option<std::string::String>,
    pub(crate) network_interface_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeregisterTransitGatewayMulticastGroupMembersInputBuilder {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_id = Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_id = input;
        self
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn group_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.group_ip_address = Some(input.into());
        self
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn set_group_ip_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.group_ip_address = input;
        self
    }
    /// Appends an item to `network_interface_ids`.
    ///
    /// To override the contents of this collection use [`set_network_interface_ids`](Self::set_network_interface_ids).
    ///
    /// <p>The IDs of the group members' network interfaces.</p>
    pub fn network_interface_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.network_interface_ids.unwrap_or_default();
        v.push(input.into());
        self.network_interface_ids = Some(v);
        self
    }
    /// <p>The IDs of the group members' network interfaces.</p>
    pub fn set_network_interface_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.network_interface_ids = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DeregisterTransitGatewayMulticastGroupMembersInput`](crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersInput).
    pub fn build(self) -> Result<crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersInput {
                transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id
                ,
                group_ip_address: self.group_ip_address
                ,
                network_interface_ids: self.network_interface_ids
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}