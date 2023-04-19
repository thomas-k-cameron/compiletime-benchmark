// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_access_keys::_list_access_keys_output::ListAccessKeysOutputBuilder;

pub use crate::operation::list_access_keys::_list_access_keys_input::ListAccessKeysInputBuilder;

/// Fluent builder constructing a request to `ListAccessKeys`.
///
/// <p>Returns information about the access key IDs associated with the specified IAM user. If there is none, the operation returns an empty list.</p>
/// <p>Although each user is limited to a small number of keys, you can still paginate the results using the <code>MaxItems</code> and <code>Marker</code> parameters.</p>
/// <p>If the <code>UserName</code> is not specified, the user name is determined implicitly based on the Amazon Web Services access key ID used to sign the request. If a temporary access key is used, then <code>UserName</code> is required. If a long-term key is assigned to the user, then <code>UserName</code> is not required. This operation works for access keys under the Amazon Web Services account. Consequently, you can use this operation to manage Amazon Web Services account root user credentials even if the Amazon Web Services account has no associated users.</p> <note>
/// <p>To ensure the security of your Amazon Web Services account, the secret access key is accessible only during key and user creation.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListAccessKeysFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_access_keys::builders::ListAccessKeysInputBuilder,
}
impl ListAccessKeysFluentBuilder {
    /// Creates a new `ListAccessKeys`.
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
            crate::operation::list_access_keys::ListAccessKeys,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_access_keys::ListAccessKeysError>,
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
        crate::operation::list_access_keys::ListAccessKeysOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_access_keys::ListAccessKeysError>,
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
    ///     let deserialized_parameters: crate::operation::list_access_keys::builders::ListAccessKeysInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.list_access_keys().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::list_access_keys::builders::ListAccessKeysInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_access_keys::paginator::ListAccessKeysPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_access_keys::paginator::ListAccessKeysPaginator {
        crate::operation::list_access_keys::paginator::ListAccessKeysPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The name of the user.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn user_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.user_name(input.into());
        self
    }
    /// <p>The name of the user.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_user_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_user_name(input);
        self
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
}
