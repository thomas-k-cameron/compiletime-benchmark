// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_layer_versions::_list_layer_versions_output::ListLayerVersionsOutputBuilder;

pub use crate::operation::list_layer_versions::_list_layer_versions_input::ListLayerVersionsInputBuilder;

/// Fluent builder constructing a request to `ListLayerVersions`.
///
/// <p>Lists the versions of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">Lambda layer</a>. Versions that have been deleted aren't listed. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only versions that indicate that they're compatible with that runtime. Specify a compatible architecture to include only layer versions that are compatible with that architecture.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListLayerVersionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_layer_versions::builders::ListLayerVersionsInputBuilder,
}
impl ListLayerVersionsFluentBuilder {
    /// Creates a new `ListLayerVersions`.
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
            crate::operation::list_layer_versions::ListLayerVersions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_layer_versions::ListLayerVersionsError,
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
        crate::operation::list_layer_versions::ListLayerVersionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_layer_versions::ListLayerVersionsError,
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
    ///     let deserialized_parameters: crate::operation::list_layer_versions::builders::ListLayerVersionsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.list_layer_versions().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::list_layer_versions::builders::ListLayerVersionsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_layer_versions::paginator::ListLayerVersionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_layer_versions::paginator::ListLayerVersionsPaginator {
        crate::operation::list_layer_versions::paginator::ListLayerVersionsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>A runtime identifier. For example, <code>go1.x</code>.</p>
    pub fn compatible_runtime(mut self, input: crate::types::Runtime) -> Self {
        self.inner = self.inner.compatible_runtime(input);
        self
    }
    /// <p>A runtime identifier. For example, <code>go1.x</code>.</p>
    pub fn set_compatible_runtime(
        mut self,
        input: std::option::Option<crate::types::Runtime>,
    ) -> Self {
        self.inner = self.inner.set_compatible_runtime(input);
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn layer_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.layer_name(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn set_layer_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_layer_name(input);
        self
    }
    /// <p>A pagination token returned by a previous call.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>A pagination token returned by a previous call.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The maximum number of versions to return.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of versions to return.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>The compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architecture</a>.</p>
    pub fn compatible_architecture(mut self, input: crate::types::Architecture) -> Self {
        self.inner = self.inner.compatible_architecture(input);
        self
    }
    /// <p>The compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architecture</a>.</p>
    pub fn set_compatible_architecture(
        mut self,
        input: std::option::Option<crate::types::Architecture>,
    ) -> Self {
        self.inner = self.inner.set_compatible_architecture(input);
        self
    }
}
