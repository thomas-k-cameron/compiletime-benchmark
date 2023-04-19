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
pub struct DeleteVpcEndpointConnectionNotificationsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The IDs of the notifications.</p>
    #[doc(hidden)]
    pub connection_notification_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl DeleteVpcEndpointConnectionNotificationsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IDs of the notifications.</p>
    pub fn connection_notification_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.connection_notification_ids.as_deref()
    }
}
impl DeleteVpcEndpointConnectionNotificationsInput {
    /// Creates a new builder-style object to manufacture [`DeleteVpcEndpointConnectionNotificationsInput`](crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotificationsInput).
    pub fn builder() -> crate::operation::delete_vpc_endpoint_connection_notifications::builders::DeleteVpcEndpointConnectionNotificationsInputBuilder{
        crate::operation::delete_vpc_endpoint_connection_notifications::builders::DeleteVpcEndpointConnectionNotificationsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotificationsInput;
/// A builder for [`DeleteVpcEndpointConnectionNotificationsInput`](crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotificationsInput).
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
pub struct DeleteVpcEndpointConnectionNotificationsInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) connection_notification_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl DeleteVpcEndpointConnectionNotificationsInputBuilder {
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
    /// Appends an item to `connection_notification_ids`.
    ///
    /// To override the contents of this collection use [`set_connection_notification_ids`](Self::set_connection_notification_ids).
    ///
    /// <p>The IDs of the notifications.</p>
    pub fn connection_notification_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.connection_notification_ids.unwrap_or_default();
        v.push(input.into());
        self.connection_notification_ids = Some(v);
        self
    }
    /// <p>The IDs of the notifications.</p>
    pub fn set_connection_notification_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.connection_notification_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVpcEndpointConnectionNotificationsInput`](crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotificationsInput).
    pub fn build(self) -> Result<crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotificationsInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::delete_vpc_endpoint_connection_notifications::DeleteVpcEndpointConnectionNotificationsInput {
                dry_run: self.dry_run
                ,
                connection_notification_ids: self.connection_notification_ids
                ,
            }
        )
    }
}