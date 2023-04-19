// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteEgressOnlyInternetGateway`](crate::operation::delete_egress_only_internet_gateway::builders::DeleteEgressOnlyInternetGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_egress_only_internet_gateway::builders::DeleteEgressOnlyInternetGatewayFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_egress_only_internet_gateway::builders::DeleteEgressOnlyInternetGatewayFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`egress_only_internet_gateway_id(impl Into<String>)`](crate::operation::delete_egress_only_internet_gateway::builders::DeleteEgressOnlyInternetGatewayFluentBuilder::egress_only_internet_gateway_id) / [`set_egress_only_internet_gateway_id(Option<String>)`](crate::operation::delete_egress_only_internet_gateway::builders::DeleteEgressOnlyInternetGatewayFluentBuilder::set_egress_only_internet_gateway_id): <p>The ID of the egress-only internet gateway.</p>
    /// - On success, responds with [`DeleteEgressOnlyInternetGatewayOutput`](crate::operation::delete_egress_only_internet_gateway::DeleteEgressOnlyInternetGatewayOutput) with field(s):
    ///   - [`return_code(Option<bool>)`](crate::operation::delete_egress_only_internet_gateway::DeleteEgressOnlyInternetGatewayOutput::return_code): <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    /// - On failure, responds with [`SdkError<DeleteEgressOnlyInternetGatewayError>`](crate::operation::delete_egress_only_internet_gateway::DeleteEgressOnlyInternetGatewayError)
    pub fn delete_egress_only_internet_gateway(&self) -> crate::operation::delete_egress_only_internet_gateway::builders::DeleteEgressOnlyInternetGatewayFluentBuilder{
        crate::operation::delete_egress_only_internet_gateway::builders::DeleteEgressOnlyInternetGatewayFluentBuilder::new(self.handle.clone())
    }
}
