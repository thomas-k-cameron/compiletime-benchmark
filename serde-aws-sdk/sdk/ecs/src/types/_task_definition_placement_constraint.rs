// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object representing a constraint on task placement in the task definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html">Task placement constraints</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note>
/// <p>Task placement constraints aren't supported for tasks run on Fargate.</p>
/// </note>
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
pub struct TaskDefinitionPlacementConstraint {
    /// <p>The type of constraint. The <code>MemberOf</code> constraint restricts selection to be from a group of valid candidates.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::types::TaskDefinitionPlacementConstraintType>,
    /// <p>A cluster query language expression to apply to the constraint. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster query language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[doc(hidden)]
    pub expression: std::option::Option<std::string::String>,
}
impl TaskDefinitionPlacementConstraint {
    /// <p>The type of constraint. The <code>MemberOf</code> constraint restricts selection to be from a group of valid candidates.</p>
    pub fn r#type(
        &self,
    ) -> std::option::Option<&crate::types::TaskDefinitionPlacementConstraintType> {
        self.r#type.as_ref()
    }
    /// <p>A cluster query language expression to apply to the constraint. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster query language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    pub fn expression(&self) -> std::option::Option<&str> {
        self.expression.as_deref()
    }
}
impl TaskDefinitionPlacementConstraint {
    /// Creates a new builder-style object to manufacture [`TaskDefinitionPlacementConstraint`](crate::types::TaskDefinitionPlacementConstraint).
    pub fn builder() -> crate::types::builders::TaskDefinitionPlacementConstraintBuilder {
        crate::types::builders::TaskDefinitionPlacementConstraintBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TaskDefinitionPlacementConstraint;
/// A builder for [`TaskDefinitionPlacementConstraint`](crate::types::TaskDefinitionPlacementConstraint).
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
pub struct TaskDefinitionPlacementConstraintBuilder {
    pub(crate) r#type: std::option::Option<crate::types::TaskDefinitionPlacementConstraintType>,
    pub(crate) expression: std::option::Option<std::string::String>,
}
impl TaskDefinitionPlacementConstraintBuilder {
    /// <p>The type of constraint. The <code>MemberOf</code> constraint restricts selection to be from a group of valid candidates.</p>
    pub fn r#type(mut self, input: crate::types::TaskDefinitionPlacementConstraintType) -> Self {
        self.r#type = Some(input);
        self
    }
    /// <p>The type of constraint. The <code>MemberOf</code> constraint restricts selection to be from a group of valid candidates.</p>
    pub fn set_type(
        mut self,
        input: std::option::Option<crate::types::TaskDefinitionPlacementConstraintType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>A cluster query language expression to apply to the constraint. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster query language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    pub fn expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.expression = Some(input.into());
        self
    }
    /// <p>A cluster query language expression to apply to the constraint. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster query language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    pub fn set_expression(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.expression = input;
        self
    }
    /// Consumes the builder and constructs a [`TaskDefinitionPlacementConstraint`](crate::types::TaskDefinitionPlacementConstraint).
    pub fn build(self) -> crate::types::TaskDefinitionPlacementConstraint {
        crate::types::TaskDefinitionPlacementConstraint {
            r#type: self.r#type,
            expression: self.expression,
        }
    }
}
