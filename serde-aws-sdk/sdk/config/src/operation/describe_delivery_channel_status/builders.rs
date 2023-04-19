// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_delivery_channel_status::_describe_delivery_channel_status_output::DescribeDeliveryChannelStatusOutputBuilder;

pub use crate::operation::describe_delivery_channel_status::_describe_delivery_channel_status_input::DescribeDeliveryChannelStatusInputBuilder;

/// Fluent builder constructing a request to `DescribeDeliveryChannelStatus`.
///
/// <p>Returns the current status of the specified delivery channel. If a delivery channel is not specified, this action returns the current status of all delivery channels associated with the account.</p> <note>
/// <p>Currently, you can specify only one delivery channel per region in your account.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDeliveryChannelStatusFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_delivery_channel_status::builders::DescribeDeliveryChannelStatusInputBuilder
            }
impl DescribeDeliveryChannelStatusFluentBuilder {
    /// Creates a new `DescribeDeliveryChannelStatus`.
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
            crate::operation::describe_delivery_channel_status::DescribeDeliveryChannelStatus,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_delivery_channel_status::DescribeDeliveryChannelStatusError,
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
        crate::operation::describe_delivery_channel_status::DescribeDeliveryChannelStatusOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_delivery_channel_status::DescribeDeliveryChannelStatusError,
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
    ///     let deserialized_parameters: crate::operation::describe_delivery_channel_status::builders::DescribeDeliveryChannelStatusInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_delivery_channel_status().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_delivery_channel_status::builders::DescribeDeliveryChannelStatusInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Appends an item to `DeliveryChannelNames`.
    ///
    /// To override the contents of this collection use [`set_delivery_channel_names`](Self::set_delivery_channel_names).
    ///
    /// <p>A list of delivery channel names.</p>
    pub fn delivery_channel_names(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.delivery_channel_names(input.into());
        self
    }
    /// <p>A list of delivery channel names.</p>
    pub fn set_delivery_channel_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_delivery_channel_names(input);
        self
    }
}