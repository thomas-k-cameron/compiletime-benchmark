// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_traffic_policy::_get_traffic_policy_output::GetTrafficPolicyOutputBuilder;

pub use crate::operation::get_traffic_policy::_get_traffic_policy_input::GetTrafficPolicyInputBuilder;

/// Fluent builder constructing a request to `GetTrafficPolicy`.
///
/// <p>Gets information about a specific traffic policy version.</p>
/// <p>For information about how of deleting a traffic policy affects the response from <code>GetTrafficPolicy</code>, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_DeleteTrafficPolicy.html">DeleteTrafficPolicy</a>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetTrafficPolicyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_traffic_policy::builders::GetTrafficPolicyInputBuilder,
}
impl GetTrafficPolicyFluentBuilder {
    /// Creates a new `GetTrafficPolicy`.
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
            crate::operation::get_traffic_policy::GetTrafficPolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_traffic_policy::GetTrafficPolicyError,
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
        crate::operation::get_traffic_policy::GetTrafficPolicyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_traffic_policy::GetTrafficPolicyError,
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
    ///     let deserialized_parameters: crate::operation::get_traffic_policy::builders::GetTrafficPolicyInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_traffic_policy().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_traffic_policy::builders::GetTrafficPolicyInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the traffic policy that you want to get information about.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the traffic policy that you want to get information about.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The version number of the traffic policy that you want to get information about.</p>
    pub fn version(mut self, input: i32) -> Self {
        self.inner = self.inner.version(input);
        self
    }
    /// <p>The version number of the traffic policy that you want to get information about.</p>
    pub fn set_version(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_version(input);
        self
    }
}