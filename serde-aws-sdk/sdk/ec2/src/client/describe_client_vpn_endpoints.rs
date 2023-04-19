// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeClientVpnEndpoints`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_vpn_endpoint_ids(Vec<String>)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::client_vpn_endpoint_ids) / [`set_client_vpn_endpoint_ids(Option<Vec<String>>)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::set_client_vpn_endpoint_ids): <p>The ID of the Client VPN endpoint.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::set_max_results): <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the nextToken value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::set_next_token): <p>The token to retrieve the next page of results.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::set_filters): <p>One or more filters. Filter names and values are case-sensitive.</p>  <ul>   <li> <p> <code>endpoint-id</code> - The ID of the Client VPN endpoint.</p> </li>   <li> <p> <code>transport-protocol</code> - The transport protocol (<code>tcp</code> | <code>udp</code>).</p> </li>  </ul>
    ///   - [`dry_run(bool)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DescribeClientVpnEndpointsOutput`](crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput) with field(s):
    ///   - [`client_vpn_endpoints(Option<Vec<ClientVpnEndpoint>>)`](crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput::client_vpn_endpoints): <p>Information about the Client VPN endpoints.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeClientVpnEndpointsError>`](crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsError)
    pub fn describe_client_vpn_endpoints(&self) -> crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder{
        crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsFluentBuilder::new(self.handle.clone())
    }
}