// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_fast_launch::_enable_fast_launch_output::EnableFastLaunchOutputBuilder;

pub use crate::operation::enable_fast_launch::_enable_fast_launch_input::EnableFastLaunchInputBuilder;

/// Fluent builder constructing a request to `EnableFastLaunch`.
///
/// <p>When you enable faster launching for a Windows AMI, images are pre-provisioned, using snapshots to launch instances up to 65% faster. To create the optimized Windows image, Amazon EC2 launches an instance and runs through Sysprep steps, rebooting as required. Then it creates a set of reserved snapshots that are used for subsequent launches. The reserved snapshots are automatically replenished as they are used, depending on your settings for launch frequency.</p> <note>
/// <p>To change these settings, you must own the AMI.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct EnableFastLaunchFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable_fast_launch::builders::EnableFastLaunchInputBuilder,
}
impl EnableFastLaunchFluentBuilder {
    /// Creates a new `EnableFastLaunch`.
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
            crate::operation::enable_fast_launch::EnableFastLaunch,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::enable_fast_launch::EnableFastLaunchError,
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
        crate::operation::enable_fast_launch::EnableFastLaunchOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::enable_fast_launch::EnableFastLaunchError,
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
    ///     let deserialized_parameters: crate::operation::enable_fast_launch::builders::EnableFastLaunchInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.enable_fast_launch().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::enable_fast_launch::builders::EnableFastLaunchInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the image for which you’re enabling faster launching.</p>
    pub fn image_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.image_id(input.into());
        self
    }
    /// <p>The ID of the image for which you’re enabling faster launching.</p>
    pub fn set_image_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_image_id(input);
        self
    }
    /// <p>The type of resource to use for pre-provisioning the Windows AMI for faster launching. Supported values include: <code>snapshot</code>, which is the default value.</p>
    pub fn resource_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_type(input.into());
        self
    }
    /// <p>The type of resource to use for pre-provisioning the Windows AMI for faster launching. Supported values include: <code>snapshot</code>, which is the default value.</p>
    pub fn set_resource_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>Configuration settings for creating and managing the snapshots that are used for pre-provisioning the Windows AMI for faster launching. The associated <code>ResourceType</code> must be <code>snapshot</code>.</p>
    pub fn snapshot_configuration(
        mut self,
        input: crate::types::FastLaunchSnapshotConfigurationRequest,
    ) -> Self {
        self.inner = self.inner.snapshot_configuration(input);
        self
    }
    /// <p>Configuration settings for creating and managing the snapshots that are used for pre-provisioning the Windows AMI for faster launching. The associated <code>ResourceType</code> must be <code>snapshot</code>.</p>
    pub fn set_snapshot_configuration(
        mut self,
        input: std::option::Option<crate::types::FastLaunchSnapshotConfigurationRequest>,
    ) -> Self {
        self.inner = self.inner.set_snapshot_configuration(input);
        self
    }
    /// <p>The launch template to use when launching Windows instances from pre-provisioned snapshots. Launch template parameters can include either the name or ID of the launch template, but not both.</p>
    pub fn launch_template(
        mut self,
        input: crate::types::FastLaunchLaunchTemplateSpecificationRequest,
    ) -> Self {
        self.inner = self.inner.launch_template(input);
        self
    }
    /// <p>The launch template to use when launching Windows instances from pre-provisioned snapshots. Launch template parameters can include either the name or ID of the launch template, but not both.</p>
    pub fn set_launch_template(
        mut self,
        input: std::option::Option<crate::types::FastLaunchLaunchTemplateSpecificationRequest>,
    ) -> Self {
        self.inner = self.inner.set_launch_template(input);
        self
    }
    /// <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows faster launching. Value must be <code>6</code> or greater.</p>
    pub fn max_parallel_launches(mut self, input: i32) -> Self {
        self.inner = self.inner.max_parallel_launches(input);
        self
    }
    /// <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows faster launching. Value must be <code>6</code> or greater.</p>
    pub fn set_max_parallel_launches(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_parallel_launches(input);
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
}
