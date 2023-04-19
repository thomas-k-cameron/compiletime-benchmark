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
pub struct ModifyInstanceEventWindowOutput {
    /// <p>Information about the event window.</p>
    #[doc(hidden)]
    pub instance_event_window: std::option::Option<crate::types::InstanceEventWindow>,
    _request_id: Option<String>,
}
impl ModifyInstanceEventWindowOutput {
    /// <p>Information about the event window.</p>
    pub fn instance_event_window(&self) -> std::option::Option<&crate::types::InstanceEventWindow> {
        self.instance_event_window.as_ref()
    }
}
impl aws_http::request_id::RequestId for ModifyInstanceEventWindowOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyInstanceEventWindowOutput {
    /// Creates a new builder-style object to manufacture [`ModifyInstanceEventWindowOutput`](crate::operation::modify_instance_event_window::ModifyInstanceEventWindowOutput).
    pub fn builder() -> crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowOutputBuilder{
        crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_instance_event_window::ModifyInstanceEventWindowOutput;
/// A builder for [`ModifyInstanceEventWindowOutput`](crate::operation::modify_instance_event_window::ModifyInstanceEventWindowOutput).
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
pub struct ModifyInstanceEventWindowOutputBuilder {
    pub(crate) instance_event_window: std::option::Option<crate::types::InstanceEventWindow>,
    _request_id: Option<String>,
}
impl ModifyInstanceEventWindowOutputBuilder {
    /// <p>Information about the event window.</p>
    pub fn instance_event_window(mut self, input: crate::types::InstanceEventWindow) -> Self {
        self.instance_event_window = Some(input);
        self
    }
    /// <p>Information about the event window.</p>
    pub fn set_instance_event_window(
        mut self,
        input: std::option::Option<crate::types::InstanceEventWindow>,
    ) -> Self {
        self.instance_event_window = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifyInstanceEventWindowOutput`](crate::operation::modify_instance_event_window::ModifyInstanceEventWindowOutput).
    pub fn build(
        self,
    ) -> crate::operation::modify_instance_event_window::ModifyInstanceEventWindowOutput {
        crate::operation::modify_instance_event_window::ModifyInstanceEventWindowOutput {
            instance_event_window: self.instance_event_window,
            _request_id: self._request_id,
        }
    }
}