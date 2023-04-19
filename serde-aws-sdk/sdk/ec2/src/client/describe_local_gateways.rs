// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeLocalGateways`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`local_gateway_ids(Vec<String>)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::local_gateway_ids) / [`set_local_gateway_ids(Option<Vec<String>>)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::set_local_gateway_ids): <p>The IDs of the local gateways.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>local-gateway-id</code> - The ID of a local gateway.</p> </li>   <li> <p> <code>outpost-arn</code> - The Amazon Resource Name (ARN) of the Outpost.</p> </li>   <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the local gateway.</p> </li>   <li> <p> <code>state</code> - The state of the association.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DescribeLocalGatewaysOutput`](crate::operation::describe_local_gateways::DescribeLocalGatewaysOutput) with field(s):
    ///   - [`local_gateways(Option<Vec<LocalGateway>>)`](crate::operation::describe_local_gateways::DescribeLocalGatewaysOutput::local_gateways): <p>Information about the local gateways.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_local_gateways::DescribeLocalGatewaysOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeLocalGatewaysError>`](crate::operation::describe_local_gateways::DescribeLocalGatewaysError)
    pub fn describe_local_gateways(
        &self,
    ) -> crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder
    {
        crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysFluentBuilder::new(
            self.handle.clone(),
        )
    }
}