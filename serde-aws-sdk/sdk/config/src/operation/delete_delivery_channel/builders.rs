// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_delivery_channel::_delete_delivery_channel_output::DeleteDeliveryChannelOutputBuilder;

pub use crate::operation::delete_delivery_channel::_delete_delivery_channel_input::DeleteDeliveryChannelInputBuilder;

/// Fluent builder constructing a request to `DeleteDeliveryChannel`.
///
/// <p>Deletes the delivery channel.</p>
/// <p>Before you can delete the delivery channel, you must stop the configuration recorder by using the <code>StopConfigurationRecorder</code> action.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDeliveryChannelFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_delivery_channel::builders::DeleteDeliveryChannelInputBuilder,
}
impl DeleteDeliveryChannelFluentBuilder {
    /// Creates a new `DeleteDeliveryChannel`.
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
            crate::operation::delete_delivery_channel::DeleteDeliveryChannel,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_delivery_channel::DeleteDeliveryChannelError,
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
        crate::operation::delete_delivery_channel::DeleteDeliveryChannelOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_delivery_channel::DeleteDeliveryChannelError,
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
    ///     let deserialized_parameters: crate::operation::delete_delivery_channel::builders::DeleteDeliveryChannelInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_delivery_channel().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_delivery_channel::builders::DeleteDeliveryChannelInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the delivery channel to delete.</p>
    pub fn delivery_channel_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.delivery_channel_name(input.into());
        self
    }
    /// <p>The name of the delivery channel to delete.</p>
    pub fn set_delivery_channel_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_delivery_channel_name(input);
        self
    }
}
