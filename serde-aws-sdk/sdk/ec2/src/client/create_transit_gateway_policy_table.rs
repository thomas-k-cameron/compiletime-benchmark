// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTransitGatewayPolicyTable`](crate::operation::create_transit_gateway_policy_table::builders::CreateTransitGatewayPolicyTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_id(impl Into<String>)`](crate::operation::create_transit_gateway_policy_table::builders::CreateTransitGatewayPolicyTableFluentBuilder::transit_gateway_id) / [`set_transit_gateway_id(Option<String>)`](crate::operation::create_transit_gateway_policy_table::builders::CreateTransitGatewayPolicyTableFluentBuilder::set_transit_gateway_id): <p>The ID of the transit gateway used for the policy table.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::create_transit_gateway_policy_table::builders::CreateTransitGatewayPolicyTableFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::create_transit_gateway_policy_table::builders::CreateTransitGatewayPolicyTableFluentBuilder::set_tag_specifications): <p>The tags specification for the transit gateway policy table created during the request.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_transit_gateway_policy_table::builders::CreateTransitGatewayPolicyTableFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_transit_gateway_policy_table::builders::CreateTransitGatewayPolicyTableFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`CreateTransitGatewayPolicyTableOutput`](crate::operation::create_transit_gateway_policy_table::CreateTransitGatewayPolicyTableOutput) with field(s):
    ///   - [`transit_gateway_policy_table(Option<TransitGatewayPolicyTable>)`](crate::operation::create_transit_gateway_policy_table::CreateTransitGatewayPolicyTableOutput::transit_gateway_policy_table): <p>Describes the created transit gateway policy table.</p>
    /// - On failure, responds with [`SdkError<CreateTransitGatewayPolicyTableError>`](crate::operation::create_transit_gateway_policy_table::CreateTransitGatewayPolicyTableError)
    pub fn create_transit_gateway_policy_table(&self) -> crate::operation::create_transit_gateway_policy_table::builders::CreateTransitGatewayPolicyTableFluentBuilder{
        crate::operation::create_transit_gateway_policy_table::builders::CreateTransitGatewayPolicyTableFluentBuilder::new(self.handle.clone())
    }
}
