// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableEbsEncryptionByDefault`](crate::operation::enable_ebs_encryption_by_default::builders::EnableEbsEncryptionByDefaultFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::enable_ebs_encryption_by_default::builders::EnableEbsEncryptionByDefaultFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::enable_ebs_encryption_by_default::builders::EnableEbsEncryptionByDefaultFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`EnableEbsEncryptionByDefaultOutput`](crate::operation::enable_ebs_encryption_by_default::EnableEbsEncryptionByDefaultOutput) with field(s):
    ///   - [`ebs_encryption_by_default(Option<bool>)`](crate::operation::enable_ebs_encryption_by_default::EnableEbsEncryptionByDefaultOutput::ebs_encryption_by_default): <p>The updated status of encryption by default.</p>
    /// - On failure, responds with [`SdkError<EnableEbsEncryptionByDefaultError>`](crate::operation::enable_ebs_encryption_by_default::EnableEbsEncryptionByDefaultError)
    pub fn enable_ebs_encryption_by_default(&self) -> crate::operation::enable_ebs_encryption_by_default::builders::EnableEbsEncryptionByDefaultFluentBuilder{
        crate::operation::enable_ebs_encryption_by_default::builders::EnableEbsEncryptionByDefaultFluentBuilder::new(self.handle.clone())
    }
}
