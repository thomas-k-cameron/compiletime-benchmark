// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_event_source_mapping::_delete_event_source_mapping_output::DeleteEventSourceMappingOutputBuilder;

pub use crate::operation::delete_event_source_mapping::_delete_event_source_mapping_input::DeleteEventSourceMappingInputBuilder;

/// Fluent builder constructing a request to `DeleteEventSourceMapping`.
///
/// <p>Deletes an <a href="https://docs.aws.amazon.com/lambda/latest/dg/intro-invocation-modes.html">event source mapping</a>. You can get the identifier of a mapping from the output of <code>ListEventSourceMappings</code>.</p>
/// <p>When you delete an event source mapping, it enters a <code>Deleting</code> state and might not be completely deleted for several seconds.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEventSourceMappingFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingInputBuilder
            }
impl DeleteEventSourceMappingFluentBuilder {
    /// Creates a new `DeleteEventSourceMapping`.
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
            crate::operation::delete_event_source_mapping::DeleteEventSourceMapping,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_event_source_mapping::DeleteEventSourceMappingError,
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
        crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_event_source_mapping::DeleteEventSourceMappingError,
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
    ///     let deserialized_parameters: crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_event_source_mapping().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The identifier of the event source mapping.</p>
    pub fn uuid(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.uuid(input.into());
        self
    }
    /// <p>The identifier of the event source mapping.</p>
    pub fn set_uuid(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_uuid(input);
        self
    }
}
