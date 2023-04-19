// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTransitGateway`](crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder::set_description): <p>A description of the transit gateway.</p>
    ///   - [`options(TransitGatewayRequestOptions)`](crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder::options) / [`set_options(Option<TransitGatewayRequestOptions>)`](crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder::set_options): <p>The transit gateway options.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder::set_tag_specifications): <p>The tags to apply to the transit gateway.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`CreateTransitGatewayOutput`](crate::operation::create_transit_gateway::CreateTransitGatewayOutput) with field(s):
    ///   - [`transit_gateway(Option<TransitGateway>)`](crate::operation::create_transit_gateway::CreateTransitGatewayOutput::transit_gateway): <p>Information about the transit gateway.</p>
    /// - On failure, responds with [`SdkError<CreateTransitGatewayError>`](crate::operation::create_transit_gateway::CreateTransitGatewayError)
    pub fn create_transit_gateway(
        &self,
    ) -> crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder {
        crate::operation::create_transit_gateway::builders::CreateTransitGatewayFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
