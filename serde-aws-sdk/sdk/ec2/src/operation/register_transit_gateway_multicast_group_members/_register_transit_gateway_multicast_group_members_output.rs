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
pub struct RegisterTransitGatewayMulticastGroupMembersOutput {
    /// <p>Information about the registered transit gateway multicast group members.</p>
    #[doc(hidden)]
    pub registered_multicast_group_members:
        std::option::Option<crate::types::TransitGatewayMulticastRegisteredGroupMembers>,
    _request_id: Option<String>,
}
impl RegisterTransitGatewayMulticastGroupMembersOutput {
    /// <p>Information about the registered transit gateway multicast group members.</p>
    pub fn registered_multicast_group_members(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayMulticastRegisteredGroupMembers> {
        self.registered_multicast_group_members.as_ref()
    }
}
impl aws_http::request_id::RequestId for RegisterTransitGatewayMulticastGroupMembersOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RegisterTransitGatewayMulticastGroupMembersOutput {
    /// Creates a new builder-style object to manufacture [`RegisterTransitGatewayMulticastGroupMembersOutput`](crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersOutput).
    pub fn builder() -> crate::operation::register_transit_gateway_multicast_group_members::builders::RegisterTransitGatewayMulticastGroupMembersOutputBuilder{
        crate::operation::register_transit_gateway_multicast_group_members::builders::RegisterTransitGatewayMulticastGroupMembersOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersOutput;
/// A builder for [`RegisterTransitGatewayMulticastGroupMembersOutput`](crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersOutput).
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
pub struct RegisterTransitGatewayMulticastGroupMembersOutputBuilder {
    pub(crate) registered_multicast_group_members:
        std::option::Option<crate::types::TransitGatewayMulticastRegisteredGroupMembers>,
    _request_id: Option<String>,
}
impl RegisterTransitGatewayMulticastGroupMembersOutputBuilder {
    /// <p>Information about the registered transit gateway multicast group members.</p>
    pub fn registered_multicast_group_members(
        mut self,
        input: crate::types::TransitGatewayMulticastRegisteredGroupMembers,
    ) -> Self {
        self.registered_multicast_group_members = Some(input);
        self
    }
    /// <p>Information about the registered transit gateway multicast group members.</p>
    pub fn set_registered_multicast_group_members(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayMulticastRegisteredGroupMembers>,
    ) -> Self {
        self.registered_multicast_group_members = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RegisterTransitGatewayMulticastGroupMembersOutput`](crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersOutput).
    pub fn build(self) -> crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersOutput{
        crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersOutput {
            registered_multicast_group_members: self.registered_multicast_group_members
            ,
            _request_id: self._request_id,
        }
    }
}
