// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_fast_launch_images::_describe_fast_launch_images_output::DescribeFastLaunchImagesOutputBuilder;

pub use crate::operation::describe_fast_launch_images::_describe_fast_launch_images_input::DescribeFastLaunchImagesInputBuilder;

/// Fluent builder constructing a request to `DescribeFastLaunchImages`.
///
/// <p>Describe details for Windows AMIs that are configured for faster launching.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFastLaunchImagesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesInputBuilder
            }
impl DescribeFastLaunchImagesFluentBuilder {
    /// Creates a new `DescribeFastLaunchImages`.
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
            crate::operation::describe_fast_launch_images::DescribeFastLaunchImages,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesError,
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
        crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesError,
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
    ///     let deserialized_parameters: crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_fast_launch_images().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_fast_launch_images::paginator::DescribeFastLaunchImagesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_fast_launch_images::paginator::DescribeFastLaunchImagesPaginator
    {
        crate::operation::describe_fast_launch_images::paginator::DescribeFastLaunchImagesPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `ImageIds`.
    ///
    /// To override the contents of this collection use [`set_image_ids`](Self::set_image_ids).
    ///
    /// <p>Details for one or more Windows AMI image IDs.</p>
    pub fn image_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.image_ids(input.into());
        self
    }
    /// <p>Details for one or more Windows AMI image IDs.</p>
    pub fn set_image_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_image_ids(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Use the following filters to streamline results.</p>
    /// <ul>
    /// <li> <p> <code>resource-type</code> - The resource type for pre-provisioning.</p> </li>
    /// <li> <p> <code>launch-template</code> - The launch template that is associated with the pre-provisioned Windows AMI.</p> </li>
    /// <li> <p> <code>owner-id</code> - The owner ID for the pre-provisioning resource.</p> </li>
    /// <li> <p> <code>state</code> - The current state of fast launching for the Windows AMI.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Use the following filters to streamline results.</p>
    /// <ul>
    /// <li> <p> <code>resource-type</code> - The resource type for pre-provisioning.</p> </li>
    /// <li> <p> <code>launch-template</code> - The launch template that is associated with the pre-provisioned Windows AMI.</p> </li>
    /// <li> <p> <code>owner-id</code> - The owner ID for the pre-provisioning resource.</p> </li>
    /// <li> <p> <code>state</code> - The current state of fast launching for the Windows AMI.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
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
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
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
}