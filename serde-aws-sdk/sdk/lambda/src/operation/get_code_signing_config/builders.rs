// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_code_signing_config::_get_code_signing_config_output::GetCodeSigningConfigOutputBuilder;

pub use crate::operation::get_code_signing_config::_get_code_signing_config_input::GetCodeSigningConfigInputBuilder;

/// Fluent builder constructing a request to `GetCodeSigningConfig`.
///
/// <p>Returns information about the specified code signing configuration.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetCodeSigningConfigFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_code_signing_config::builders::GetCodeSigningConfigInputBuilder,
}
impl GetCodeSigningConfigFluentBuilder {
    /// Creates a new `GetCodeSigningConfig`.
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
            crate::operation::get_code_signing_config::GetCodeSigningConfig,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_code_signing_config::GetCodeSigningConfigError,
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
        crate::operation::get_code_signing_config::GetCodeSigningConfigOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_code_signing_config::GetCodeSigningConfigError,
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
    ///     let deserialized_parameters: crate::operation::get_code_signing_config::builders::GetCodeSigningConfigInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_code_signing_config().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_code_signing_config::builders::GetCodeSigningConfigInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration. </p>
    pub fn code_signing_config_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.code_signing_config_arn(input.into());
        self
    }
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration. </p>
    pub fn set_code_signing_config_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_code_signing_config_arn(input);
        self
    }
}
