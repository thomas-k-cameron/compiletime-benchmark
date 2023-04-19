// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the deregistered transit gateway multicast group sources.</p>
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
pub struct TransitGatewayMulticastDeregisteredGroupSources {
    /// <p>The ID of the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    /// <p>The network interface IDs of the non-registered members.</p>
    #[doc(hidden)]
    pub deregistered_network_interface_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    #[doc(hidden)]
    pub group_ip_address: std::option::Option<std::string::String>,
}
impl TransitGatewayMulticastDeregisteredGroupSources {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_multicast_domain_id.as_deref()
    }
    /// <p>The network interface IDs of the non-registered members.</p>
    pub fn deregistered_network_interface_ids(
        &self,
    ) -> std::option::Option<&[std::string::String]> {
        self.deregistered_network_interface_ids.as_deref()
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn group_ip_address(&self) -> std::option::Option<&str> {
        self.group_ip_address.as_deref()
    }
}
impl TransitGatewayMulticastDeregisteredGroupSources {
    /// Creates a new builder-style object to manufacture [`TransitGatewayMulticastDeregisteredGroupSources`](crate::types::TransitGatewayMulticastDeregisteredGroupSources).
    pub fn builder(
    ) -> crate::types::builders::TransitGatewayMulticastDeregisteredGroupSourcesBuilder {
        crate::types::builders::TransitGatewayMulticastDeregisteredGroupSourcesBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TransitGatewayMulticastDeregisteredGroupSources;
/// A builder for [`TransitGatewayMulticastDeregisteredGroupSources`](crate::types::TransitGatewayMulticastDeregisteredGroupSources).
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
pub struct TransitGatewayMulticastDeregisteredGroupSourcesBuilder {
    pub(crate) transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    pub(crate) deregistered_network_interface_ids:
        std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) group_ip_address: std::option::Option<std::string::String>,
}
impl TransitGatewayMulticastDeregisteredGroupSourcesBuilder {
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
    /// Appends an item to `deregistered_network_interface_ids`.
    ///
    /// To override the contents of this collection use [`set_deregistered_network_interface_ids`](Self::set_deregistered_network_interface_ids).
    ///
    /// <p>The network interface IDs of the non-registered members.</p>
    pub fn deregistered_network_interface_ids(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        let mut v = self.deregistered_network_interface_ids.unwrap_or_default();
        v.push(input.into());
        self.deregistered_network_interface_ids = Some(v);
        self
    }
    /// <p>The network interface IDs of the non-registered members.</p>
    pub fn set_deregistered_network_interface_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.deregistered_network_interface_ids = input;
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
    /// Consumes the builder and constructs a [`TransitGatewayMulticastDeregisteredGroupSources`](crate::types::TransitGatewayMulticastDeregisteredGroupSources).
    pub fn build(self) -> crate::types::TransitGatewayMulticastDeregisteredGroupSources {
        crate::types::TransitGatewayMulticastDeregisteredGroupSources {
            transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id,
            deregistered_network_interface_ids: self.deregistered_network_interface_ids,
            group_ip_address: self.group_ip_address,
        }
    }
}
