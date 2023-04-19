// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAggregationAuthorizations`](crate::operation::describe_aggregation_authorizations::builders::DescribeAggregationAuthorizationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_aggregation_authorizations::builders::DescribeAggregationAuthorizationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`limit(i32)`](crate::operation::describe_aggregation_authorizations::builders::DescribeAggregationAuthorizationsFluentBuilder::limit) / [`set_limit(i32)`](crate::operation::describe_aggregation_authorizations::builders::DescribeAggregationAuthorizationsFluentBuilder::set_limit): <p>The maximum number of AggregationAuthorizations returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_aggregation_authorizations::builders::DescribeAggregationAuthorizationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_aggregation_authorizations::builders::DescribeAggregationAuthorizationsFluentBuilder::set_next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    /// - On success, responds with [`DescribeAggregationAuthorizationsOutput`](crate::operation::describe_aggregation_authorizations::DescribeAggregationAuthorizationsOutput) with field(s):
    ///   - [`aggregation_authorizations(Option<Vec<AggregationAuthorization>>)`](crate::operation::describe_aggregation_authorizations::DescribeAggregationAuthorizationsOutput::aggregation_authorizations): <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_aggregation_authorizations::DescribeAggregationAuthorizationsOutput::next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    /// - On failure, responds with [`SdkError<DescribeAggregationAuthorizationsError>`](crate::operation::describe_aggregation_authorizations::DescribeAggregationAuthorizationsError)
    pub fn describe_aggregation_authorizations(&self) -> crate::operation::describe_aggregation_authorizations::builders::DescribeAggregationAuthorizationsFluentBuilder{
        crate::operation::describe_aggregation_authorizations::builders::DescribeAggregationAuthorizationsFluentBuilder::new(self.handle.clone())
    }
}
