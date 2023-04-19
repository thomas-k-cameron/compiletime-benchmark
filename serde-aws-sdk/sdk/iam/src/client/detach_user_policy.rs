// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DetachUserPolicy`](crate::operation::detach_user_policy::builders::DetachUserPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::detach_user_policy::builders::DetachUserPolicyFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::detach_user_policy::builders::DetachUserPolicyFluentBuilder::set_user_name): <p>The name (friendly name, not ARN) of the IAM user to detach the policy from.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`policy_arn(impl Into<String>)`](crate::operation::detach_user_policy::builders::DetachUserPolicyFluentBuilder::policy_arn) / [`set_policy_arn(Option<String>)`](crate::operation::detach_user_policy::builders::DetachUserPolicyFluentBuilder::set_policy_arn): <p>The Amazon Resource Name (ARN) of the IAM policy you want to detach.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    /// - On success, responds with [`DetachUserPolicyOutput`](crate::operation::detach_user_policy::DetachUserPolicyOutput)
    /// - On failure, responds with [`SdkError<DetachUserPolicyError>`](crate::operation::detach_user_policy::DetachUserPolicyError)
    pub fn detach_user_policy(
        &self,
    ) -> crate::operation::detach_user_policy::builders::DetachUserPolicyFluentBuilder {
        crate::operation::detach_user_policy::builders::DetachUserPolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
