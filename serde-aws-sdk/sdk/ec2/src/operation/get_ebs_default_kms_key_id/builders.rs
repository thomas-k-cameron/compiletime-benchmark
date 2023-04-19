// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_ebs_default_kms_key_id::_get_ebs_default_kms_key_id_output::GetEbsDefaultKmsKeyIdOutputBuilder;

pub use crate::operation::get_ebs_default_kms_key_id::_get_ebs_default_kms_key_id_input::GetEbsDefaultKmsKeyIdInputBuilder;

/// Fluent builder constructing a request to `GetEbsDefaultKmsKeyId`.
///
/// <p>Describes the default KMS key for EBS encryption by default for your account in this Region. You can change the default KMS key for encryption by default using <code>ModifyEbsDefaultKmsKeyId</code> or <code>ResetEbsDefaultKmsKeyId</code>.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetEbsDefaultKmsKeyIdFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::get_ebs_default_kms_key_id::builders::GetEbsDefaultKmsKeyIdInputBuilder,
}
impl GetEbsDefaultKmsKeyIdFluentBuilder {
    /// Creates a new `GetEbsDefaultKmsKeyId`.
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
            crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyId,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdError,
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
        crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdError,
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
    ///     let deserialized_parameters: crate::operation::get_ebs_default_kms_key_id::builders::GetEbsDefaultKmsKeyIdInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_ebs_default_kms_key_id().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_ebs_default_kms_key_id::builders::GetEbsDefaultKmsKeyIdInputBuilder,
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
