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
pub struct PutUserPermissionsBoundaryInput {
    /// <p>The name (friendly name, not ARN) of the IAM user for which you want to set the permissions boundary.</p>
    #[doc(hidden)]
    pub user_name: std::option::Option<std::string::String>,
    /// <p>The ARN of the managed policy that is used to set the permissions boundary for the user.</p>
    /// <p>A permissions boundary policy defines the maximum permissions that identity-based policies can grant to an entity, but does not grant permissions. Permissions boundaries do not define the maximum permissions that a resource-based policy can grant to an entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries for IAM entities</a> in the <i>IAM User Guide</i>.</p>
    /// <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types </a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub permissions_boundary: std::option::Option<std::string::String>,
}
impl PutUserPermissionsBoundaryInput {
    /// <p>The name (friendly name, not ARN) of the IAM user for which you want to set the permissions boundary.</p>
    pub fn user_name(&self) -> std::option::Option<&str> {
        self.user_name.as_deref()
    }
    /// <p>The ARN of the managed policy that is used to set the permissions boundary for the user.</p>
    /// <p>A permissions boundary policy defines the maximum permissions that identity-based policies can grant to an entity, but does not grant permissions. Permissions boundaries do not define the maximum permissions that a resource-based policy can grant to an entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries for IAM entities</a> in the <i>IAM User Guide</i>.</p>
    /// <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types </a> in the <i>IAM User Guide</i>.</p>
    pub fn permissions_boundary(&self) -> std::option::Option<&str> {
        self.permissions_boundary.as_deref()
    }
}
impl PutUserPermissionsBoundaryInput {
    /// Creates a new builder-style object to manufacture [`PutUserPermissionsBoundaryInput`](crate::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryInput).
    pub fn builder() -> crate::operation::put_user_permissions_boundary::builders::PutUserPermissionsBoundaryInputBuilder{
        crate::operation::put_user_permissions_boundary::builders::PutUserPermissionsBoundaryInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryInput;
/// A builder for [`PutUserPermissionsBoundaryInput`](crate::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryInput).
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
pub struct PutUserPermissionsBoundaryInputBuilder {
    pub(crate) user_name: std::option::Option<std::string::String>,
    pub(crate) permissions_boundary: std::option::Option<std::string::String>,
}
impl PutUserPermissionsBoundaryInputBuilder {
    /// <p>The name (friendly name, not ARN) of the IAM user for which you want to set the permissions boundary.</p>
    pub fn user_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.user_name = Some(input.into());
        self
    }
    /// <p>The name (friendly name, not ARN) of the IAM user for which you want to set the permissions boundary.</p>
    pub fn set_user_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.user_name = input;
        self
    }
    /// <p>The ARN of the managed policy that is used to set the permissions boundary for the user.</p>
    /// <p>A permissions boundary policy defines the maximum permissions that identity-based policies can grant to an entity, but does not grant permissions. Permissions boundaries do not define the maximum permissions that a resource-based policy can grant to an entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries for IAM entities</a> in the <i>IAM User Guide</i>.</p>
    /// <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types </a> in the <i>IAM User Guide</i>.</p>
    pub fn permissions_boundary(mut self, input: impl Into<std::string::String>) -> Self {
        self.permissions_boundary = Some(input.into());
        self
    }
    /// <p>The ARN of the managed policy that is used to set the permissions boundary for the user.</p>
    /// <p>A permissions boundary policy defines the maximum permissions that identity-based policies can grant to an entity, but does not grant permissions. Permissions boundaries do not define the maximum permissions that a resource-based policy can grant to an entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries for IAM entities</a> in the <i>IAM User Guide</i>.</p>
    /// <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types </a> in the <i>IAM User Guide</i>.</p>
    pub fn set_permissions_boundary(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.permissions_boundary = input;
        self
    }
    /// Consumes the builder and constructs a [`PutUserPermissionsBoundaryInput`](crate::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryInput {
                user_name: self.user_name,
                permissions_boundary: self.permissions_boundary,
            },
        )
    }
}
