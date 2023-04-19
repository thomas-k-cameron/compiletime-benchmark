// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeIpv6Pools`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`pool_ids(Vec<String>)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::pool_ids) / [`set_pool_ids(Option<Vec<String>>)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::set_pool_ids): <p>The IDs of the IPv6 address pools.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>  </ul>
    /// - On success, responds with [`DescribeIpv6PoolsOutput`](crate::operation::describe_ipv6_pools::DescribeIpv6PoolsOutput) with field(s):
    ///   - [`ipv6_pools(Option<Vec<Ipv6Pool>>)`](crate::operation::describe_ipv6_pools::DescribeIpv6PoolsOutput::ipv6_pools): <p>Information about the IPv6 address pools.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_ipv6_pools::DescribeIpv6PoolsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeIpv6PoolsError>`](crate::operation::describe_ipv6_pools::DescribeIpv6PoolsError)
    pub fn describe_ipv6_pools(
        &self,
    ) -> crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder {
        crate::operation::describe_ipv6_pools::builders::DescribeIpv6PoolsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}