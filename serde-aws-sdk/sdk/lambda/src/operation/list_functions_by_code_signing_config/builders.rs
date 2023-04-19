// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_functions_by_code_signing_config::_list_functions_by_code_signing_config_output::ListFunctionsByCodeSigningConfigOutputBuilder;

pub use crate::operation::list_functions_by_code_signing_config::_list_functions_by_code_signing_config_input::ListFunctionsByCodeSigningConfigInputBuilder;

/// Fluent builder constructing a request to `ListFunctionsByCodeSigningConfig`.
///
/// <p>List the functions that use the specified code signing configuration. You can use this method prior to deleting a code signing configuration, to verify that no functions are using it.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListFunctionsByCodeSigningConfigFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigInputBuilder
            }
impl ListFunctionsByCodeSigningConfigFluentBuilder {
    /// Creates a new `ListFunctionsByCodeSigningConfig`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfig, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigOutput, aws_smithy_http::result::SdkError<crate::operation::list_functions_by_code_signing_config::ListFunctionsByCodeSigningConfigError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.list_functions_by_code_signing_config().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::list_functions_by_code_signing_config::builders::ListFunctionsByCodeSigningConfigInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_functions_by_code_signing_config::paginator::ListFunctionsByCodeSigningConfigPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_functions_by_code_signing_config::paginator::ListFunctionsByCodeSigningConfigPaginator{
        crate::operation::list_functions_by_code_signing_config::paginator::ListFunctionsByCodeSigningConfigPaginator::new(self.handle, self.inner)
    }
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    pub fn code_signing_config_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.code_signing_config_arn(input.into());
        self
    }
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    pub fn set_code_signing_config_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_code_signing_config_arn(input);
        self
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>Maximum number of items to return.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>Maximum number of items to return.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
}
