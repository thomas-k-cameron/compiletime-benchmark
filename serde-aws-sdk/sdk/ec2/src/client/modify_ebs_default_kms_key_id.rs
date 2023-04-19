// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyEbsDefaultKmsKeyId`](crate::operation::modify_ebs_default_kms_key_id::builders::ModifyEbsDefaultKmsKeyIdFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`kms_key_id(impl Into<String>)`](crate::operation::modify_ebs_default_kms_key_id::builders::ModifyEbsDefaultKmsKeyIdFluentBuilder::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::operation::modify_ebs_default_kms_key_id::builders::ModifyEbsDefaultKmsKeyIdFluentBuilder::set_kms_key_id): <p>The identifier of the Key Management Service (KMS) KMS key to use for Amazon EBS encryption. If this parameter is not specified, your KMS key for Amazon EBS is used. If <code>KmsKeyId</code> is specified, the encrypted state must be <code>true</code>.</p>  <p>You can specify the KMS key using any of the following:</p>  <ul>   <li> <p>Key ID. For example, 1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>   <li> <p>Key alias. For example, alias/ExampleAlias.</p> </li>   <li> <p>Key ARN. For example, arn:aws:kms:us-east-1:012345678910:key/1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>   <li> <p>Alias ARN. For example, arn:aws:kms:us-east-1:012345678910:alias/ExampleAlias.</p> </li>  </ul>  <p>Amazon Web Services authenticates the KMS key asynchronously. Therefore, if you specify an ID, alias, or ARN that is not valid, the action can appear to complete, but eventually fails.</p>  <p>Amazon EBS does not support asymmetric KMS keys.</p>
    ///   - [`dry_run(bool)`](crate::operation::modify_ebs_default_kms_key_id::builders::ModifyEbsDefaultKmsKeyIdFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_ebs_default_kms_key_id::builders::ModifyEbsDefaultKmsKeyIdFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ModifyEbsDefaultKmsKeyIdOutput`](crate::operation::modify_ebs_default_kms_key_id::ModifyEbsDefaultKmsKeyIdOutput) with field(s):
    ///   - [`kms_key_id(Option<String>)`](crate::operation::modify_ebs_default_kms_key_id::ModifyEbsDefaultKmsKeyIdOutput::kms_key_id): <p>The Amazon Resource Name (ARN) of the default KMS key for encryption by default.</p>
    /// - On failure, responds with [`SdkError<ModifyEbsDefaultKmsKeyIdError>`](crate::operation::modify_ebs_default_kms_key_id::ModifyEbsDefaultKmsKeyIdError)
    pub fn modify_ebs_default_kms_key_id(&self) -> crate::operation::modify_ebs_default_kms_key_id::builders::ModifyEbsDefaultKmsKeyIdFluentBuilder{
        crate::operation::modify_ebs_default_kms_key_id::builders::ModifyEbsDefaultKmsKeyIdFluentBuilder::new(self.handle.clone())
    }
}
