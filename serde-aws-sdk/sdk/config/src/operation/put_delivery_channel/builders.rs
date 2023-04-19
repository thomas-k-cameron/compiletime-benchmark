// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_delivery_channel::_put_delivery_channel_output::PutDeliveryChannelOutputBuilder;

pub use crate::operation::put_delivery_channel::_put_delivery_channel_input::PutDeliveryChannelInputBuilder;

/// Fluent builder constructing a request to `PutDeliveryChannel`.
///
/// <p>Creates a delivery channel object to deliver configuration information to an Amazon S3 bucket and Amazon SNS topic.</p>
/// <p>Before you can create a delivery channel, you must create a configuration recorder.</p>
/// <p>You can use this action to change the Amazon S3 bucket or an Amazon SNS topic of the existing delivery channel. To change the Amazon S3 bucket or an Amazon SNS topic, call this action and specify the changed values for the S3 bucket and the SNS topic. If you specify a different value for either the S3 bucket or the SNS topic, this action will keep the existing value for the parameter that is not changed.</p> <note>
/// <p>You can have only one delivery channel per region in your account.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutDeliveryChannelFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_delivery_channel::builders::PutDeliveryChannelInputBuilder,
}
impl PutDeliveryChannelFluentBuilder {
    /// Creates a new `PutDeliveryChannel`.
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
            crate::operation::put_delivery_channel::PutDeliveryChannel,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_delivery_channel::PutDeliveryChannelError,
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
        crate::operation::put_delivery_channel::PutDeliveryChannelOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_delivery_channel::PutDeliveryChannelError,
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
    ///     let deserialized_parameters: crate::operation::put_delivery_channel::builders::PutDeliveryChannelInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.put_delivery_channel().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::put_delivery_channel::builders::PutDeliveryChannelInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The configuration delivery channel object that delivers the configuration information to an Amazon S3 bucket and to an Amazon SNS topic.</p>
    pub fn delivery_channel(mut self, input: crate::types::DeliveryChannel) -> Self {
        self.inner = self.inner.delivery_channel(input);
        self
    }
    /// <p>The configuration delivery channel object that delivers the configuration information to an Amazon S3 bucket and to an Amazon SNS topic.</p>
    pub fn set_delivery_channel(
        mut self,
        input: std::option::Option<crate::types::DeliveryChannel>,
    ) -> Self {
        self.inner = self.inner.set_delivery_channel(input);
        self
    }
}
