// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVpcEndpoints`](crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`vpc_endpoint_ids(Vec<String>)`](crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsFluentBuilder::vpc_endpoint_ids) / [`set_vpc_endpoint_ids(Option<Vec<String>>)`](crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsFluentBuilder::set_vpc_endpoint_ids): <p>The IDs of the VPC endpoints.</p>
    /// - On success, responds with [`DeleteVpcEndpointsOutput`](crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsOutput) with field(s):
    ///   - [`unsuccessful(Option<Vec<UnsuccessfulItem>>)`](crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsOutput::unsuccessful): <p>Information about the VPC endpoints that were not successfully deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteVpcEndpointsError>`](crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsError)
    pub fn delete_vpc_endpoints(
        &self,
    ) -> crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsFluentBuilder {
        crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
