// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_vpc_endpoint_connection_notifications::_delete_vpc_endpoint_connection_notifications_output::DeleteVpcEndpointConnectionNotificationsOutputBuilder;

pub use crate::operation::delete_vpc_endpoint_connection_notifications::_delete_vpc_endpoint_connection_notifications_input::DeleteVpcEndpointConnectionNotificationsInputBuilder;

/// Fluent builder constructing a request to `DeleteVpcEndpointConnectionNotifications`.
///
/// <p>Deletes the specified VPC endpoint connection notifications.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteVpcEndpointConnectionNotificationsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_vpc_endpoint_connection_notifications::builders::DeleteVpcEndpointConnectionNotificationsInputBuilder
            }
impl DeleteVpcEndpointConnectionNotificationsFluentBuilder {
    /// Creates a new `DeleteVpcEndpointConnectionNotifications`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotifications, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotificationsError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotificationsOutput, aws_smithy_http::result::SdkError<crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotificationsError>>
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
    ///     let deserialized_parameters: crate::operation::delete_vpc_endpoint_connection_notifications::builders::DeleteVpcEndpointConnectionNotificationsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_vpc_endpoint_connection_notifications().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_vpc_endpoint_connection_notifications::builders::DeleteVpcEndpointConnectionNotificationsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `ConnectionNotificationIds`.
    ///
    /// To override the contents of this collection use [`set_connection_notification_ids`](Self::set_connection_notification_ids).
    ///
    /// <p>The IDs of the notifications.</p>
    pub fn connection_notification_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.connection_notification_ids(input.into());
        self
    }
    /// <p>The IDs of the notifications.</p>
    pub fn set_connection_notification_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_connection_notification_ids(input);
        self
    }
}
