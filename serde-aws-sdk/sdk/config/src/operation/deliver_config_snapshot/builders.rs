// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deliver_config_snapshot::_deliver_config_snapshot_output::DeliverConfigSnapshotOutputBuilder;

pub use crate::operation::deliver_config_snapshot::_deliver_config_snapshot_input::DeliverConfigSnapshotInputBuilder;

/// Fluent builder constructing a request to `DeliverConfigSnapshot`.
///
/// <p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, Config sends the following notifications using an Amazon SNS topic that you have specified.</p>
/// <ul>
/// <li> <p>Notification of the start of the delivery.</p> </li>
/// <li> <p>Notification of the completion of the delivery, if the delivery was successfully completed.</p> </li>
/// <li> <p>Notification of delivery failure, if the delivery failed.</p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeliverConfigSnapshotFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotInputBuilder,
}
impl DeliverConfigSnapshotFluentBuilder {
    /// Creates a new `DeliverConfigSnapshot`.
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
            crate::operation::deliver_config_snapshot::DeliverConfigSnapshot,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::deliver_config_snapshot::DeliverConfigSnapshotError,
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
        crate::operation::deliver_config_snapshot::DeliverConfigSnapshotOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::deliver_config_snapshot::DeliverConfigSnapshotError,
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
    ///     let deserialized_parameters: crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.deliver_config_snapshot().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the delivery channel through which the snapshot is delivered.</p>
    pub fn delivery_channel_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.delivery_channel_name(input.into());
        self
    }
    /// <p>The name of the delivery channel through which the snapshot is delivered.</p>
    pub fn set_delivery_channel_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_delivery_channel_name(input);
        self
    }
}
