// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAccountPasswordPolicy`](crate::operation::delete_account_password_policy::builders::DeleteAccountPasswordPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::delete_account_password_policy::builders::DeleteAccountPasswordPolicyFluentBuilder::send) it.
    /// - On success, responds with [`DeleteAccountPasswordPolicyOutput`](crate::operation::delete_account_password_policy::DeleteAccountPasswordPolicyOutput)
    /// - On failure, responds with [`SdkError<DeleteAccountPasswordPolicyError>`](crate::operation::delete_account_password_policy::DeleteAccountPasswordPolicyError)
    pub fn delete_account_password_policy(&self) -> crate::operation::delete_account_password_policy::builders::DeleteAccountPasswordPolicyFluentBuilder{
        crate::operation::delete_account_password_policy::builders::DeleteAccountPasswordPolicyFluentBuilder::new(self.handle.clone())
    }
}
