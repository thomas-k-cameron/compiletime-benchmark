// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_spot_fleet_requests::_describe_spot_fleet_requests_output::DescribeSpotFleetRequestsOutputBuilder;

pub use crate::operation::describe_spot_fleet_requests::_describe_spot_fleet_requests_input::DescribeSpotFleetRequestsInputBuilder;

/// Fluent builder constructing a request to `DescribeSpotFleetRequests`.
///
/// <p>Describes your Spot Fleet requests.</p>
/// <p>Spot Fleet requests are deleted 48 hours after they are canceled and their instances are terminated.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSpotFleetRequestsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsInputBuilder
            }
impl DescribeSpotFleetRequestsFluentBuilder {
    /// Creates a new `DescribeSpotFleetRequests`.
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
            crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequests,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsError,
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
        crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsError,
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
    ///     let deserialized_parameters: crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_spot_fleet_requests().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_spot_fleet_requests::paginator::DescribeSpotFleetRequestsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_spot_fleet_requests::paginator::DescribeSpotFleetRequestsPaginator
    {
        crate::operation::describe_spot_fleet_requests::paginator::DescribeSpotFleetRequestsPaginator::new(self.handle, self.inner)
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
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// Appends an item to `SpotFleetRequestIds`.
    ///
    /// To override the contents of this collection use [`set_spot_fleet_request_ids`](Self::set_spot_fleet_request_ids).
    ///
    /// <p>The IDs of the Spot Fleet requests.</p>
    pub fn spot_fleet_request_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.spot_fleet_request_ids(input.into());
        self
    }
    /// <p>The IDs of the Spot Fleet requests.</p>
    pub fn set_spot_fleet_request_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_spot_fleet_request_ids(input);
        self
    }
}
