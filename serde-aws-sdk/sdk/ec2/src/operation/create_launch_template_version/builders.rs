// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_launch_template_version::_create_launch_template_version_output::CreateLaunchTemplateVersionOutputBuilder;

pub use crate::operation::create_launch_template_version::_create_launch_template_version_input::CreateLaunchTemplateVersionInputBuilder;

/// Fluent builder constructing a request to `CreateLaunchTemplateVersion`.
///
/// <p>Creates a new version of a launch template. You can specify an existing version of launch template from which to base the new version.</p>
/// <p>Launch template versions are numbered in the order in which they are created. You cannot specify, change, or replace the numbering of launch template versions.</p>
/// <p>Launch templates are immutable; after you create a launch template, you can't modify it. Instead, you can create a new version of the launch template that includes any changes you require.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html#manage-launch-template-versions">Modify a launch template (manage launch template versions)</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateLaunchTemplateVersionFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionInputBuilder
            }
impl CreateLaunchTemplateVersionFluentBuilder {
    /// Creates a new `CreateLaunchTemplateVersion`.
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
            crate::operation::create_launch_template_version::CreateLaunchTemplateVersion,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_launch_template_version::CreateLaunchTemplateVersionError,
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
        crate::operation::create_launch_template_version::CreateLaunchTemplateVersionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_launch_template_version::CreateLaunchTemplateVersionError,
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
    ///     let deserialized_parameters: crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_launch_template_version().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    /// <p>Constraint: Maximum 128 ASCII characters.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    /// <p>Constraint: Maximum 128 ASCII characters.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify either the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn launch_template_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.launch_template_id(input.into());
        self
    }
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify either the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn set_launch_template_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_launch_template_id(input);
        self
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn launch_template_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.launch_template_name(input.into());
        self
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn set_launch_template_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_launch_template_name(input);
        self
    }
    /// <p>The version number of the launch template version on which to base the new version. The new version inherits the same launch parameters as the source version, except for parameters that you specify in <code>LaunchTemplateData</code>. Snapshots applied to the block device mapping are ignored when creating a new version unless they are explicitly included.</p>
    pub fn source_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.source_version(input.into());
        self
    }
    /// <p>The version number of the launch template version on which to base the new version. The new version inherits the same launch parameters as the source version, except for parameters that you specify in <code>LaunchTemplateData</code>. Snapshots applied to the block device mapping are ignored when creating a new version unless they are explicitly included.</p>
    pub fn set_source_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_source_version(input);
        self
    }
    /// <p>A description for the version of the launch template.</p>
    pub fn version_description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.version_description(input.into());
        self
    }
    /// <p>A description for the version of the launch template.</p>
    pub fn set_version_description(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_version_description(input);
        self
    }
    /// <p>The information for the launch template.</p>
    pub fn launch_template_data(mut self, input: crate::types::RequestLaunchTemplateData) -> Self {
        self.inner = self.inner.launch_template_data(input);
        self
    }
    /// <p>The information for the launch template.</p>
    pub fn set_launch_template_data(
        mut self,
        input: std::option::Option<crate::types::RequestLaunchTemplateData>,
    ) -> Self {
        self.inner = self.inner.set_launch_template_data(input);
        self
    }
    /// <p>If <code>true</code>, and if a Systems Manager parameter is specified for <code>ImageId</code>, the AMI ID is displayed in the response for <code>imageID</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html#use-an-ssm-parameter-instead-of-an-ami-id">Use a Systems Manager parameter instead of an AMI ID</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>Default: <code>false</code> </p>
    pub fn resolve_alias(mut self, input: bool) -> Self {
        self.inner = self.inner.resolve_alias(input);
        self
    }
    /// <p>If <code>true</code>, and if a Systems Manager parameter is specified for <code>ImageId</code>, the AMI ID is displayed in the response for <code>imageID</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html#use-an-ssm-parameter-instead-of-an-ami-id">Use a Systems Manager parameter instead of an AMI ID</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>Default: <code>false</code> </p>
    pub fn set_resolve_alias(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_resolve_alias(input);
        self
    }
}
