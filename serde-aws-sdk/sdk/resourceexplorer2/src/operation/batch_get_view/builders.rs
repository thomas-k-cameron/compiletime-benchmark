// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_get_view::_batch_get_view_output::BatchGetViewOutputBuilder;

pub use crate::operation::batch_get_view::_batch_get_view_input::BatchGetViewInputBuilder;

/// Fluent builder constructing a request to `BatchGetView`.
///
/// <p>Retrieves details about a list of views.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchGetViewFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_get_view::builders::BatchGetViewInputBuilder,
}
impl BatchGetViewFluentBuilder {
    /// Creates a new `BatchGetView`.
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
            crate::operation::batch_get_view::BatchGetView,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::batch_get_view::BatchGetViewError>,
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
        crate::operation::batch_get_view::BatchGetViewOutput,
        aws_smithy_http::result::SdkError<crate::operation::batch_get_view::BatchGetViewError>,
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
    ///     let deserialized_parameters: crate::operation::batch_get_view::builders::BatchGetViewInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.batch_get_view().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::batch_get_view::builders::BatchGetViewInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Appends an item to `ViewArns`.
    ///
    /// To override the contents of this collection use [`set_view_arns`](Self::set_view_arns).
    ///
    /// <p>A list of <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource names (ARNs)</a> that identify the views you want details for.</p>
    pub fn view_arns(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.view_arns(input.into());
        self
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource names (ARNs)</a> that identify the views you want details for.</p>
    pub fn set_view_arns(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_view_arns(input);
        self
    }
}
