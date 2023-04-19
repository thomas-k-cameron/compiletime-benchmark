// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableTransitGatewayRouteTablePropagation`](crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_route_table_id(impl Into<String>)`](crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder::transit_gateway_route_table_id) / [`set_transit_gateway_route_table_id(Option<String>)`](crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder::set_transit_gateway_route_table_id): <p>The ID of the propagation route table.</p>
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder::set_transit_gateway_attachment_id): <p>The ID of the attachment.</p>
    ///   - [`dry_run(bool)`](crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`transit_gateway_route_table_announcement_id(impl Into<String>)`](crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder::transit_gateway_route_table_announcement_id) / [`set_transit_gateway_route_table_announcement_id(Option<String>)`](crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder::set_transit_gateway_route_table_announcement_id): <p>The ID of the transit gateway route table announcement.</p>
    /// - On success, responds with [`EnableTransitGatewayRouteTablePropagationOutput`](crate::operation::enable_transit_gateway_route_table_propagation::EnableTransitGatewayRouteTablePropagationOutput) with field(s):
    ///   - [`propagation(Option<TransitGatewayPropagation>)`](crate::operation::enable_transit_gateway_route_table_propagation::EnableTransitGatewayRouteTablePropagationOutput::propagation): <p>Information about route propagation.</p>
    /// - On failure, responds with [`SdkError<EnableTransitGatewayRouteTablePropagationError>`](crate::operation::enable_transit_gateway_route_table_propagation::EnableTransitGatewayRouteTablePropagationError)
    pub fn enable_transit_gateway_route_table_propagation(&self) -> crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder{
        crate::operation::enable_transit_gateway_route_table_propagation::builders::EnableTransitGatewayRouteTablePropagationFluentBuilder::new(self.handle.clone())
    }
}
