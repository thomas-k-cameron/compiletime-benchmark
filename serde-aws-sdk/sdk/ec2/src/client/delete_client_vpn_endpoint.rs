// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteClientVpnEndpoint`](crate::operation::delete_client_vpn_endpoint::builders::DeleteClientVpnEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_vpn_endpoint_id(impl Into<String>)`](crate::operation::delete_client_vpn_endpoint::builders::DeleteClientVpnEndpointFluentBuilder::client_vpn_endpoint_id) / [`set_client_vpn_endpoint_id(Option<String>)`](crate::operation::delete_client_vpn_endpoint::builders::DeleteClientVpnEndpointFluentBuilder::set_client_vpn_endpoint_id): <p>The ID of the Client VPN to be deleted.</p>
    ///   - [`dry_run(bool)`](crate::operation::delete_client_vpn_endpoint::builders::DeleteClientVpnEndpointFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_client_vpn_endpoint::builders::DeleteClientVpnEndpointFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DeleteClientVpnEndpointOutput`](crate::operation::delete_client_vpn_endpoint::DeleteClientVpnEndpointOutput) with field(s):
    ///   - [`status(Option<ClientVpnEndpointStatus>)`](crate::operation::delete_client_vpn_endpoint::DeleteClientVpnEndpointOutput::status): <p>The current state of the Client VPN endpoint.</p>
    /// - On failure, responds with [`SdkError<DeleteClientVpnEndpointError>`](crate::operation::delete_client_vpn_endpoint::DeleteClientVpnEndpointError)
    pub fn delete_client_vpn_endpoint(
        &self,
    ) -> crate::operation::delete_client_vpn_endpoint::builders::DeleteClientVpnEndpointFluentBuilder
    {
        crate::operation::delete_client_vpn_endpoint::builders::DeleteClientVpnEndpointFluentBuilder::new(self.handle.clone())
    }
}
