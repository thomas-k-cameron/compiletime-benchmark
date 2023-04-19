// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCoipPools`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`pool_ids(Vec<String>)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::pool_ids) / [`set_pool_ids(Option<Vec<String>>)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::set_pool_ids): <p>The IDs of the address pools.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>coip-pool.local-gateway-route-table-id</code> - The ID of the local gateway route table.</p> </li>   <li> <p> <code>coip-pool.pool-id</code> - The ID of the address pool.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DescribeCoipPoolsOutput`](crate::operation::describe_coip_pools::DescribeCoipPoolsOutput) with field(s):
    ///   - [`coip_pools(Option<Vec<CoipPool>>)`](crate::operation::describe_coip_pools::DescribeCoipPoolsOutput::coip_pools): <p>Information about the address pools.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_coip_pools::DescribeCoipPoolsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeCoipPoolsError>`](crate::operation::describe_coip_pools::DescribeCoipPoolsError)
    pub fn describe_coip_pools(
        &self,
    ) -> crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder {
        crate::operation::describe_coip_pools::builders::DescribeCoipPoolsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
