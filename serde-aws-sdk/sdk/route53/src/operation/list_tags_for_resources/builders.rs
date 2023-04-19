// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_tags_for_resources::_list_tags_for_resources_output::ListTagsForResourcesOutputBuilder;

pub use crate::operation::list_tags_for_resources::_list_tags_for_resources_input::ListTagsForResourcesInputBuilder;

/// Fluent builder constructing a request to `ListTagsForResources`.
///
/// <p>Lists tags for up to 10 health checks or hosted zones.</p>
/// <p>For information about using tags for cost allocation, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>Billing and Cost Management User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResourcesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_tags_for_resources::builders::ListTagsForResourcesInputBuilder,
}
impl ListTagsForResourcesFluentBuilder {
    /// Creates a new `ListTagsForResources`.
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
            crate::operation::list_tags_for_resources::ListTagsForResources,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_resources::ListTagsForResourcesError,
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
        crate::operation::list_tags_for_resources::ListTagsForResourcesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_resources::ListTagsForResourcesError,
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
    ///     let deserialized_parameters: crate::operation::list_tags_for_resources::builders::ListTagsForResourcesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.list_tags_for_resources().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::list_tags_for_resources::builders::ListTagsForResourcesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The type of the resources.</p>
    /// <ul>
    /// <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li>
    /// <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li>
    /// </ul>
    pub fn resource_type(mut self, input: crate::types::TagResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>The type of the resources.</p>
    /// <ul>
    /// <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li>
    /// <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li>
    /// </ul>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::TagResourceType>,
    ) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// Appends an item to `ResourceIds`.
    ///
    /// To override the contents of this collection use [`set_resource_ids`](Self::set_resource_ids).
    ///
    /// <p>A complex type that contains the ResourceId element for each resource for which you want to get a list of tags.</p>
    pub fn resource_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_ids(input.into());
        self
    }
    /// <p>A complex type that contains the ResourceId element for each resource for which you want to get a list of tags.</p>
    pub fn set_resource_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_resource_ids(input);
        self
    }
}