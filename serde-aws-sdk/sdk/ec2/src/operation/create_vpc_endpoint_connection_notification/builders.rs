// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_vpc_endpoint_connection_notification::_create_vpc_endpoint_connection_notification_output::CreateVpcEndpointConnectionNotificationOutputBuilder;

pub use crate::operation::create_vpc_endpoint_connection_notification::_create_vpc_endpoint_connection_notification_input::CreateVpcEndpointConnectionNotificationInputBuilder;

/// Fluent builder constructing a request to `CreateVpcEndpointConnectionNotification`.
///
/// <p>Creates a connection notification for a specified VPC endpoint or VPC endpoint service. A connection notification notifies you of specific endpoint events. You must create an SNS topic to receive notifications. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Create a Topic</a> in the <i>Amazon Simple Notification Service Developer Guide</i>.</p>
/// <p>You can create a connection notification for interface endpoints only.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateVpcEndpointConnectionNotificationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_vpc_endpoint_connection_notification::builders::CreateVpcEndpointConnectionNotificationInputBuilder
            }
impl CreateVpcEndpointConnectionNotificationFluentBuilder {
    /// Creates a new `CreateVpcEndpointConnectionNotification`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotification, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationOutput, aws_smithy_http::result::SdkError<crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationError>>
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
    ///     let deserialized_parameters: crate::operation::create_vpc_endpoint_connection_notification::builders::CreateVpcEndpointConnectionNotificationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_vpc_endpoint_connection_notification().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_vpc_endpoint_connection_notification::builders::CreateVpcEndpointConnectionNotificationInputBuilder,
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
    /// <p>The ID of the endpoint service.</p>
    pub fn service_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_id(input.into());
        self
    }
    /// <p>The ID of the endpoint service.</p>
    pub fn set_service_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_service_id(input);
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn vpc_endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpc_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn set_vpc_endpoint_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_endpoint_id(input);
        self
    }
    /// <p>The ARN of the SNS topic for the notifications.</p>
    pub fn connection_notification_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.connection_notification_arn(input.into());
        self
    }
    /// <p>The ARN of the SNS topic for the notifications.</p>
    pub fn set_connection_notification_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_connection_notification_arn(input);
        self
    }
    /// Appends an item to `ConnectionEvents`.
    ///
    /// To override the contents of this collection use [`set_connection_events`](Self::set_connection_events).
    ///
    /// <p>The endpoint events for which to receive notifications. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn connection_events(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.connection_events(input.into());
        self
    }
    /// <p>The endpoint events for which to receive notifications. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn set_connection_events(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_connection_events(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
