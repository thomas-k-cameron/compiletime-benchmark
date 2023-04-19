// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_data_retrieval_policy::_get_data_retrieval_policy_output::GetDataRetrievalPolicyOutputBuilder;

pub use crate::operation::get_data_retrieval_policy::_get_data_retrieval_policy_input::GetDataRetrievalPolicyInputBuilder;

/// Fluent builder constructing a request to `GetDataRetrievalPolicy`.
///
/// <p>This operation returns the current data retrieval policy for the account and region specified in the GET request. For more information about data retrieval policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetDataRetrievalPolicyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::get_data_retrieval_policy::builders::GetDataRetrievalPolicyInputBuilder,
}
impl GetDataRetrievalPolicyFluentBuilder {
    /// Creates a new `GetDataRetrievalPolicy`.
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
            crate::operation::get_data_retrieval_policy::GetDataRetrievalPolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_data_retrieval_policy::GetDataRetrievalPolicyError,
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
        crate::operation::get_data_retrieval_policy::GetDataRetrievalPolicyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_data_retrieval_policy::GetDataRetrievalPolicyError,
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
    ///     let deserialized_parameters: crate::operation::get_data_retrieval_policy::builders::GetDataRetrievalPolicyInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_data_retrieval_policy().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_data_retrieval_policy::builders::GetDataRetrievalPolicyInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID. </p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID. </p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
}
