// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AbortVaultLock`](crate::operation::abort_vault_lock::builders::AbortVaultLockFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::abort_vault_lock::builders::AbortVaultLockFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::abort_vault_lock::builders::AbortVaultLockFluentBuilder::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    ///   - [`vault_name(impl Into<String>)`](crate::operation::abort_vault_lock::builders::AbortVaultLockFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::abort_vault_lock::builders::AbortVaultLockFluentBuilder::set_vault_name): <p>The name of the vault.</p>
    /// - On success, responds with [`AbortVaultLockOutput`](crate::operation::abort_vault_lock::AbortVaultLockOutput)
    /// - On failure, responds with [`SdkError<AbortVaultLockError>`](crate::operation::abort_vault_lock::AbortVaultLockError)
    pub fn abort_vault_lock(
        &self,
    ) -> crate::operation::abort_vault_lock::builders::AbortVaultLockFluentBuilder {
        crate::operation::abort_vault_lock::builders::AbortVaultLockFluentBuilder::new(
            self.handle.clone(),
        )
    }
}