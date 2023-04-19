// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_policy_version::_get_policy_version_output::GetPolicyVersionOutputBuilder;

pub use crate::operation::get_policy_version::_get_policy_version_input::GetPolicyVersionInputBuilder;

/// Fluent builder constructing a request to `GetPolicyVersion`.
///
/// <p>Retrieves information about the specified version of the specified managed policy, including the policy document.</p> <note>
/// <p>Policies returned by this operation are URL-encoded compliant with <a href="https://tools.ietf.org/html/rfc3986">RFC 3986</a>. You can use a URL decoding method to convert the policy back to plain JSON text. For example, if you use Java, you can use the <code>decode</code> method of the <code>java.net.URLDecoder</code> utility class in the Java SDK. Other languages and SDKs provide similar functionality.</p>
/// </note>
/// <p>To list the available versions for a policy, use <code>ListPolicyVersions</code>.</p>
/// <p>This operation retrieves information about managed policies. To retrieve information about an inline policy that is embedded in a user, group, or role, use <code>GetUserPolicy</code>, <code>GetGroupPolicy</code>, or <code>GetRolePolicy</code>.</p>
/// <p>For more information about the types of policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-inline.html">Managed policies and inline policies</a> in the <i>IAM User Guide</i>.</p>
/// <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetPolicyVersionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_policy_version::builders::GetPolicyVersionInputBuilder,
}
impl GetPolicyVersionFluentBuilder {
    /// Creates a new `GetPolicyVersion`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_policy_version::GetPolicyVersion,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_policy_version::GetPolicyVersionError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::get_policy_version::GetPolicyVersionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_policy_version::GetPolicyVersionError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::get_policy_version::builders::GetPolicyVersionInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_policy_version().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_policy_version::builders::GetPolicyVersionInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the managed policy that you want information about.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn policy_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.policy_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the managed policy that you want information about.</p>
    /// <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn set_policy_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_policy_arn(input);
        self
    }
    /// <p>Identifies the policy version to retrieve.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that consists of the lowercase letter 'v' followed by one or two digits, and optionally followed by a period '.' and a string of letters and digits.</p>
    pub fn version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.version_id(input.into());
        self
    }
    /// <p>Identifies the policy version to retrieve.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that consists of the lowercase letter 'v' followed by one or two digits, and optionally followed by a period '.' and a string of letters and digits.</p>
    pub fn set_version_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_version_id(input);
        self
    }
}