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
pub struct DeleteTaskDefinitionsInput {
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to delete. You must specify a <code>revision</code>.</p>
    /// <p>You can specify up to 10 task definitions as a comma separated list.</p>
    #[doc(hidden)]
    pub task_definitions: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl DeleteTaskDefinitionsInput {
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to delete. You must specify a <code>revision</code>.</p>
    /// <p>You can specify up to 10 task definitions as a comma separated list.</p>
    pub fn task_definitions(&self) -> std::option::Option<&[std::string::String]> {
        self.task_definitions.as_deref()
    }
}
impl DeleteTaskDefinitionsInput {
    /// Creates a new builder-style object to manufacture [`DeleteTaskDefinitionsInput`](crate::operation::delete_task_definitions::DeleteTaskDefinitionsInput).
    pub fn builder(
    ) -> crate::operation::delete_task_definitions::builders::DeleteTaskDefinitionsInputBuilder
    {
        crate::operation::delete_task_definitions::builders::DeleteTaskDefinitionsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_task_definitions::DeleteTaskDefinitionsInput;
/// A builder for [`DeleteTaskDefinitionsInput`](crate::operation::delete_task_definitions::DeleteTaskDefinitionsInput).
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
pub struct DeleteTaskDefinitionsInputBuilder {
    pub(crate) task_definitions: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl DeleteTaskDefinitionsInputBuilder {
    /// Appends an item to `task_definitions`.
    ///
    /// To override the contents of this collection use [`set_task_definitions`](Self::set_task_definitions).
    ///
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to delete. You must specify a <code>revision</code>.</p>
    /// <p>You can specify up to 10 task definitions as a comma separated list.</p>
    pub fn task_definitions(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.task_definitions.unwrap_or_default();
        v.push(input.into());
        self.task_definitions = Some(v);
        self
    }
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to delete. You must specify a <code>revision</code>.</p>
    /// <p>You can specify up to 10 task definitions as a comma separated list.</p>
    pub fn set_task_definitions(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.task_definitions = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteTaskDefinitionsInput`](crate::operation::delete_task_definitions::DeleteTaskDefinitionsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_task_definitions::DeleteTaskDefinitionsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_task_definitions::DeleteTaskDefinitionsInput {
                task_definitions: self.task_definitions,
            },
        )
    }
}
