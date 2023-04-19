// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the registered transit gateway multicast group members.</p>
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
pub struct TransitGatewayMulticastRegisteredGroupMembers {
    /// <p>The ID of the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    /// <p>The ID of the registered network interfaces.</p>
    #[doc(hidden)]
    pub registered_network_interface_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    #[doc(hidden)]
    pub group_ip_address: std::option::Option<std::string::String>,
}
impl TransitGatewayMulticastRegisteredGroupMembers {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_multicast_domain_id.as_deref()
    }
    /// <p>The ID of the registered network interfaces.</p>
    pub fn registered_network_interface_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.registered_network_interface_ids.as_deref()
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn group_ip_address(&self) -> std::option::Option<&str> {
        self.group_ip_address.as_deref()
    }
}
impl TransitGatewayMulticastRegisteredGroupMembers {
    /// Creates a new builder-style object to manufacture [`TransitGatewayMulticastRegisteredGroupMembers`](crate::types::TransitGatewayMulticastRegisteredGroupMembers).
    pub fn builder() -> crate::types::builders::TransitGatewayMulticastRegisteredGroupMembersBuilder
    {
        crate::types::builders::TransitGatewayMulticastRegisteredGroupMembersBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TransitGatewayMulticastRegisteredGroupMembers;
/// A builder for [`TransitGatewayMulticastRegisteredGroupMembers`](crate::types::TransitGatewayMulticastRegisteredGroupMembers).
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
pub struct TransitGatewayMulticastRegisteredGroupMembersBuilder {
    pub(crate) transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    pub(crate) registered_network_interface_ids:
        std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) group_ip_address: std::option::Option<std::string::String>,
}
impl TransitGatewayMulticastRegisteredGroupMembersBuilder {
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
    /// Appends an item to `registered_network_interface_ids`.
    ///
    /// To override the contents of this collection use [`set_registered_network_interface_ids`](Self::set_registered_network_interface_ids).
    ///
    /// <p>The ID of the registered network interfaces.</p>
    pub fn registered_network_interface_ids(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        let mut v = self.registered_network_interface_ids.unwrap_or_default();
        v.push(input.into());
        self.registered_network_interface_ids = Some(v);
        self
    }
    /// <p>The ID of the registered network interfaces.</p>
    pub fn set_registered_network_interface_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.registered_network_interface_ids = input;
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
    /// Consumes the builder and constructs a [`TransitGatewayMulticastRegisteredGroupMembers`](crate::types::TransitGatewayMulticastRegisteredGroupMembers).
    pub fn build(self) -> crate::types::TransitGatewayMulticastRegisteredGroupMembers {
        crate::types::TransitGatewayMulticastRegisteredGroupMembers {
            transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id,
            registered_network_interface_ids: self.registered_network_interface_ids,
            group_ip_address: self.group_ip_address,
        }
    }
}
