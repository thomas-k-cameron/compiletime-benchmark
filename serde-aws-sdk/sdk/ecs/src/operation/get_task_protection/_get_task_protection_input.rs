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
pub struct GetTaskProtectionInput {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task sets exist in.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<std::string::String>,
    /// <p>A list of up to 100 task IDs or full ARN entries.</p>
    #[doc(hidden)]
    pub tasks: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GetTaskProtectionInput {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task sets exist in.</p>
    pub fn cluster(&self) -> std::option::Option<&str> {
        self.cluster.as_deref()
    }
    /// <p>A list of up to 100 task IDs or full ARN entries.</p>
    pub fn tasks(&self) -> std::option::Option<&[std::string::String]> {
        self.tasks.as_deref()
    }
}
impl GetTaskProtectionInput {
    /// Creates a new builder-style object to manufacture [`GetTaskProtectionInput`](crate::operation::get_task_protection::GetTaskProtectionInput).
    pub fn builder(
    ) -> crate::operation::get_task_protection::builders::GetTaskProtectionInputBuilder {
        crate::operation::get_task_protection::builders::GetTaskProtectionInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_task_protection::GetTaskProtectionInput;
/// A builder for [`GetTaskProtectionInput`](crate::operation::get_task_protection::GetTaskProtectionInput).
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
pub struct GetTaskProtectionInputBuilder {
    pub(crate) cluster: std::option::Option<std::string::String>,
    pub(crate) tasks: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GetTaskProtectionInputBuilder {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task sets exist in.</p>
    pub fn cluster(mut self, input: impl Into<std::string::String>) -> Self {
        self.cluster = Some(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task sets exist in.</p>
    pub fn set_cluster(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cluster = input;
        self
    }
    /// Appends an item to `tasks`.
    ///
    /// To override the contents of this collection use [`set_tasks`](Self::set_tasks).
    ///
    /// <p>A list of up to 100 task IDs or full ARN entries.</p>
    pub fn tasks(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.tasks.unwrap_or_default();
        v.push(input.into());
        self.tasks = Some(v);
        self
    }
    /// <p>A list of up to 100 task IDs or full ARN entries.</p>
    pub fn set_tasks(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.tasks = input;
        self
    }
    /// Consumes the builder and constructs a [`GetTaskProtectionInput`](crate::operation::get_task_protection::GetTaskProtectionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_task_protection::GetTaskProtectionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_task_protection::GetTaskProtectionInput {
                cluster: self.cluster,
                tasks: self.tasks,
            },
        )
    }
}
