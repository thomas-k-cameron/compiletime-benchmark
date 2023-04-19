// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a volume status event.</p>
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
pub struct VolumeStatusEvent {
    /// <p>A description of the event.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The ID of this event.</p>
    #[doc(hidden)]
    pub event_id: std::option::Option<std::string::String>,
    /// <p>The type of this event.</p>
    #[doc(hidden)]
    pub event_type: std::option::Option<std::string::String>,
    /// <p>The latest end time of the event.</p>
    #[doc(hidden)]
    pub not_after: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The earliest start time of the event.</p>
    #[doc(hidden)]
    pub not_before: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The ID of the instance associated with the event.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
}
impl VolumeStatusEvent {
    /// <p>A description of the event.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The ID of this event.</p>
    pub fn event_id(&self) -> std::option::Option<&str> {
        self.event_id.as_deref()
    }
    /// <p>The type of this event.</p>
    pub fn event_type(&self) -> std::option::Option<&str> {
        self.event_type.as_deref()
    }
    /// <p>The latest end time of the event.</p>
    pub fn not_after(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.not_after.as_ref()
    }
    /// <p>The earliest start time of the event.</p>
    pub fn not_before(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.not_before.as_ref()
    }
    /// <p>The ID of the instance associated with the event.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
}
impl VolumeStatusEvent {
    /// Creates a new builder-style object to manufacture [`VolumeStatusEvent`](crate::types::VolumeStatusEvent).
    pub fn builder() -> crate::types::builders::VolumeStatusEventBuilder {
        crate::types::builders::VolumeStatusEventBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VolumeStatusEvent;
/// A builder for [`VolumeStatusEvent`](crate::types::VolumeStatusEvent).
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
pub struct VolumeStatusEventBuilder {
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) event_id: std::option::Option<std::string::String>,
    pub(crate) event_type: std::option::Option<std::string::String>,
    pub(crate) not_after: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) not_before: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
}
impl VolumeStatusEventBuilder {
    /// <p>A description of the event.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A description of the event.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The ID of this event.</p>
    pub fn event_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.event_id = Some(input.into());
        self
    }
    /// <p>The ID of this event.</p>
    pub fn set_event_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.event_id = input;
        self
    }
    /// <p>The type of this event.</p>
    pub fn event_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.event_type = Some(input.into());
        self
    }
    /// <p>The type of this event.</p>
    pub fn set_event_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.event_type = input;
        self
    }
    /// <p>The latest end time of the event.</p>
    pub fn not_after(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.not_after = Some(input);
        self
    }
    /// <p>The latest end time of the event.</p>
    pub fn set_not_after(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.not_after = input;
        self
    }
    /// <p>The earliest start time of the event.</p>
    pub fn not_before(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.not_before = Some(input);
        self
    }
    /// <p>The earliest start time of the event.</p>
    pub fn set_not_before(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.not_before = input;
        self
    }
    /// <p>The ID of the instance associated with the event.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance associated with the event.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// Consumes the builder and constructs a [`VolumeStatusEvent`](crate::types::VolumeStatusEvent).
    pub fn build(self) -> crate::types::VolumeStatusEvent {
        crate::types::VolumeStatusEvent {
            description: self.description,
            event_id: self.event_id,
            event_type: self.event_type,
            not_after: self.not_after,
            not_before: self.not_before,
            instance_id: self.instance_id,
        }
    }
}
