// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_account_setting::_put_account_setting_output::PutAccountSettingOutputBuilder;

pub use crate::operation::put_account_setting::_put_account_setting_input::PutAccountSettingInputBuilder;

/// Fluent builder constructing a request to `PutAccountSetting`.
///
/// <p>Modifies an account setting. Account settings are set on a per-Region basis.</p>
/// <p>If you change the account setting for the root user, the default settings for all of the users and roles that no individual account setting was specified are reset for. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html">Account Settings</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
/// <p>When <code>serviceLongArnFormat</code>, <code>taskLongArnFormat</code>, or <code>containerInstanceLongArnFormat</code> are specified, the Amazon Resource Name (ARN) and resource ID format of the resource type for a specified user, role, or the root user for an account is affected. The opt-in and opt-out account setting must be set for each Amazon ECS resource separately. The ARN and resource ID format of a resource is defined by the opt-in status of the user or role that created the resource. You must turn on this setting to use Amazon ECS features such as resource tagging.</p>
/// <p>When <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for any new container instances that support the feature is changed. If <code>awsvpcTrunking</code> is enabled, any new container instances that support the feature are launched have the increased ENI limits available to them. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-eni.html">Elastic Network Interface Trunking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
/// <p>When <code>containerInsights</code> is specified, the default setting indicating whether CloudWatch Container Insights is enabled for your clusters is changed. If <code>containerInsights</code> is enabled, any new clusters that are created will have Container Insights enabled unless you disable it during cluster creation. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cloudwatch-container-insights.html">CloudWatch Container Insights</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutAccountSettingFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_account_setting::builders::PutAccountSettingInputBuilder,
}
impl PutAccountSettingFluentBuilder {
    /// Creates a new `PutAccountSetting`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::put_account_setting::PutAccountSetting,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_account_setting::PutAccountSettingError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::put_account_setting::PutAccountSettingOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_account_setting::PutAccountSettingError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::put_account_setting::builders::PutAccountSettingInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.put_account_setting().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::put_account_setting::builders::PutAccountSettingInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The Amazon ECS resource name for which to modify the account setting. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected. If <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for your Amazon ECS container instances is affected. If <code>containerInsights</code> is specified, the default setting for CloudWatch Container Insights for your clusters is affected.</p>
    pub fn name(mut self, input: crate::types::SettingName) -> Self {
        self.inner = self.inner.name(input);
        self
    }
    /// <p>The Amazon ECS resource name for which to modify the account setting. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected. If <code>awsvpcTrunking</code> is specified, the elastic network interface (ENI) limit for your Amazon ECS container instances is affected. If <code>containerInsights</code> is specified, the default setting for CloudWatch Container Insights for your clusters is affected.</p>
    pub fn set_name(mut self, input: std::option::Option<crate::types::SettingName>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.value(input.into());
        self
    }
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_value(input);
        self
    }
    /// <p>The ARN of the principal, which can be a user, role, or the root user. If you specify the root user, it modifies the account setting for all users, roles, and the root user of the account unless a user or role explicitly overrides these settings. If this field is omitted, the setting is changed only for the authenticated user.</p> <note>
    /// <p>Federated users assume the account setting of the root user and can't have explicit account settings set for them.</p>
    /// </note>
    pub fn principal_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.principal_arn(input.into());
        self
    }
    /// <p>The ARN of the principal, which can be a user, role, or the root user. If you specify the root user, it modifies the account setting for all users, roles, and the root user of the account unless a user or role explicitly overrides these settings. If this field is omitted, the setting is changed only for the authenticated user.</p> <note>
    /// <p>Federated users assume the account setting of the root user and can't have explicit account settings set for them.</p>
    /// </note>
    pub fn set_principal_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_principal_arn(input);
        self
    }
}