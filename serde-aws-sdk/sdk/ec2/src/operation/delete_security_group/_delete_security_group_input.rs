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
pub struct DeleteSecurityGroupInput {
    /// <p>The ID of the security group. Required for a nondefault VPC.</p>
    #[doc(hidden)]
    pub group_id: std::option::Option<std::string::String>,
    /// <p>[EC2-Classic, default VPC] The name of the security group. You can specify either the security group name or the security group ID. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    #[doc(hidden)]
    pub group_name: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeleteSecurityGroupInput {
    /// <p>The ID of the security group. Required for a nondefault VPC.</p>
    pub fn group_id(&self) -> std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>[EC2-Classic, default VPC] The name of the security group. You can specify either the security group name or the security group ID. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn group_name(&self) -> std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteSecurityGroupInput {
    /// Creates a new builder-style object to manufacture [`DeleteSecurityGroupInput`](crate::operation::delete_security_group::DeleteSecurityGroupInput).
    pub fn builder(
    ) -> crate::operation::delete_security_group::builders::DeleteSecurityGroupInputBuilder {
        crate::operation::delete_security_group::builders::DeleteSecurityGroupInputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_security_group::DeleteSecurityGroupInput;
/// A builder for [`DeleteSecurityGroupInput`](crate::operation::delete_security_group::DeleteSecurityGroupInput).
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
pub struct DeleteSecurityGroupInputBuilder {
    pub(crate) group_id: std::option::Option<std::string::String>,
    pub(crate) group_name: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeleteSecurityGroupInputBuilder {
    /// <p>The ID of the security group. Required for a nondefault VPC.</p>
    pub fn group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.group_id = Some(input.into());
        self
    }
    /// <p>The ID of the security group. Required for a nondefault VPC.</p>
    pub fn set_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>[EC2-Classic, default VPC] The name of the security group. You can specify either the security group name or the security group ID. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.group_name = Some(input.into());
        self
    }
    /// <p>[EC2-Classic, default VPC] The name of the security group. You can specify either the security group name or the security group ID. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn set_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteSecurityGroupInput`](crate::operation::delete_security_group::DeleteSecurityGroupInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_security_group::DeleteSecurityGroupInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_security_group::DeleteSecurityGroupInput {
                group_id: self.group_id,
                group_name: self.group_name,
                dry_run: self.dry_run,
            },
        )
    }
}
