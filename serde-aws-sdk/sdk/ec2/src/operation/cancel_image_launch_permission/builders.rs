// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_image_launch_permission::_cancel_image_launch_permission_output::CancelImageLaunchPermissionOutputBuilder;

pub use crate::operation::cancel_image_launch_permission::_cancel_image_launch_permission_input::CancelImageLaunchPermissionInputBuilder;

/// Fluent builder constructing a request to `CancelImageLaunchPermission`.
///
/// <p>Removes your Amazon Web Services account from the launch permissions for the specified AMI. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/cancel-sharing-an-AMI.html"> Cancel having an AMI shared with your Amazon Web Services account</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CancelImageLaunchPermissionFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::cancel_image_launch_permission::builders::CancelImageLaunchPermissionInputBuilder
            }
impl CancelImageLaunchPermissionFluentBuilder {
    /// Creates a new `CancelImageLaunchPermission`.
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
            crate::operation::cancel_image_launch_permission::CancelImageLaunchPermission,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::cancel_image_launch_permission::CancelImageLaunchPermissionError,
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
        crate::operation::cancel_image_launch_permission::CancelImageLaunchPermissionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::cancel_image_launch_permission::CancelImageLaunchPermissionError,
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
    ///     let deserialized_parameters: crate::operation::cancel_image_launch_permission::builders::CancelImageLaunchPermissionInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.cancel_image_launch_permission().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::cancel_image_launch_permission::builders::CancelImageLaunchPermissionInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the AMI that was shared with your Amazon Web Services account.</p>
    pub fn image_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.image_id(input.into());
        self
    }
    /// <p>The ID of the AMI that was shared with your Amazon Web Services account.</p>
    pub fn set_image_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_image_id(input);
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
