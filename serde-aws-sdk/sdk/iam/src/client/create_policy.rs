// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreatePolicy`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_name(impl Into<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::set_policy_name): <p>The friendly name of the policy.</p>  <p>IAM user, group, role, and policy names must be unique within the account. Names are not distinguished by case. For example, you cannot create resources named both "MyResource" and "myresource".</p>
    ///   - [`path(impl Into<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::path) / [`set_path(Option<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::set_path): <p>The path for the policy.</p>  <p>For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>  <p>This parameter is optional. If it is not included, it defaults to a slash (/).</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of either a forward slash (/) by itself or a string that must begin and end with forward slashes. In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including most punctuation characters, digits, and upper and lowercased letters.</p> <note>   <p>You cannot use an asterisk (*) in the path name.</p>  </note>
    ///   - [`policy_document(impl Into<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::policy_document) / [`set_policy_document(Option<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::set_policy_document): <p>The JSON policy document that you want to use as the content for the new policy.</p>  <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>  <p>The maximum length of the policy document that you can pass in this operation, including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p>  <p>To learn more about JSON policy grammar, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_grammar.html">Grammar of the IAM JSON policy language</a> in the <i>IAM User Guide</i>. </p>  <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>  <ul>   <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>   <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>   <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>  </ul>
    ///   - [`description(impl Into<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::set_description): <p>A friendly description of the policy.</p>  <p>Typically used to store information about the permissions defined in the policy. For example, "Grants access to production DynamoDB tables."</p>  <p>The policy description is immutable. After a value is assigned, it cannot be changed.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_policy::builders::CreatePolicyFluentBuilder::set_tags): <p>A list of tags that you want to attach to the new IAM customer managed policy. Each tag consists of a key name and an associated value. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p> <note>   <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request fails and the resource is not created.</p>  </note>
    /// - On success, responds with [`CreatePolicyOutput`](crate::operation::create_policy::CreatePolicyOutput) with field(s):
    ///   - [`policy(Option<Policy>)`](crate::operation::create_policy::CreatePolicyOutput::policy): <p>A structure containing details about the new policy.</p>
    /// - On failure, responds with [`SdkError<CreatePolicyError>`](crate::operation::create_policy::CreatePolicyError)
    pub fn create_policy(
        &self,
    ) -> crate::operation::create_policy::builders::CreatePolicyFluentBuilder {
        crate::operation::create_policy::builders::CreatePolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}