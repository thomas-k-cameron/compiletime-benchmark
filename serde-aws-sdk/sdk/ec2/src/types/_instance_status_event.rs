// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a scheduled event for an instance.</p>
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
pub struct InstanceStatusEvent {
    /// <p>The ID of the event.</p>
    #[doc(hidden)]
    pub instance_event_id: std::option::Option<std::string::String>,
    /// <p>The event code.</p>
    #[doc(hidden)]
    pub code: std::option::Option<crate::types::EventCode>,
    /// <p>A description of the event.</p>
    /// <p>After a scheduled event is completed, it can still be described for up to a week. If the event has been completed, this description starts with the following text: [Completed].</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The latest scheduled end time for the event.</p>
    #[doc(hidden)]
    pub not_after: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The earliest scheduled start time for the event.</p>
    #[doc(hidden)]
    pub not_before: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The deadline for starting the event.</p>
    #[doc(hidden)]
    pub not_before_deadline: std::option::Option<aws_smithy_types::DateTime>,
}
impl InstanceStatusEvent {
    /// <p>The ID of the event.</p>
    pub fn instance_event_id(&self) -> std::option::Option<&str> {
        self.instance_event_id.as_deref()
    }
    /// <p>The event code.</p>
    pub fn code(&self) -> std::option::Option<&crate::types::EventCode> {
        self.code.as_ref()
    }
    /// <p>A description of the event.</p>
    /// <p>After a scheduled event is completed, it can still be described for up to a week. If the event has been completed, this description starts with the following text: [Completed].</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The latest scheduled end time for the event.</p>
    pub fn not_after(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.not_after.as_ref()
    }
    /// <p>The earliest scheduled start time for the event.</p>
    pub fn not_before(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.not_before.as_ref()
    }
    /// <p>The deadline for starting the event.</p>
    pub fn not_before_deadline(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.not_before_deadline.as_ref()
    }
}
impl InstanceStatusEvent {
    /// Creates a new builder-style object to manufacture [`InstanceStatusEvent`](crate::types::InstanceStatusEvent).
    pub fn builder() -> crate::types::builders::InstanceStatusEventBuilder {
        crate::types::builders::InstanceStatusEventBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::InstanceStatusEvent;
/// A builder for [`InstanceStatusEvent`](crate::types::InstanceStatusEvent).
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
pub struct InstanceStatusEventBuilder {
    pub(crate) instance_event_id: std::option::Option<std::string::String>,
    pub(crate) code: std::option::Option<crate::types::EventCode>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) not_after: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) not_before: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) not_before_deadline: std::option::Option<aws_smithy_types::DateTime>,
}
impl InstanceStatusEventBuilder {
    /// <p>The ID of the event.</p>
    pub fn instance_event_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_event_id = Some(input.into());
        self
    }
    /// <p>The ID of the event.</p>
    pub fn set_instance_event_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.instance_event_id = input;
        self
    }
    /// <p>The event code.</p>
    pub fn code(mut self, input: crate::types::EventCode) -> Self {
        self.code = Some(input);
        self
    }
    /// <p>The event code.</p>
    pub fn set_code(mut self, input: std::option::Option<crate::types::EventCode>) -> Self {
        self.code = input;
        self
    }
    /// <p>A description of the event.</p>
    /// <p>After a scheduled event is completed, it can still be described for up to a week. If the event has been completed, this description starts with the following text: [Completed].</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A description of the event.</p>
    /// <p>After a scheduled event is completed, it can still be described for up to a week. If the event has been completed, this description starts with the following text: [Completed].</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The latest scheduled end time for the event.</p>
    pub fn not_after(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.not_after = Some(input);
        self
    }
    /// <p>The latest scheduled end time for the event.</p>
    pub fn set_not_after(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.not_after = input;
        self
    }
    /// <p>The earliest scheduled start time for the event.</p>
    pub fn not_before(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.not_before = Some(input);
        self
    }
    /// <p>The earliest scheduled start time for the event.</p>
    pub fn set_not_before(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.not_before = input;
        self
    }
    /// <p>The deadline for starting the event.</p>
    pub fn not_before_deadline(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.not_before_deadline = Some(input);
        self
    }
    /// <p>The deadline for starting the event.</p>
    pub fn set_not_before_deadline(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.not_before_deadline = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceStatusEvent`](crate::types::InstanceStatusEvent).
    pub fn build(self) -> crate::types::InstanceStatusEvent {
        crate::types::InstanceStatusEvent {
            instance_event_id: self.instance_event_id,
            code: self.code,
            description: self.description,
            not_after: self.not_after,
            not_before: self.not_before,
            not_before_deadline: self.not_before_deadline,
        }
    }
}
