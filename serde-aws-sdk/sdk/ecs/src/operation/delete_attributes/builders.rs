// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_attributes::_delete_attributes_output::DeleteAttributesOutputBuilder;

pub use crate::operation::delete_attributes::_delete_attributes_input::DeleteAttributesInputBuilder;

/// Fluent builder constructing a request to `DeleteAttributes`.
///
/// <p>Deletes one or more custom attributes from an Amazon ECS resource.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAttributesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_attributes::builders::DeleteAttributesInputBuilder,
}
impl DeleteAttributesFluentBuilder {
    /// Creates a new `DeleteAttributes`.
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
            crate::operation::delete_attributes::DeleteAttributes,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_attributes::DeleteAttributesError,
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
        crate::operation::delete_attributes::DeleteAttributesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_attributes::DeleteAttributesError,
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
    ///     let deserialized_parameters: crate::operation::delete_attributes::builders::DeleteAttributesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_attributes().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_attributes::builders::DeleteAttributesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the resource to delete attributes. If you do not specify a cluster, the default cluster is assumed.</p>
    pub fn cluster(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cluster(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the resource to delete attributes. If you do not specify a cluster, the default cluster is assumed.</p>
    pub fn set_cluster(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cluster(input);
        self
    }
    /// Appends an item to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>The attributes to delete from your resource. You can specify up to 10 attributes for each request. For custom attributes, specify the attribute name and target ID, but don't specify the value. If you specify the target ID using the short form, you must also specify the target type.</p>
    pub fn attributes(mut self, input: crate::types::Attribute) -> Self {
        self.inner = self.inner.attributes(input);
        self
    }
    /// <p>The attributes to delete from your resource. You can specify up to 10 attributes for each request. For custom attributes, specify the attribute name and target ID, but don't specify the value. If you specify the target ID using the short form, you must also specify the target type.</p>
    pub fn set_attributes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    ) -> Self {
        self.inner = self.inner.set_attributes(input);
        self
    }
}
