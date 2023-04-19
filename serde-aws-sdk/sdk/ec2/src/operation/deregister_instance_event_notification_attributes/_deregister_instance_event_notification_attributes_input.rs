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
pub struct DeregisterInstanceEventNotificationAttributesInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>Information about the tag keys to deregister.</p>
    #[doc(hidden)]
    pub instance_tag_attribute:
        std::option::Option<crate::types::DeregisterInstanceTagAttributeRequest>,
}
impl DeregisterInstanceEventNotificationAttributesInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Information about the tag keys to deregister.</p>
    pub fn instance_tag_attribute(
        &self,
    ) -> std::option::Option<&crate::types::DeregisterInstanceTagAttributeRequest> {
        self.instance_tag_attribute.as_ref()
    }
}
impl DeregisterInstanceEventNotificationAttributesInput {
    /// Creates a new builder-style object to manufacture [`DeregisterInstanceEventNotificationAttributesInput`](crate::operation::deregister_instance_event_notification_attributes::DeregisterInstanceEventNotificationAttributesInput).
    pub fn builder() -> crate::operation::deregister_instance_event_notification_attributes::builders::DeregisterInstanceEventNotificationAttributesInputBuilder{
        crate::operation::deregister_instance_event_notification_attributes::builders::DeregisterInstanceEventNotificationAttributesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::deregister_instance_event_notification_attributes::DeregisterInstanceEventNotificationAttributesInput;
/// A builder for [`DeregisterInstanceEventNotificationAttributesInput`](crate::operation::deregister_instance_event_notification_attributes::DeregisterInstanceEventNotificationAttributesInput).
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
pub struct DeregisterInstanceEventNotificationAttributesInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) instance_tag_attribute:
        std::option::Option<crate::types::DeregisterInstanceTagAttributeRequest>,
}
impl DeregisterInstanceEventNotificationAttributesInputBuilder {
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
    /// <p>Information about the tag keys to deregister.</p>
    pub fn instance_tag_attribute(
        mut self,
        input: crate::types::DeregisterInstanceTagAttributeRequest,
    ) -> Self {
        self.instance_tag_attribute = Some(input);
        self
    }
    /// <p>Information about the tag keys to deregister.</p>
    pub fn set_instance_tag_attribute(
        mut self,
        input: std::option::Option<crate::types::DeregisterInstanceTagAttributeRequest>,
    ) -> Self {
        self.instance_tag_attribute = input;
        self
    }
    /// Consumes the builder and constructs a [`DeregisterInstanceEventNotificationAttributesInput`](crate::operation::deregister_instance_event_notification_attributes::DeregisterInstanceEventNotificationAttributesInput).
    pub fn build(self) -> Result<crate::operation::deregister_instance_event_notification_attributes::DeregisterInstanceEventNotificationAttributesInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::deregister_instance_event_notification_attributes::DeregisterInstanceEventNotificationAttributesInput {
                dry_run: self.dry_run
                ,
                instance_tag_attribute: self.instance_tag_attribute
                ,
            }
        )
    }
}
