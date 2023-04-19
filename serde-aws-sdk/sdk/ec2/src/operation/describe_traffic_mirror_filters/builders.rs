// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_traffic_mirror_filters::_describe_traffic_mirror_filters_output::DescribeTrafficMirrorFiltersOutputBuilder;

pub use crate::operation::describe_traffic_mirror_filters::_describe_traffic_mirror_filters_input::DescribeTrafficMirrorFiltersInputBuilder;

/// Fluent builder constructing a request to `DescribeTrafficMirrorFilters`.
///
/// <p>Describes one or more Traffic Mirror filters.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeTrafficMirrorFiltersFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersInputBuilder
            }
impl DescribeTrafficMirrorFiltersFluentBuilder {
    /// Creates a new `DescribeTrafficMirrorFilters`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_traffic_mirror_filters::DescribeTrafficMirrorFilters,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_traffic_mirror_filters::DescribeTrafficMirrorFiltersError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::describe_traffic_mirror_filters::DescribeTrafficMirrorFiltersOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_traffic_mirror_filters::DescribeTrafficMirrorFiltersError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_traffic_mirror_filters().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_traffic_mirror_filters::builders::DescribeTrafficMirrorFiltersInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_traffic_mirror_filters::paginator::DescribeTrafficMirrorFiltersPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_traffic_mirror_filters::paginator::DescribeTrafficMirrorFiltersPaginator{
        crate::operation::describe_traffic_mirror_filters::paginator::DescribeTrafficMirrorFiltersPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `TrafficMirrorFilterIds`.
    ///
    /// To override the contents of this collection use [`set_traffic_mirror_filter_ids`](Self::set_traffic_mirror_filter_ids).
    ///
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn traffic_mirror_filter_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.traffic_mirror_filter_ids(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn set_traffic_mirror_filter_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_traffic_mirror_filter_ids(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>description</code>: The Traffic Mirror filter description.</p> </li>
    /// <li> <p> <code>traffic-mirror-filter-id</code>: The ID of the Traffic Mirror filter.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>description</code>: The Traffic Mirror filter description.</p> </li>
    /// <li> <p> <code>traffic-mirror-filter-id</code>: The ID of the Traffic Mirror filter.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
