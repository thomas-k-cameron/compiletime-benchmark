// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateEnclaveCertificateIamRole`](crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`certificate_arn(impl Into<String>)`](crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleFluentBuilder::certificate_arn) / [`set_certificate_arn(Option<String>)`](crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleFluentBuilder::set_certificate_arn): <p>The ARN of the ACM certificate with which to associate the IAM role.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleFluentBuilder::set_role_arn): <p>The ARN of the IAM role to associate with the ACM certificate. You can associate up to 16 IAM roles with an ACM certificate.</p>
    ///   - [`dry_run(bool)`](crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`AssociateEnclaveCertificateIamRoleOutput`](crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleOutput) with field(s):
    ///   - [`certificate_s3_bucket_name(Option<String>)`](crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleOutput::certificate_s3_bucket_name): <p>The name of the Amazon S3 bucket to which the certificate was uploaded.</p>
    ///   - [`certificate_s3_object_key(Option<String>)`](crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleOutput::certificate_s3_object_key): <p>The Amazon S3 object key where the certificate, certificate chain, and encrypted private key bundle are stored. The object key is formatted as follows: <code>role_arn</code>/<code>certificate_arn</code>.</p>
    ///   - [`encryption_kms_key_id(Option<String>)`](crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleOutput::encryption_kms_key_id): <p>The ID of the KMS key used to encrypt the private key of the certificate.</p>
    /// - On failure, responds with [`SdkError<AssociateEnclaveCertificateIamRoleError>`](crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleError)
    pub fn associate_enclave_certificate_iam_role(&self) -> crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleFluentBuilder{
        crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleFluentBuilder::new(self.handle.clone())
    }
}
