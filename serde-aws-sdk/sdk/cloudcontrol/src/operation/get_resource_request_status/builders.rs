// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_resource_request_status::_get_resource_request_status_output::GetResourceRequestStatusOutputBuilder;

pub use crate::operation::get_resource_request_status::_get_resource_request_status_input::GetResourceRequestStatusInputBuilder;

/// Fluent builder constructing a request to `GetResourceRequestStatus`.
///
/// <p>Returns the current status of a resource operation request. For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html#resource-operations-manage-requests-track">Tracking the progress of resource operation requests</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceRequestStatusFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_resource_request_status::builders::GetResourceRequestStatusInputBuilder
            }
impl GetResourceRequestStatusFluentBuilder {
    /// Creates a new `GetResourceRequestStatus`.
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
            crate::operation::get_resource_request_status::GetResourceRequestStatus,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_resource_request_status::GetResourceRequestStatusError,
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
        crate::operation::get_resource_request_status::GetResourceRequestStatusOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_resource_request_status::GetResourceRequestStatusError,
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
    ///     let deserialized_parameters: crate::operation::get_resource_request_status::builders::GetResourceRequestStatusInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_resource_request_status().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_resource_request_status::builders::GetResourceRequestStatusInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>A unique token used to track the progress of the resource operation request.</p>
    /// <p>Request tokens are included in the <code>ProgressEvent</code> type returned by a resource operation request.</p>
    pub fn request_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.request_token(input.into());
        self
    }
    /// <p>A unique token used to track the progress of the resource operation request.</p>
    /// <p>Request tokens are included in the <code>ProgressEvent</code> type returned by a resource operation request.</p>
    pub fn set_request_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_request_token(input);
        self
    }
}
