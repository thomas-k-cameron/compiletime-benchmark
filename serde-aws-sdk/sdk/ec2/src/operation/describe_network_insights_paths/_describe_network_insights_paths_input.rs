// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeNetworkInsightsPathsInput {
    /// <p>The IDs of the paths.</p>
    #[doc(hidden)]
    pub network_insights_path_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The filters. The following are the possible values:</p>
    /// <ul>
    /// <li> <p>destination - The ID of the resource.</p> </li>
    /// <li> <p>destination-port - The destination port.</p> </li>
    /// <li> <p>protocol - The protocol.</p> </li>
    /// <li> <p>source - The ID of the resource.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The token for the next page of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeNetworkInsightsPathsInput {
    /// <p>The IDs of the paths.</p>
    pub fn network_insights_path_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.network_insights_path_ids.as_deref()
    }
    /// <p>The filters. The following are the possible values:</p>
    /// <ul>
    /// <li> <p>destination - The ID of the resource.</p> </li>
    /// <li> <p>destination-port - The destination port.</p> </li>
    /// <li> <p>protocol - The protocol.</p> </li>
    /// <li> <p>source - The ID of the resource.</p> </li>
    /// </ul>
    pub fn filters(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeNetworkInsightsPathsInput {
    /// Creates a new builder-style object to manufacture [`DescribeNetworkInsightsPathsInput`](crate::operation::describe_network_insights_paths::DescribeNetworkInsightsPathsInput).
    pub fn builder() -> crate::operation::describe_network_insights_paths::builders::DescribeNetworkInsightsPathsInputBuilder{
        crate::operation::describe_network_insights_paths::builders::DescribeNetworkInsightsPathsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_network_insights_paths::DescribeNetworkInsightsPathsInput;
/// A builder for [`DescribeNetworkInsightsPathsInput`](crate::operation::describe_network_insights_paths::DescribeNetworkInsightsPathsInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct DescribeNetworkInsightsPathsInputBuilder {
    pub(crate) network_insights_path_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl DescribeNetworkInsightsPathsInputBuilder {
    /// Appends an item to `network_insights_path_ids`.
    ///
    /// To override the contents of this collection use [`set_network_insights_path_ids`](Self::set_network_insights_path_ids).
    ///
    /// <p>The IDs of the paths.</p>
    pub fn network_insights_path_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.network_insights_path_ids.unwrap_or_default();
        v.push(input.into());
        self.network_insights_path_ids = Some(v);
        self
    }
    /// <p>The IDs of the paths.</p>
    pub fn set_network_insights_path_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.network_insights_path_ids = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters. The following are the possible values:</p>
    /// <ul>
    /// <li> <p>destination - The ID of the resource.</p> </li>
    /// <li> <p>destination-port - The destination port.</p> </li>
    /// <li> <p>protocol - The protocol.</p> </li>
    /// <li> <p>source - The ID of the resource.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = Some(v);
        self
    }
    /// <p>The filters. The following are the possible values:</p>
    /// <ul>
    /// <li> <p>destination - The ID of the resource.</p> </li>
    /// <li> <p>destination-port - The destination port.</p> </li>
    /// <li> <p>protocol - The protocol.</p> </li>
    /// <li> <p>source - The ID of the resource.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeNetworkInsightsPathsInput`](crate::operation::describe_network_insights_paths::DescribeNetworkInsightsPathsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_network_insights_paths::DescribeNetworkInsightsPathsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_network_insights_paths::DescribeNetworkInsightsPathsInput {
                network_insights_path_ids: self.network_insights_path_ids,
                filters: self.filters,
                max_results: self.max_results,
                dry_run: self.dry_run,
                next_token: self.next_token,
            },
        )
    }
}
