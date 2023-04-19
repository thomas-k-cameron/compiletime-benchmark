// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetDefaultPolicyVersion`](crate::operation::set_default_policy_version::builders::SetDefaultPolicyVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_arn(impl Into<String>)`](crate::operation::set_default_policy_version::builders::SetDefaultPolicyVersionFluentBuilder::policy_arn) / [`set_policy_arn(Option<String>)`](crate::operation::set_default_policy_version::builders::SetDefaultPolicyVersionFluentBuilder::set_policy_arn): <p>The Amazon Resource Name (ARN) of the IAM policy whose default version you want to set.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`version_id(impl Into<String>)`](crate::operation::set_default_policy_version::builders::SetDefaultPolicyVersionFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::set_default_policy_version::builders::SetDefaultPolicyVersionFluentBuilder::set_version_id): <p>The version of the policy to set as the default (operative) version.</p>  <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    /// - On success, responds with [`SetDefaultPolicyVersionOutput`](crate::operation::set_default_policy_version::SetDefaultPolicyVersionOutput)
    /// - On failure, responds with [`SdkError<SetDefaultPolicyVersionError>`](crate::operation::set_default_policy_version::SetDefaultPolicyVersionError)
    pub fn set_default_policy_version(
        &self,
    ) -> crate::operation::set_default_policy_version::builders::SetDefaultPolicyVersionFluentBuilder
    {
        crate::operation::set_default_policy_version::builders::SetDefaultPolicyVersionFluentBuilder::new(self.handle.clone())
    }
}
