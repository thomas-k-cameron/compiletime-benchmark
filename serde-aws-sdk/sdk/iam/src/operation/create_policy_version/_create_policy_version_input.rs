// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreatePolicyVersionInput {
    /// <p>The Amazon Resource Name (ARN) of the IAM policy to which you want to add a new version.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[doc(hidden)]
    pub policy_arn: std::option::Option<std::string::String>,
    /// <p>The JSON policy document that you want to use as the content for this new version of the policy.</p>
    /// <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// <p>The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub policy_document: std::option::Option<std::string::String>,
    /// <p>Specifies whether to set this version as the policy's default version.</p>
    /// <p>When this parameter is <code>true</code>, the new policy version becomes the operative version. That is, it becomes the version that is in effect for the IAM users, groups, and roles that the policy is attached to.</p>
    /// <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub set_as_default: std::option::Option<bool>,
}
impl CreatePolicyVersionInput {
    /// <p>The Amazon Resource Name (ARN) of the IAM policy to which you want to add a new version.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn policy_arn(&self) -> std::option::Option<&str> {
        self.policy_arn.as_deref()
    }
    /// <p>The JSON policy document that you want to use as the content for this new version of the policy.</p>
    /// <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// <p>The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    pub fn policy_document(&self) -> std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>Specifies whether to set this version as the policy's default version.</p>
    /// <p>When this parameter is <code>true</code>, the new policy version becomes the operative version. That is, it becomes the version that is in effect for the IAM users, groups, and roles that the policy is attached to.</p>
    /// <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_as_default(&self) -> std::option::Option<bool> {
        self.set_as_default
    }
}
impl CreatePolicyVersionInput {
    /// Creates a new builder-style object to manufacture [`CreatePolicyVersionInput`](crate::operation::create_policy_version::CreatePolicyVersionInput).
    pub fn builder(
    ) -> crate::operation::create_policy_version::builders::CreatePolicyVersionInputBuilder {
        crate::operation::create_policy_version::builders::CreatePolicyVersionInputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_policy_version::CreatePolicyVersionInput;
/// A builder for [`CreatePolicyVersionInput`](crate::operation::create_policy_version::CreatePolicyVersionInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct CreatePolicyVersionInputBuilder {
    pub(crate) policy_arn: std::option::Option<std::string::String>,
    pub(crate) policy_document: std::option::Option<std::string::String>,
    pub(crate) set_as_default: std::option::Option<bool>,
}
impl CreatePolicyVersionInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the IAM policy to which you want to add a new version.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn policy_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM policy to which you want to add a new version.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn set_policy_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_arn = input;
        self
    }
    /// <p>The JSON policy document that you want to use as the content for this new version of the policy.</p>
    /// <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// <p>The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    pub fn policy_document(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_document = Some(input.into());
        self
    }
    /// <p>The JSON policy document that you want to use as the content for this new version of the policy.</p>
    /// <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// <p>The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    pub fn set_policy_document(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>Specifies whether to set this version as the policy's default version.</p>
    /// <p>When this parameter is <code>true</code>, the new policy version becomes the operative version. That is, it becomes the version that is in effect for the IAM users, groups, and roles that the policy is attached to.</p>
    /// <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_as_default(mut self, input: bool) -> Self {
        self.set_as_default = Some(input);
        self
    }
    /// <p>Specifies whether to set this version as the policy's default version.</p>
    /// <p>When this parameter is <code>true</code>, the new policy version becomes the operative version. That is, it becomes the version that is in effect for the IAM users, groups, and roles that the policy is attached to.</p>
    /// <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_set_as_default(mut self, input: std::option::Option<bool>) -> Self {
        self.set_as_default = input;
        self
    }
    /// Consumes the builder and constructs a [`CreatePolicyVersionInput`](crate::operation::create_policy_version::CreatePolicyVersionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_policy_version::CreatePolicyVersionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_policy_version::CreatePolicyVersionInput {
                policy_arn: self.policy_arn,
                policy_document: self.policy_document,
                set_as_default: self.set_as_default,
            },
        )
    }
}