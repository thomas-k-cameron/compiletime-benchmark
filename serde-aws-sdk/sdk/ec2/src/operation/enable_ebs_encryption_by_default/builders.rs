// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_ebs_encryption_by_default::_enable_ebs_encryption_by_default_output::EnableEbsEncryptionByDefaultOutputBuilder;

pub use crate::operation::enable_ebs_encryption_by_default::_enable_ebs_encryption_by_default_input::EnableEbsEncryptionByDefaultInputBuilder;

/// Fluent builder constructing a request to `EnableEbsEncryptionByDefault`.
///
/// <p>Enables EBS encryption by default for your account in the current Region.</p>
/// <p>After you enable encryption by default, the EBS volumes that you create are always encrypted, either using the default KMS key or the KMS key that you specified when you created each volume. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// <p>You can specify the default KMS key for encryption by default using <code>ModifyEbsDefaultKmsKeyId</code> or <code>ResetEbsDefaultKmsKeyId</code>.</p>
/// <p>Enabling encryption by default has no effect on the encryption status of your existing volumes.</p>
/// <p>After you enable encryption by default, you can no longer launch instances using instance types that do not support encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported instance types</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct EnableEbsEncryptionByDefaultFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::enable_ebs_encryption_by_default::builders::EnableEbsEncryptionByDefaultInputBuilder
            }
impl EnableEbsEncryptionByDefaultFluentBuilder {
    /// Creates a new `EnableEbsEncryptionByDefault`.
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
            crate::operation::enable_ebs_encryption_by_default::EnableEbsEncryptionByDefault,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::enable_ebs_encryption_by_default::EnableEbsEncryptionByDefaultError,
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
        crate::operation::enable_ebs_encryption_by_default::EnableEbsEncryptionByDefaultOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::enable_ebs_encryption_by_default::EnableEbsEncryptionByDefaultError,
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
    ///     let deserialized_parameters: crate::operation::enable_ebs_encryption_by_default::builders::EnableEbsEncryptionByDefaultInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.enable_ebs_encryption_by_default().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::enable_ebs_encryption_by_default::builders::EnableEbsEncryptionByDefaultInputBuilder,
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
}