// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_conformance_pack_status::_describe_conformance_pack_status_output::DescribeConformancePackStatusOutputBuilder;

pub use crate::operation::describe_conformance_pack_status::_describe_conformance_pack_status_input::DescribeConformancePackStatusInputBuilder;

/// Fluent builder constructing a request to `DescribeConformancePackStatus`.
///
/// <p>Provides one or more conformance packs deployment status.</p> <note>
/// <p>If there are no conformance packs then you will see an empty result.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConformancePackStatusFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_conformance_pack_status::builders::DescribeConformancePackStatusInputBuilder
            }
impl DescribeConformancePackStatusFluentBuilder {
    /// Creates a new `DescribeConformancePackStatus`.
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
            crate::operation::describe_conformance_pack_status::DescribeConformancePackStatus,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusError,
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
        crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_conformance_pack_status::DescribeConformancePackStatusError,
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
    ///     let deserialized_parameters: crate::operation::describe_conformance_pack_status::builders::DescribeConformancePackStatusInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_conformance_pack_status().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_conformance_pack_status::builders::DescribeConformancePackStatusInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_conformance_pack_status::paginator::DescribeConformancePackStatusPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_conformance_pack_status::paginator::DescribeConformancePackStatusPaginator{
        crate::operation::describe_conformance_pack_status::paginator::DescribeConformancePackStatusPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `ConformancePackNames`.
    ///
    /// To override the contents of this collection use [`set_conformance_pack_names`](Self::set_conformance_pack_names).
    ///
    /// <p>Comma-separated list of conformance pack names.</p>
    pub fn conformance_pack_names(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.conformance_pack_names(input.into());
        self
    }
    /// <p>Comma-separated list of conformance pack names.</p>
    pub fn set_conformance_pack_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_conformance_pack_names(input);
        self
    }
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}