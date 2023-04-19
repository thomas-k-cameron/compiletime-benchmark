// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCapacityProviders`](crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`capacity_providers(Vec<String>)`](crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder::capacity_providers) / [`set_capacity_providers(Option<Vec<String>>)`](crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder::set_capacity_providers): <p>The short name or full Amazon Resource Name (ARN) of one or more capacity providers. Up to <code>100</code> capacity providers can be described in an action.</p>
    ///   - [`include(Vec<CapacityProviderField>)`](crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder::include) / [`set_include(Option<Vec<CapacityProviderField>>)`](crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder::set_include): <p>Specifies whether or not you want to see the resource tags for the capacity provider. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder::set_max_results): <p>The maximum number of account setting results returned by <code>DescribeCapacityProviders</code> in paginated output. When this parameter is used, <code>DescribeCapacityProviders</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeCapacityProviders</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 10. If this parameter is not used, then <code>DescribeCapacityProviders</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder::set_next_token): <p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeCapacityProviders</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note>   <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>  </note>
    /// - On success, responds with [`DescribeCapacityProvidersOutput`](crate::operation::describe_capacity_providers::DescribeCapacityProvidersOutput) with field(s):
    ///   - [`capacity_providers(Option<Vec<CapacityProvider>>)`](crate::operation::describe_capacity_providers::DescribeCapacityProvidersOutput::capacity_providers): <p>The list of capacity providers.</p>
    ///   - [`failures(Option<Vec<Failure>>)`](crate::operation::describe_capacity_providers::DescribeCapacityProvidersOutput::failures): <p>Any failures associated with the call.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_capacity_providers::DescribeCapacityProvidersOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>DescribeCapacityProviders</code> request. When the results of a <code>DescribeCapacityProviders</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeCapacityProvidersError>`](crate::operation::describe_capacity_providers::DescribeCapacityProvidersError)
    pub fn describe_capacity_providers(&self) -> crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder{
        crate::operation::describe_capacity_providers::builders::DescribeCapacityProvidersFluentBuilder::new(self.handle.clone())
    }
}
