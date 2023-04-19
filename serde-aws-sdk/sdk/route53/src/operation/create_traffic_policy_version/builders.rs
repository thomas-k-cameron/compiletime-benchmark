// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_traffic_policy_version::_create_traffic_policy_version_output::CreateTrafficPolicyVersionOutputBuilder;

pub use crate::operation::create_traffic_policy_version::_create_traffic_policy_version_input::CreateTrafficPolicyVersionInputBuilder;

/// Fluent builder constructing a request to `CreateTrafficPolicyVersion`.
///
/// <p>Creates a new version of an existing traffic policy. When you create a new version of a traffic policy, you specify the ID of the traffic policy that you want to update and a JSON-formatted document that describes the new version. You use traffic policies to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com). You can create a maximum of 1000 versions of a traffic policy. If you reach the limit and need to create another version, you'll need to start a new traffic policy.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateTrafficPolicyVersionFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_traffic_policy_version::builders::CreateTrafficPolicyVersionInputBuilder
            }
impl CreateTrafficPolicyVersionFluentBuilder {
    /// Creates a new `CreateTrafficPolicyVersion`.
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
            crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersion,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError,
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
        crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError,
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
    ///     let deserialized_parameters: crate::operation::create_traffic_policy_version::builders::CreateTrafficPolicyVersionInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_traffic_policy_version().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_traffic_policy_version::builders::CreateTrafficPolicyVersionInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the traffic policy for which you want to create a new version.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the traffic policy for which you want to create a new version.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The definition of this version of the traffic policy, in JSON format. You specified the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateTrafficPolicy.html">CreateTrafficPolicy</a>.</p>
    pub fn document(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.document(input.into());
        self
    }
    /// <p>The definition of this version of the traffic policy, in JSON format. You specified the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateTrafficPolicy.html">CreateTrafficPolicy</a>.</p>
    pub fn set_document(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_document(input);
        self
    }
    /// <p>The comment that you specified in the <code>CreateTrafficPolicyVersion</code> request, if any.</p>
    pub fn comment(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.comment(input.into());
        self
    }
    /// <p>The comment that you specified in the <code>CreateTrafficPolicyVersion</code> request, if any.</p>
    pub fn set_comment(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_comment(input);
        self
    }
}