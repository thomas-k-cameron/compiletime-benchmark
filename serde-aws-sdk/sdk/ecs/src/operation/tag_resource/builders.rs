// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::tag_resource::_tag_resource_output::TagResourceOutputBuilder;

pub use crate::operation::tag_resource::_tag_resource_input::TagResourceInputBuilder;

/// Fluent builder constructing a request to `TagResource`.
///
/// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags that are associated with that resource are deleted as well.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct TagResourceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::tag_resource::builders::TagResourceInputBuilder,
}
impl TagResourceFluentBuilder {
    /// Creates a new `TagResource`.
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
            crate::operation::tag_resource::TagResource,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::tag_resource::TagResourceError>,
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
        crate::operation::tag_resource::TagResourceOutput,
        aws_smithy_http::result::SdkError<crate::operation::tag_resource::TagResourceError>,
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
    ///     let deserialized_parameters: crate::operation::tag_resource::builders::TagResourceInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.tag_resource().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::tag_resource::builders::TagResourceInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource to add tags to. Currently, the supported resources are Amazon ECS capacity providers, tasks, services, task definitions, clusters, and container instances.</p>
    pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource to add tags to. Currently, the supported resources are Amazon ECS capacity providers, tasks, services, task definitions, clusters, and container instances.</p>
    pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li> <p>Maximum number of tags per resource - 50</p> </li>
    /// <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>
    /// <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li>
    /// <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>
    /// <li> <p>Tag keys and values are case-sensitive.</p> </li>
    /// <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li>
    /// </ul>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li> <p>Maximum number of tags per resource - 50</p> </li>
    /// <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>
    /// <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li>
    /// <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li>
    /// <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>
    /// <li> <p>Tag keys and values are case-sensitive.</p> </li>
    /// <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li>
    /// </ul>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}