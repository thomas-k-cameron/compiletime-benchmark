// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeregisterTransitGatewayMulticastGroupMembers`](crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_multicast_domain_id(impl Into<String>)`](crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder::transit_gateway_multicast_domain_id) / [`set_transit_gateway_multicast_domain_id(Option<String>)`](crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder::set_transit_gateway_multicast_domain_id): <p>The ID of the transit gateway multicast domain.</p>
    ///   - [`group_ip_address(impl Into<String>)`](crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder::group_ip_address) / [`set_group_ip_address(Option<String>)`](crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder::set_group_ip_address): <p>The IP address assigned to the transit gateway multicast group.</p>
    ///   - [`network_interface_ids(Vec<String>)`](crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder::network_interface_ids) / [`set_network_interface_ids(Option<Vec<String>>)`](crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder::set_network_interface_ids): <p>The IDs of the group members' network interfaces.</p>
    ///   - [`dry_run(bool)`](crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DeregisterTransitGatewayMulticastGroupMembersOutput`](crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersOutput) with field(s):
    ///   - [`deregistered_multicast_group_members(Option<TransitGatewayMulticastDeregisteredGroupMembers>)`](crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersOutput::deregistered_multicast_group_members): <p>Information about the deregistered members.</p>
    /// - On failure, responds with [`SdkError<DeregisterTransitGatewayMulticastGroupMembersError>`](crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersError)
    pub fn deregister_transit_gateway_multicast_group_members(&self) -> crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder{
        crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersFluentBuilder::new(self.handle.clone())
    }
}
