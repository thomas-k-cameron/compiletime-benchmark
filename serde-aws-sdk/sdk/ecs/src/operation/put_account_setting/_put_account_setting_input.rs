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
pub struct PutAccountSettingInput {
    /// <p>The Amazon ECS resource name for which to modify the account setting. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected. If <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for your Amazon ECS container instances is affected. If <code>containerInsights</code> is specified, the default setting for CloudWatch Container Insights for your clusters is affected.</p>
    #[doc(hidden)]
    pub name: std::option::Option<crate::types::SettingName>,
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    #[doc(hidden)]
    pub value: std::option::Option<std::string::String>,
    /// <p>The ARN of the principal, which can be a user, role, or the root user. If you specify the root user, it modifies the account setting for all users, roles, and the root user of the account unless a user or role explicitly overrides these settings. If this field is omitted, the setting is changed only for the authenticated user.</p> <note>
    /// <p>Federated users assume the account setting of the root user and can't have explicit account settings set for them.</p>
    /// </note>
    #[doc(hidden)]
    pub principal_arn: std::option::Option<std::string::String>,
}
impl PutAccountSettingInput {
    /// <p>The Amazon ECS resource name for which to modify the account setting. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected. If <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for your Amazon ECS container instances is affected. If <code>containerInsights</code> is specified, the default setting for CloudWatch Container Insights for your clusters is affected.</p>
    pub fn name(&self) -> std::option::Option<&crate::types::SettingName> {
        self.name.as_ref()
    }
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    pub fn value(&self) -> std::option::Option<&str> {
        self.value.as_deref()
    }
    /// <p>The ARN of the principal, which can be a user, role, or the root user. If you specify the root user, it modifies the account setting for all users, roles, and the root user of the account unless a user or role explicitly overrides these settings. If this field is omitted, the setting is changed only for the authenticated user.</p> <note>
    /// <p>Federated users assume the account setting of the root user and can't have explicit account settings set for them.</p>
    /// </note>
    pub fn principal_arn(&self) -> std::option::Option<&str> {
        self.principal_arn.as_deref()
    }
}
impl PutAccountSettingInput {
    /// Creates a new builder-style object to manufacture [`PutAccountSettingInput`](crate::operation::put_account_setting::PutAccountSettingInput).
    pub fn builder(
    ) -> crate::operation::put_account_setting::builders::PutAccountSettingInputBuilder {
        crate::operation::put_account_setting::builders::PutAccountSettingInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::put_account_setting::PutAccountSettingInput;
/// A builder for [`PutAccountSettingInput`](crate::operation::put_account_setting::PutAccountSettingInput).
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
pub struct PutAccountSettingInputBuilder {
    pub(crate) name: std::option::Option<crate::types::SettingName>,
    pub(crate) value: std::option::Option<std::string::String>,
    pub(crate) principal_arn: std::option::Option<std::string::String>,
}
impl PutAccountSettingInputBuilder {
    /// <p>The Amazon ECS resource name for which to modify the account setting. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected. If <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for your Amazon ECS container instances is affected. If <code>containerInsights</code> is specified, the default setting for CloudWatch Container Insights for your clusters is affected.</p>
    pub fn name(mut self, input: crate::types::SettingName) -> Self {
        self.name = Some(input);
        self
    }
    /// <p>The Amazon ECS resource name for which to modify the account setting. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected. If <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for your Amazon ECS container instances is affected. If <code>containerInsights</code> is specified, the default setting for CloudWatch Container Insights for your clusters is affected.</p>
    pub fn set_name(mut self, input: std::option::Option<crate::types::SettingName>) -> Self {
        self.name = input;
        self
    }
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
        self.value = Some(input.into());
        self
    }
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>The ARN of the principal, which can be a user, role, or the root user. If you specify the root user, it modifies the account setting for all users, roles, and the root user of the account unless a user or role explicitly overrides these settings. If this field is omitted, the setting is changed only for the authenticated user.</p> <note>
    /// <p>Federated users assume the account setting of the root user and can't have explicit account settings set for them.</p>
    /// </note>
    pub fn principal_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.principal_arn = Some(input.into());
        self
    }
    /// <p>The ARN of the principal, which can be a user, role, or the root user. If you specify the root user, it modifies the account setting for all users, roles, and the root user of the account unless a user or role explicitly overrides these settings. If this field is omitted, the setting is changed only for the authenticated user.</p> <note>
    /// <p>Federated users assume the account setting of the root user and can't have explicit account settings set for them.</p>
    /// </note>
    pub fn set_principal_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.principal_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`PutAccountSettingInput`](crate::operation::put_account_setting::PutAccountSettingInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::put_account_setting::PutAccountSettingInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::put_account_setting::PutAccountSettingInput {
                name: self.name,
                value: self.value,
                principal_arn: self.principal_arn,
            },
        )
    }
}