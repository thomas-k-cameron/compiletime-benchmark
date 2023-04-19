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
pub struct UpdateTaskSetOutput {
    /// <p>Details about the task set.</p>
    #[doc(hidden)]
    pub task_set: std::option::Option<crate::types::TaskSet>,
    _request_id: Option<String>,
}
impl UpdateTaskSetOutput {
    /// <p>Details about the task set.</p>
    pub fn task_set(&self) -> std::option::Option<&crate::types::TaskSet> {
        self.task_set.as_ref()
    }
}
impl aws_http::request_id::RequestId for UpdateTaskSetOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateTaskSetOutput {
    /// Creates a new builder-style object to manufacture [`UpdateTaskSetOutput`](crate::operation::update_task_set::UpdateTaskSetOutput).
    pub fn builder() -> crate::operation::update_task_set::builders::UpdateTaskSetOutputBuilder {
        crate::operation::update_task_set::builders::UpdateTaskSetOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::update_task_set::UpdateTaskSetOutput;
/// A builder for [`UpdateTaskSetOutput`](crate::operation::update_task_set::UpdateTaskSetOutput).
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
pub struct UpdateTaskSetOutputBuilder {
    pub(crate) task_set: std::option::Option<crate::types::TaskSet>,
    _request_id: Option<String>,
}
impl UpdateTaskSetOutputBuilder {
    /// <p>Details about the task set.</p>
    pub fn task_set(mut self, input: crate::types::TaskSet) -> Self {
        self.task_set = Some(input);
        self
    }
    /// <p>Details about the task set.</p>
    pub fn set_task_set(mut self, input: std::option::Option<crate::types::TaskSet>) -> Self {
        self.task_set = input;
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
    /// Consumes the builder and constructs a [`UpdateTaskSetOutput`](crate::operation::update_task_set::UpdateTaskSetOutput).
    pub fn build(self) -> crate::operation::update_task_set::UpdateTaskSetOutput {
        crate::operation::update_task_set::UpdateTaskSetOutput {
            task_set: self.task_set,
            _request_id: self._request_id,
        }
    }
}
