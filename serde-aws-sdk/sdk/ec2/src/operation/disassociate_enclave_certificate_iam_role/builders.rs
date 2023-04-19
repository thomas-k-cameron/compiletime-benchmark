// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_enclave_certificate_iam_role::_disassociate_enclave_certificate_iam_role_output::DisassociateEnclaveCertificateIamRoleOutputBuilder;

pub use crate::operation::disassociate_enclave_certificate_iam_role::_disassociate_enclave_certificate_iam_role_input::DisassociateEnclaveCertificateIamRoleInputBuilder;

/// Fluent builder constructing a request to `DisassociateEnclaveCertificateIamRole`.
///
/// <p>Disassociates an IAM role from an Certificate Manager (ACM) certificate. Disassociating an IAM role from an ACM certificate removes the Amazon S3 object that contains the certificate, certificate chain, and encrypted private key from the Amazon S3 bucket. It also revokes the IAM role's permission to use the KMS key used to encrypt the private key. This effectively revokes the role's permission to use the certificate.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateEnclaveCertificateIamRoleFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::disassociate_enclave_certificate_iam_role::builders::DisassociateEnclaveCertificateIamRoleInputBuilder
            }
impl DisassociateEnclaveCertificateIamRoleFluentBuilder {
    /// Creates a new `DisassociateEnclaveCertificateIamRole`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRole, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleOutput, aws_smithy_http::result::SdkError<crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::disassociate_enclave_certificate_iam_role::builders::DisassociateEnclaveCertificateIamRoleInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.disassociate_enclave_certificate_iam_role().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::disassociate_enclave_certificate_iam_role::builders::DisassociateEnclaveCertificateIamRoleInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ARN of the ACM certificate from which to disassociate the IAM role.</p>
    pub fn certificate_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.certificate_arn(input.into());
        self
    }
    /// <p>The ARN of the ACM certificate from which to disassociate the IAM role.</p>
    pub fn set_certificate_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_certificate_arn(input);
        self
    }
    /// <p>The ARN of the IAM role to disassociate.</p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM role to disassociate.</p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
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
