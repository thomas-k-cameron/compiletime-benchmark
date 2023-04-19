// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetKeyRotationStatus`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::set_key_id): <p>Gets the rotation status for the specified KMS key.</p>  <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    /// - On success, responds with [`GetKeyRotationStatusOutput`](crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput) with field(s):
    ///   - [`key_rotation_enabled(bool)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput::key_rotation_enabled): <p>A Boolean value that specifies whether key rotation is enabled.</p>
    /// - On failure, responds with [`SdkError<GetKeyRotationStatusError>`](crate::operation::get_key_rotation_status::GetKeyRotationStatusError)
    pub fn get_key_rotation_status(
        &self,
    ) -> crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder
    {
        crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
