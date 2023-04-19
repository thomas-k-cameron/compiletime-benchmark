// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_traffic_policy_instances::_list_traffic_policy_instances_output::ListTrafficPolicyInstancesOutputBuilder;

pub use crate::operation::list_traffic_policy_instances::_list_traffic_policy_instances_input::ListTrafficPolicyInstancesInputBuilder;

/// Fluent builder constructing a request to `ListTrafficPolicyInstances`.
///
/// <p>Gets information about the traffic policy instances that you created by using the current Amazon Web Services account.</p> <note>
/// <p>After you submit an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p>
/// </note>
/// <p>Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListTrafficPolicyInstancesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::list_traffic_policy_instances::builders::ListTrafficPolicyInstancesInputBuilder
            }
impl ListTrafficPolicyInstancesFluentBuilder {
    /// Creates a new `ListTrafficPolicyInstances`.
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
            crate::operation::list_traffic_policy_instances::ListTrafficPolicyInstances,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_traffic_policy_instances::ListTrafficPolicyInstancesError,
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
        crate::operation::list_traffic_policy_instances::ListTrafficPolicyInstancesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_traffic_policy_instances::ListTrafficPolicyInstancesError,
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
    ///     let deserialized_parameters: crate::operation::list_traffic_policy_instances::builders::ListTrafficPolicyInstancesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.list_traffic_policy_instances().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::list_traffic_policy_instances::builders::ListTrafficPolicyInstancesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>HostedZoneId</code>, specify the value of <code>HostedZoneIdMarker</code> from the previous response, which is the hosted zone ID of the first traffic policy instance in the next group of traffic policy instances.</p>
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub fn hosted_zone_id_marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.hosted_zone_id_marker(input.into());
        self
    }
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>HostedZoneId</code>, specify the value of <code>HostedZoneIdMarker</code> from the previous response, which is the hosted zone ID of the first traffic policy instance in the next group of traffic policy instances.</p>
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub fn set_hosted_zone_id_marker(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_hosted_zone_id_marker(input);
        self
    }
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancename</code>, specify the value of <code>TrafficPolicyInstanceNameMarker</code> from the previous response, which is the name of the first traffic policy instance in the next group of traffic policy instances.</p>
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub fn traffic_policy_instance_name_marker(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.traffic_policy_instance_name_marker(input.into());
        self
    }
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancename</code>, specify the value of <code>TrafficPolicyInstanceNameMarker</code> from the previous response, which is the name of the first traffic policy instance in the next group of traffic policy instances.</p>
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub fn set_traffic_policy_instance_name_marker(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_traffic_policy_instance_name_marker(input);
        self
    }
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancetype</code>, specify the value of <code>TrafficPolicyInstanceTypeMarker</code> from the previous response, which is the type of the first traffic policy instance in the next group of traffic policy instances.</p>
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub fn traffic_policy_instance_type_marker(mut self, input: crate::types::RrType) -> Self {
        self.inner = self.inner.traffic_policy_instance_type_marker(input);
        self
    }
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancetype</code>, specify the value of <code>TrafficPolicyInstanceTypeMarker</code> from the previous response, which is the type of the first traffic policy instance in the next group of traffic policy instances.</p>
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub fn set_traffic_policy_instance_type_marker(
        mut self,
        input: std::option::Option<crate::types::RrType>,
    ) -> Self {
        self.inner = self.inner.set_traffic_policy_instance_type_marker(input);
        self
    }
    /// <p>The maximum number of traffic policy instances that you want Amazon Route 53 to return in response to a <code>ListTrafficPolicyInstances</code> request. If you have more than <code>MaxItems</code> traffic policy instances, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> represent the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of traffic policy instances that you want Amazon Route 53 to return in response to a <code>ListTrafficPolicyInstances</code> request. If you have more than <code>MaxItems</code> traffic policy instances, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> represent the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
}
