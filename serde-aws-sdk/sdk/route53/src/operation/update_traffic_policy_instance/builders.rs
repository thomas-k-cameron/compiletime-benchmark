// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_traffic_policy_instance::_update_traffic_policy_instance_output::UpdateTrafficPolicyInstanceOutputBuilder;

pub use crate::operation::update_traffic_policy_instance::_update_traffic_policy_instance_input::UpdateTrafficPolicyInstanceInputBuilder;

/// Fluent builder constructing a request to `UpdateTrafficPolicyInstance`.
///
/// <p>Updates the resource record sets in a specified hosted zone that were created based on the settings in a specified traffic policy version.</p>
/// <p>When you update a traffic policy instance, Amazon Route 53 continues to respond to DNS queries for the root resource record set name (such as example.com) while it replaces one group of resource record sets with another. Route 53 performs the following operations:</p>
/// <ol>
/// <li> <p>Route 53 creates a new group of resource record sets based on the specified traffic policy. This is true regardless of how significant the differences are between the existing resource record sets and the new resource record sets. </p> </li>
/// <li> <p>When all of the new resource record sets have been created, Route 53 starts to respond to DNS queries for the root resource record set name (such as example.com) by using the new resource record sets.</p> </li>
/// <li> <p>Route 53 deletes the old group of resource record sets that are associated with the root resource record set name.</p> </li>
/// </ol>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateTrafficPolicyInstanceFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::update_traffic_policy_instance::builders::UpdateTrafficPolicyInstanceInputBuilder
            }
impl UpdateTrafficPolicyInstanceFluentBuilder {
    /// Creates a new `UpdateTrafficPolicyInstance`.
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
            crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstance,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError,
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
        crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_traffic_policy_instance::UpdateTrafficPolicyInstanceError,
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
    ///     let deserialized_parameters: crate::operation::update_traffic_policy_instance::builders::UpdateTrafficPolicyInstanceInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.update_traffic_policy_instance().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::update_traffic_policy_instance::builders::UpdateTrafficPolicyInstanceInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the traffic policy instance that you want to update.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the traffic policy instance that you want to update.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The TTL that you want Amazon Route 53 to assign to all of the updated resource record sets.</p>
    pub fn ttl(mut self, input: i64) -> Self {
        self.inner = self.inner.ttl(input);
        self
    }
    /// <p>The TTL that you want Amazon Route 53 to assign to all of the updated resource record sets.</p>
    pub fn set_ttl(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_ttl(input);
        self
    }
    /// <p>The ID of the traffic policy that you want Amazon Route 53 to use to update resource record sets for the specified traffic policy instance.</p>
    pub fn traffic_policy_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.traffic_policy_id(input.into());
        self
    }
    /// <p>The ID of the traffic policy that you want Amazon Route 53 to use to update resource record sets for the specified traffic policy instance.</p>
    pub fn set_traffic_policy_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_traffic_policy_id(input);
        self
    }
    /// <p>The version of the traffic policy that you want Amazon Route 53 to use to update resource record sets for the specified traffic policy instance.</p>
    pub fn traffic_policy_version(mut self, input: i32) -> Self {
        self.inner = self.inner.traffic_policy_version(input);
        self
    }
    /// <p>The version of the traffic policy that you want Amazon Route 53 to use to update resource record sets for the specified traffic policy instance.</p>
    pub fn set_traffic_policy_version(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_traffic_policy_version(input);
        self
    }
}
