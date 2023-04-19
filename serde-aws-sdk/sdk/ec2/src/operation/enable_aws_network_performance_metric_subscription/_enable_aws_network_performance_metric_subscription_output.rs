// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct EnableAwsNetworkPerformanceMetricSubscriptionOutput {
    /// <p>Indicates whether the subscribe action was successful.</p>
    #[doc(hidden)]
    pub output: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl EnableAwsNetworkPerformanceMetricSubscriptionOutput {
    /// <p>Indicates whether the subscribe action was successful.</p>
    pub fn output(&self) -> std::option::Option<bool> {
        self.output
    }
}
impl aws_http::request_id::RequestId for EnableAwsNetworkPerformanceMetricSubscriptionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl EnableAwsNetworkPerformanceMetricSubscriptionOutput {
    /// Creates a new builder-style object to manufacture [`EnableAwsNetworkPerformanceMetricSubscriptionOutput`](crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionOutput).
    pub fn builder() -> crate::operation::enable_aws_network_performance_metric_subscription::builders::EnableAwsNetworkPerformanceMetricSubscriptionOutputBuilder{
        crate::operation::enable_aws_network_performance_metric_subscription::builders::EnableAwsNetworkPerformanceMetricSubscriptionOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionOutput;
/// A builder for [`EnableAwsNetworkPerformanceMetricSubscriptionOutput`](crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct EnableAwsNetworkPerformanceMetricSubscriptionOutputBuilder {
    pub(crate) output: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl EnableAwsNetworkPerformanceMetricSubscriptionOutputBuilder {
    /// <p>Indicates whether the subscribe action was successful.</p>
    pub fn output(mut self, input: bool) -> Self {
        self.output = Some(input);
        self
    }
    /// <p>Indicates whether the subscribe action was successful.</p>
    pub fn set_output(mut self, input: std::option::Option<bool>) -> Self {
        self.output = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`EnableAwsNetworkPerformanceMetricSubscriptionOutput`](crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionOutput).
    pub fn build(self) -> crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionOutput{
        crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionOutput {
            output: self.output
            ,
            _request_id: self._request_id,
        }
    }
}