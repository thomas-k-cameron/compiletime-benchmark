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
pub struct GetContextKeysForPrincipalPolicyInput {
    /// <p>The ARN of a user, group, or role whose policies contain the context keys that you want listed. If you specify a user, the list includes context keys that are found in all policies that are attached to the user. The list also includes all groups that the user is a member of. If you pick a group or a role, then it includes only those context keys that are found in policies attached to that entity. Note that all parameters are shown in unencoded form here for clarity, but must be URL encoded to be included as a part of a real HTML request.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[doc(hidden)]
    pub policy_source_arn: std::option::Option<std::string::String>,
    /// <p>An optional list of additional policies for which you want the list of context keys that are referenced.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub policy_input_list: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GetContextKeysForPrincipalPolicyInput {
    /// <p>The ARN of a user, group, or role whose policies contain the context keys that you want listed. If you specify a user, the list includes context keys that are found in all policies that are attached to the user. The list also includes all groups that the user is a member of. If you pick a group or a role, then it includes only those context keys that are found in policies attached to that entity. Note that all parameters are shown in unencoded form here for clarity, but must be URL encoded to be included as a part of a real HTML request.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn policy_source_arn(&self) -> std::option::Option<&str> {
        self.policy_source_arn.as_deref()
    }
    /// <p>An optional list of additional policies for which you want the list of context keys that are referenced.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    pub fn policy_input_list(&self) -> std::option::Option<&[std::string::String]> {
        self.policy_input_list.as_deref()
    }
}
impl GetContextKeysForPrincipalPolicyInput {
    /// Creates a new builder-style object to manufacture [`GetContextKeysForPrincipalPolicyInput`](crate::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyInput).
    pub fn builder() -> crate::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyInputBuilder{
        crate::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyInput;
/// A builder for [`GetContextKeysForPrincipalPolicyInput`](crate::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyInput).
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
pub struct GetContextKeysForPrincipalPolicyInputBuilder {
    pub(crate) policy_source_arn: std::option::Option<std::string::String>,
    pub(crate) policy_input_list: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GetContextKeysForPrincipalPolicyInputBuilder {
    /// <p>The ARN of a user, group, or role whose policies contain the context keys that you want listed. If you specify a user, the list includes context keys that are found in all policies that are attached to the user. The list also includes all groups that the user is a member of. If you pick a group or a role, then it includes only those context keys that are found in policies attached to that entity. Note that all parameters are shown in unencoded form here for clarity, but must be URL encoded to be included as a part of a real HTML request.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn policy_source_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_source_arn = Some(input.into());
        self
    }
    /// <p>The ARN of a user, group, or role whose policies contain the context keys that you want listed. If you specify a user, the list includes context keys that are found in all policies that are attached to the user. The list also includes all groups that the user is a member of. If you pick a group or a role, then it includes only those context keys that are found in policies attached to that entity. Note that all parameters are shown in unencoded form here for clarity, but must be URL encoded to be included as a part of a real HTML request.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn set_policy_source_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.policy_source_arn = input;
        self
    }
    /// Appends an item to `policy_input_list`.
    ///
    /// To override the contents of this collection use [`set_policy_input_list`](Self::set_policy_input_list).
    ///
    /// <p>An optional list of additional policies for which you want the list of context keys that are referenced.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    pub fn policy_input_list(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.policy_input_list.unwrap_or_default();
        v.push(input.into());
        self.policy_input_list = Some(v);
        self
    }
    /// <p>An optional list of additional policies for which you want the list of context keys that are referenced.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    pub fn set_policy_input_list(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.policy_input_list = input;
        self
    }
    /// Consumes the builder and constructs a [`GetContextKeysForPrincipalPolicyInput`](crate::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyInput).
    pub fn build(self) -> Result<crate::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyInput {
                policy_source_arn: self.policy_source_arn
                ,
                policy_input_list: self.policy_input_list
                ,
            }
        )
    }
}