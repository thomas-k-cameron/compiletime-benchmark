// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_configuration_recorder::_stop_configuration_recorder_output::StopConfigurationRecorderOutputBuilder;

pub use crate::operation::stop_configuration_recorder::_stop_configuration_recorder_input::StopConfigurationRecorderInputBuilder;

/// Fluent builder constructing a request to `StopConfigurationRecorder`.
///
/// <p>Stops recording configurations of the Amazon Web Services resources you have selected to record in your Amazon Web Services account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct StopConfigurationRecorderFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::stop_configuration_recorder::builders::StopConfigurationRecorderInputBuilder
            }
impl StopConfigurationRecorderFluentBuilder {
    /// Creates a new `StopConfigurationRecorder`.
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
            crate::operation::stop_configuration_recorder::StopConfigurationRecorder,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::stop_configuration_recorder::StopConfigurationRecorderError,
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
        crate::operation::stop_configuration_recorder::StopConfigurationRecorderOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::stop_configuration_recorder::StopConfigurationRecorderError,
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
    ///     let deserialized_parameters: crate::operation::stop_configuration_recorder::builders::StopConfigurationRecorderInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.stop_configuration_recorder().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::stop_configuration_recorder::builders::StopConfigurationRecorderInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    pub fn configuration_recorder_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.configuration_recorder_name(input.into());
        self
    }
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    pub fn set_configuration_recorder_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_recorder_name(input);
        self
    }
}
