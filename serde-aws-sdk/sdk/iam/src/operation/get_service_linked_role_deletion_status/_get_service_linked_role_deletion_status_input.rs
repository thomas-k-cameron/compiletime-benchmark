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
pub struct GetServiceLinkedRoleDeletionStatusInput {
    /// <p>The deletion task identifier. This identifier is returned by the <code>DeleteServiceLinkedRole</code> operation in the format <code>task/aws-service-role/
    /// <service-principal-name>
    /// /
    /// <role-name>
    /// /
    /// <task-uuid></task-uuid>
    /// </role-name>
    /// </service-principal-name></code>.</p>
    #[doc(hidden)]
    pub deletion_task_id: std::option::Option<std::string::String>,
}
impl GetServiceLinkedRoleDeletionStatusInput {
    /// <p>The deletion task identifier. This identifier is returned by the <code>DeleteServiceLinkedRole</code> operation in the format <code>task/aws-service-role/
    /// <service-principal-name>
    /// /
    /// <role-name>
    /// /
    /// <task-uuid></task-uuid>
    /// </role-name>
    /// </service-principal-name></code>.</p>
    pub fn deletion_task_id(&self) -> std::option::Option<&str> {
        self.deletion_task_id.as_deref()
    }
}
impl GetServiceLinkedRoleDeletionStatusInput {
    /// Creates a new builder-style object to manufacture [`GetServiceLinkedRoleDeletionStatusInput`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusInput).
    pub fn builder() -> crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusInputBuilder{
        crate::operation::get_service_linked_role_deletion_status::builders::GetServiceLinkedRoleDeletionStatusInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusInput;
/// A builder for [`GetServiceLinkedRoleDeletionStatusInput`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusInput).
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
pub struct GetServiceLinkedRoleDeletionStatusInputBuilder {
    pub(crate) deletion_task_id: std::option::Option<std::string::String>,
}
impl GetServiceLinkedRoleDeletionStatusInputBuilder {
    /// <p>The deletion task identifier. This identifier is returned by the <code>DeleteServiceLinkedRole</code> operation in the format <code>task/aws-service-role/
    /// <service-principal-name>
    /// /
    /// <role-name>
    /// /
    /// <task-uuid></task-uuid>
    /// </role-name>
    /// </service-principal-name></code>.</p>
    pub fn deletion_task_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.deletion_task_id = Some(input.into());
        self
    }
    /// <p>The deletion task identifier. This identifier is returned by the <code>DeleteServiceLinkedRole</code> operation in the format <code>task/aws-service-role/
    /// <service-principal-name>
    /// /
    /// <role-name>
    /// /
    /// <task-uuid></task-uuid>
    /// </role-name>
    /// </service-principal-name></code>.</p>
    pub fn set_deletion_task_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.deletion_task_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetServiceLinkedRoleDeletionStatusInput`](crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusInput).
    pub fn build(self) -> Result<crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusInput {
                deletion_task_id: self.deletion_task_id
                ,
            }
        )
    }
}
