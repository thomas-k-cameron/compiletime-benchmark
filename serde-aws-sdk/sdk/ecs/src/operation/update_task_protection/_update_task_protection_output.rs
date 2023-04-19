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
pub struct UpdateTaskProtectionOutput {
    /// <p>A list of tasks with the following information.</p>
    /// <ul>
    /// <li> <p> <code>taskArn</code>: The task ARN.</p> </li>
    /// <li> <p> <code>protectionEnabled</code>: The protection status of the task. If scale-in protection is enabled for a task, the value is <code>true</code>. Otherwise, it is <code>false</code>.</p> </li>
    /// <li> <p> <code>expirationDate</code>: The epoch time when protection for the task will expire.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub protected_tasks: std::option::Option<std::vec::Vec<crate::types::ProtectedTask>>,
    /// <p>Any failures associated with the call.</p>
    #[doc(hidden)]
    pub failures: std::option::Option<std::vec::Vec<crate::types::Failure>>,
    _request_id: Option<String>,
}
impl UpdateTaskProtectionOutput {
    /// <p>A list of tasks with the following information.</p>
    /// <ul>
    /// <li> <p> <code>taskArn</code>: The task ARN.</p> </li>
    /// <li> <p> <code>protectionEnabled</code>: The protection status of the task. If scale-in protection is enabled for a task, the value is <code>true</code>. Otherwise, it is <code>false</code>.</p> </li>
    /// <li> <p> <code>expirationDate</code>: The epoch time when protection for the task will expire.</p> </li>
    /// </ul>
    pub fn protected_tasks(&self) -> std::option::Option<&[crate::types::ProtectedTask]> {
        self.protected_tasks.as_deref()
    }
    /// <p>Any failures associated with the call.</p>
    pub fn failures(&self) -> std::option::Option<&[crate::types::Failure]> {
        self.failures.as_deref()
    }
}
impl aws_http::request_id::RequestId for UpdateTaskProtectionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateTaskProtectionOutput {
    /// Creates a new builder-style object to manufacture [`UpdateTaskProtectionOutput`](crate::operation::update_task_protection::UpdateTaskProtectionOutput).
    pub fn builder(
    ) -> crate::operation::update_task_protection::builders::UpdateTaskProtectionOutputBuilder {
        crate::operation::update_task_protection::builders::UpdateTaskProtectionOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::update_task_protection::UpdateTaskProtectionOutput;
/// A builder for [`UpdateTaskProtectionOutput`](crate::operation::update_task_protection::UpdateTaskProtectionOutput).
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
pub struct UpdateTaskProtectionOutputBuilder {
    pub(crate) protected_tasks: std::option::Option<std::vec::Vec<crate::types::ProtectedTask>>,
    pub(crate) failures: std::option::Option<std::vec::Vec<crate::types::Failure>>,
    _request_id: Option<String>,
}
impl UpdateTaskProtectionOutputBuilder {
    /// Appends an item to `protected_tasks`.
    ///
    /// To override the contents of this collection use [`set_protected_tasks`](Self::set_protected_tasks).
    ///
    /// <p>A list of tasks with the following information.</p>
    /// <ul>
    /// <li> <p> <code>taskArn</code>: The task ARN.</p> </li>
    /// <li> <p> <code>protectionEnabled</code>: The protection status of the task. If scale-in protection is enabled for a task, the value is <code>true</code>. Otherwise, it is <code>false</code>.</p> </li>
    /// <li> <p> <code>expirationDate</code>: The epoch time when protection for the task will expire.</p> </li>
    /// </ul>
    pub fn protected_tasks(mut self, input: crate::types::ProtectedTask) -> Self {
        let mut v = self.protected_tasks.unwrap_or_default();
        v.push(input);
        self.protected_tasks = Some(v);
        self
    }
    /// <p>A list of tasks with the following information.</p>
    /// <ul>
    /// <li> <p> <code>taskArn</code>: The task ARN.</p> </li>
    /// <li> <p> <code>protectionEnabled</code>: The protection status of the task. If scale-in protection is enabled for a task, the value is <code>true</code>. Otherwise, it is <code>false</code>.</p> </li>
    /// <li> <p> <code>expirationDate</code>: The epoch time when protection for the task will expire.</p> </li>
    /// </ul>
    pub fn set_protected_tasks(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ProtectedTask>>,
    ) -> Self {
        self.protected_tasks = input;
        self
    }
    /// Appends an item to `failures`.
    ///
    /// To override the contents of this collection use [`set_failures`](Self::set_failures).
    ///
    /// <p>Any failures associated with the call.</p>
    pub fn failures(mut self, input: crate::types::Failure) -> Self {
        let mut v = self.failures.unwrap_or_default();
        v.push(input);
        self.failures = Some(v);
        self
    }
    /// <p>Any failures associated with the call.</p>
    pub fn set_failures(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Failure>>,
    ) -> Self {
        self.failures = input;
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
    /// Consumes the builder and constructs a [`UpdateTaskProtectionOutput`](crate::operation::update_task_protection::UpdateTaskProtectionOutput).
    pub fn build(self) -> crate::operation::update_task_protection::UpdateTaskProtectionOutput {
        crate::operation::update_task_protection::UpdateTaskProtectionOutput {
            protected_tasks: self.protected_tasks,
            failures: self.failures,
            _request_id: self._request_id,
        }
    }
}