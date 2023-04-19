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
pub struct GetAwsNetworkPerformanceDataInput {
    /// <p>A list of network performance data queries.</p>
    #[doc(hidden)]
    pub data_queries: std::option::Option<std::vec::Vec<crate::types::DataQuery>>,
    /// <p>The starting time for the performance data request. The starting time must be formatted as <code>yyyy-mm-ddThh:mm:ss</code>. For example, <code>2022-06-10T12:00:00.000Z</code>.</p>
    #[doc(hidden)]
    pub start_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The ending time for the performance data request. The end time must be formatted as <code>yyyy-mm-ddThh:mm:ss</code>. For example, <code>2022-06-12T12:00:00.000Z</code>.</p>
    #[doc(hidden)]
    pub end_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>The token for the next page of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl GetAwsNetworkPerformanceDataInput {
    /// <p>A list of network performance data queries.</p>
    pub fn data_queries(&self) -> std::option::Option<&[crate::types::DataQuery]> {
        self.data_queries.as_deref()
    }
    /// <p>The starting time for the performance data request. The starting time must be formatted as <code>yyyy-mm-ddThh:mm:ss</code>. For example, <code>2022-06-10T12:00:00.000Z</code>.</p>
    pub fn start_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The ending time for the performance data request. The end time must be formatted as <code>yyyy-mm-ddThh:mm:ss</code>. For example, <code>2022-06-12T12:00:00.000Z</code>.</p>
    pub fn end_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl GetAwsNetworkPerformanceDataInput {
    /// Creates a new builder-style object to manufacture [`GetAwsNetworkPerformanceDataInput`](crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataInput).
    pub fn builder() -> crate::operation::get_aws_network_performance_data::builders::GetAwsNetworkPerformanceDataInputBuilder{
        crate::operation::get_aws_network_performance_data::builders::GetAwsNetworkPerformanceDataInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataInput;
/// A builder for [`GetAwsNetworkPerformanceDataInput`](crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataInput).
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
pub struct GetAwsNetworkPerformanceDataInputBuilder {
    pub(crate) data_queries: std::option::Option<std::vec::Vec<crate::types::DataQuery>>,
    pub(crate) start_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) end_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl GetAwsNetworkPerformanceDataInputBuilder {
    /// Appends an item to `data_queries`.
    ///
    /// To override the contents of this collection use [`set_data_queries`](Self::set_data_queries).
    ///
    /// <p>A list of network performance data queries.</p>
    pub fn data_queries(mut self, input: crate::types::DataQuery) -> Self {
        let mut v = self.data_queries.unwrap_or_default();
        v.push(input);
        self.data_queries = Some(v);
        self
    }
    /// <p>A list of network performance data queries.</p>
    pub fn set_data_queries(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::DataQuery>>,
    ) -> Self {
        self.data_queries = input;
        self
    }
    /// <p>The starting time for the performance data request. The starting time must be formatted as <code>yyyy-mm-ddThh:mm:ss</code>. For example, <code>2022-06-10T12:00:00.000Z</code>.</p>
    pub fn start_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.start_time = Some(input);
        self
    }
    /// <p>The starting time for the performance data request. The starting time must be formatted as <code>yyyy-mm-ddThh:mm:ss</code>. For example, <code>2022-06-10T12:00:00.000Z</code>.</p>
    pub fn set_start_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The ending time for the performance data request. The end time must be formatted as <code>yyyy-mm-ddThh:mm:ss</code>. For example, <code>2022-06-12T12:00:00.000Z</code>.</p>
    pub fn end_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.end_time = Some(input);
        self
    }
    /// <p>The ending time for the performance data request. The end time must be formatted as <code>yyyy-mm-ddThh:mm:ss</code>. For example, <code>2022-06-12T12:00:00.000Z</code>.</p>
    pub fn set_end_time(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.end_time = input;
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
    /// Consumes the builder and constructs a [`GetAwsNetworkPerformanceDataInput`](crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataInput {
                data_queries: self.data_queries,
                start_time: self.start_time,
                end_time: self.end_time,
                max_results: self.max_results,
                next_token: self.next_token,
                dry_run: self.dry_run,
            },
        )
    }
}