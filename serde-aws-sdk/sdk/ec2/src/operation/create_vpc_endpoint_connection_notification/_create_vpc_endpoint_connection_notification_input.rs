// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateVpcEndpointConnectionNotificationInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The ID of the endpoint service.</p>
    #[doc(hidden)]
    pub service_id: std::option::Option<std::string::String>,
    /// <p>The ID of the endpoint.</p>
    #[doc(hidden)]
    pub vpc_endpoint_id: std::option::Option<std::string::String>,
    /// <p>The ARN of the SNS topic for the notifications.</p>
    #[doc(hidden)]
    pub connection_notification_arn: std::option::Option<std::string::String>,
    /// <p>The endpoint events for which to receive notifications. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    #[doc(hidden)]
    pub connection_events: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
}
impl CreateVpcEndpointConnectionNotificationInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the endpoint service.</p>
    pub fn service_id(&self) -> std::option::Option<&str> {
        self.service_id.as_deref()
    }
    /// <p>The ID of the endpoint.</p>
    pub fn vpc_endpoint_id(&self) -> std::option::Option<&str> {
        self.vpc_endpoint_id.as_deref()
    }
    /// <p>The ARN of the SNS topic for the notifications.</p>
    pub fn connection_notification_arn(&self) -> std::option::Option<&str> {
        self.connection_notification_arn.as_deref()
    }
    /// <p>The endpoint events for which to receive notifications. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn connection_events(&self) -> std::option::Option<&[std::string::String]> {
        self.connection_events.as_deref()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl CreateVpcEndpointConnectionNotificationInput {
    /// Creates a new builder-style object to manufacture [`CreateVpcEndpointConnectionNotificationInput`](crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationInput).
    pub fn builder() -> crate::operation::create_vpc_endpoint_connection_notification::builders::CreateVpcEndpointConnectionNotificationInputBuilder{
        crate::operation::create_vpc_endpoint_connection_notification::builders::CreateVpcEndpointConnectionNotificationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationInput;
/// A builder for [`CreateVpcEndpointConnectionNotificationInput`](crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct CreateVpcEndpointConnectionNotificationInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) service_id: std::option::Option<std::string::String>,
    pub(crate) vpc_endpoint_id: std::option::Option<std::string::String>,
    pub(crate) connection_notification_arn: std::option::Option<std::string::String>,
    pub(crate) connection_events: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) client_token: std::option::Option<std::string::String>,
}
impl CreateVpcEndpointConnectionNotificationInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The ID of the endpoint service.</p>
    pub fn service_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.service_id = Some(input.into());
        self
    }
    /// <p>The ID of the endpoint service.</p>
    pub fn set_service_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.service_id = input;
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn vpc_endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_endpoint_id = Some(input.into());
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn set_vpc_endpoint_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vpc_endpoint_id = input;
        self
    }
    /// <p>The ARN of the SNS topic for the notifications.</p>
    pub fn connection_notification_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.connection_notification_arn = Some(input.into());
        self
    }
    /// <p>The ARN of the SNS topic for the notifications.</p>
    pub fn set_connection_notification_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.connection_notification_arn = input;
        self
    }
    /// Appends an item to `connection_events`.
    ///
    /// To override the contents of this collection use [`set_connection_events`](Self::set_connection_events).
    ///
    /// <p>The endpoint events for which to receive notifications. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn connection_events(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.connection_events.unwrap_or_default();
        v.push(input.into());
        self.connection_events = Some(v);
        self
    }
    /// <p>The endpoint events for which to receive notifications. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn set_connection_events(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.connection_events = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateVpcEndpointConnectionNotificationInput`](crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationInput).
    pub fn build(self) -> Result<crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::create_vpc_endpoint_connection_notification::CreateVpcEndpointConnectionNotificationInput {
                dry_run: self.dry_run
                ,
                service_id: self.service_id
                ,
                vpc_endpoint_id: self.vpc_endpoint_id
                ,
                connection_notification_arn: self.connection_notification_arn
                ,
                connection_events: self.connection_events
                ,
                client_token: self.client_token
                ,
            }
        )
    }
}
