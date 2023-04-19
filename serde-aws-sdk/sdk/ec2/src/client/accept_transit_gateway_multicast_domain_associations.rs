// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AcceptTransitGatewayMulticastDomainAssociations`](crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_multicast_domain_id(impl Into<String>)`](crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder::transit_gateway_multicast_domain_id) / [`set_transit_gateway_multicast_domain_id(Option<String>)`](crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder::set_transit_gateway_multicast_domain_id): <p>The ID of the transit gateway multicast domain.</p>
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder::set_transit_gateway_attachment_id): <p>The ID of the transit gateway attachment.</p>
    ///   - [`subnet_ids(Vec<String>)`](crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder::subnet_ids) / [`set_subnet_ids(Option<Vec<String>>)`](crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder::set_subnet_ids): <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    ///   - [`dry_run(bool)`](crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`AcceptTransitGatewayMulticastDomainAssociationsOutput`](crate::operation::accept_transit_gateway_multicast_domain_associations::AcceptTransitGatewayMulticastDomainAssociationsOutput) with field(s):
    ///   - [`associations(Option<TransitGatewayMulticastDomainAssociations>)`](crate::operation::accept_transit_gateway_multicast_domain_associations::AcceptTransitGatewayMulticastDomainAssociationsOutput::associations): <p>Information about the multicast domain associations.</p>
    /// - On failure, responds with [`SdkError<AcceptTransitGatewayMulticastDomainAssociationsError>`](crate::operation::accept_transit_gateway_multicast_domain_associations::AcceptTransitGatewayMulticastDomainAssociationsError)
    pub fn accept_transit_gateway_multicast_domain_associations(&self) -> crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder{
        crate::operation::accept_transit_gateway_multicast_domain_associations::builders::AcceptTransitGatewayMulticastDomainAssociationsFluentBuilder::new(self.handle.clone())
    }
}
