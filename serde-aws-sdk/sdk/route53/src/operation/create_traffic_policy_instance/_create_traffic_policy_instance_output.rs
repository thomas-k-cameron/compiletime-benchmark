// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the response information for the <code>CreateTrafficPolicyInstance</code> request.</p>
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
pub struct CreateTrafficPolicyInstanceOutput {
    /// <p>A complex type that contains settings for the new traffic policy instance.</p>
    #[doc(hidden)]
    pub traffic_policy_instance: std::option::Option<crate::types::TrafficPolicyInstance>,
    /// <p>A unique URL that represents a new traffic policy instance.</p>
    #[doc(hidden)]
    pub location: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CreateTrafficPolicyInstanceOutput {
    /// <p>A complex type that contains settings for the new traffic policy instance.</p>
    pub fn traffic_policy_instance(
        &self,
    ) -> std::option::Option<&crate::types::TrafficPolicyInstance> {
        self.traffic_policy_instance.as_ref()
    }
    /// <p>A unique URL that represents a new traffic policy instance.</p>
    pub fn location(&self) -> std::option::Option<&str> {
        self.location.as_deref()
    }
}
impl aws_http::request_id::RequestId for CreateTrafficPolicyInstanceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateTrafficPolicyInstanceOutput {
    /// Creates a new builder-style object to manufacture [`CreateTrafficPolicyInstanceOutput`](crate::operation::create_traffic_policy_instance::CreateTrafficPolicyInstanceOutput).
    pub fn builder() -> crate::operation::create_traffic_policy_instance::builders::CreateTrafficPolicyInstanceOutputBuilder{
        crate::operation::create_traffic_policy_instance::builders::CreateTrafficPolicyInstanceOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_traffic_policy_instance::CreateTrafficPolicyInstanceOutput;
/// A builder for [`CreateTrafficPolicyInstanceOutput`](crate::operation::create_traffic_policy_instance::CreateTrafficPolicyInstanceOutput).
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
pub struct CreateTrafficPolicyInstanceOutputBuilder {
    pub(crate) traffic_policy_instance: std::option::Option<crate::types::TrafficPolicyInstance>,
    pub(crate) location: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CreateTrafficPolicyInstanceOutputBuilder {
    /// <p>A complex type that contains settings for the new traffic policy instance.</p>
    pub fn traffic_policy_instance(mut self, input: crate::types::TrafficPolicyInstance) -> Self {
        self.traffic_policy_instance = Some(input);
        self
    }
    /// <p>A complex type that contains settings for the new traffic policy instance.</p>
    pub fn set_traffic_policy_instance(
        mut self,
        input: std::option::Option<crate::types::TrafficPolicyInstance>,
    ) -> Self {
        self.traffic_policy_instance = input;
        self
    }
    /// <p>A unique URL that represents a new traffic policy instance.</p>
    pub fn location(mut self, input: impl Into<std::string::String>) -> Self {
        self.location = Some(input.into());
        self
    }
    /// <p>A unique URL that represents a new traffic policy instance.</p>
    pub fn set_location(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.location = input;
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
    /// Consumes the builder and constructs a [`CreateTrafficPolicyInstanceOutput`](crate::operation::create_traffic_policy_instance::CreateTrafficPolicyInstanceOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_traffic_policy_instance::CreateTrafficPolicyInstanceOutput {
        crate::operation::create_traffic_policy_instance::CreateTrafficPolicyInstanceOutput {
            traffic_policy_instance: self.traffic_policy_instance,
            location: self.location,
            _request_id: self._request_id,
        }
    }
}