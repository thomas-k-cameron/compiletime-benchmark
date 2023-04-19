// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateTransitGatewayRouteTable`](crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_route_table_id(impl Into<String>)`](crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableFluentBuilder::transit_gateway_route_table_id) / [`set_transit_gateway_route_table_id(Option<String>)`](crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableFluentBuilder::set_transit_gateway_route_table_id): <p>The ID of the transit gateway route table.</p>
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableFluentBuilder::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableFluentBuilder::set_transit_gateway_attachment_id): <p>The ID of the attachment.</p>
    ///   - [`dry_run(bool)`](crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`AssociateTransitGatewayRouteTableOutput`](crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableOutput) with field(s):
    ///   - [`association(Option<TransitGatewayAssociation>)`](crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableOutput::association): <p>The ID of the association.</p>
    /// - On failure, responds with [`SdkError<AssociateTransitGatewayRouteTableError>`](crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableError)
    pub fn associate_transit_gateway_route_table(&self) -> crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableFluentBuilder{
        crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableFluentBuilder::new(self.handle.clone())
    }
}
