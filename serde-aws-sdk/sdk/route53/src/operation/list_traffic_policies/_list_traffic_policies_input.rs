// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the information about the request to list the traffic policies that are associated with the current Amazon Web Services account.</p>
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
pub struct ListTrafficPoliciesInput {
    /// <p>(Conditional) For your first request to <code>ListTrafficPolicies</code>, don't include the <code>TrafficPolicyIdMarker</code> parameter.</p>
    /// <p>If you have more traffic policies than the value of <code>MaxItems</code>, <code>ListTrafficPolicies</code> returns only the first <code>MaxItems</code> traffic policies. To get the next group of policies, submit another request to <code>ListTrafficPolicies</code>. For the value of <code>TrafficPolicyIdMarker</code>, specify the value of <code>TrafficPolicyIdMarker</code> that was returned in the previous response.</p>
    #[doc(hidden)]
    pub traffic_policy_id_marker: std::option::Option<std::string::String>,
    /// <p>(Optional) The maximum number of traffic policies that you want Amazon Route 53 to return in response to this request. If you have more than <code>MaxItems</code> traffic policies, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of <code>TrafficPolicyIdMarker</code> is the ID of the first traffic policy that Route 53 will return if you submit another request.</p>
    #[doc(hidden)]
    pub max_items: std::option::Option<i32>,
}
impl ListTrafficPoliciesInput {
    /// <p>(Conditional) For your first request to <code>ListTrafficPolicies</code>, don't include the <code>TrafficPolicyIdMarker</code> parameter.</p>
    /// <p>If you have more traffic policies than the value of <code>MaxItems</code>, <code>ListTrafficPolicies</code> returns only the first <code>MaxItems</code> traffic policies. To get the next group of policies, submit another request to <code>ListTrafficPolicies</code>. For the value of <code>TrafficPolicyIdMarker</code>, specify the value of <code>TrafficPolicyIdMarker</code> that was returned in the previous response.</p>
    pub fn traffic_policy_id_marker(&self) -> std::option::Option<&str> {
        self.traffic_policy_id_marker.as_deref()
    }
    /// <p>(Optional) The maximum number of traffic policies that you want Amazon Route 53 to return in response to this request. If you have more than <code>MaxItems</code> traffic policies, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of <code>TrafficPolicyIdMarker</code> is the ID of the first traffic policy that Route 53 will return if you submit another request.</p>
    pub fn max_items(&self) -> std::option::Option<i32> {
        self.max_items
    }
}
impl ListTrafficPoliciesInput {
    /// Creates a new builder-style object to manufacture [`ListTrafficPoliciesInput`](crate::operation::list_traffic_policies::ListTrafficPoliciesInput).
    pub fn builder(
    ) -> crate::operation::list_traffic_policies::builders::ListTrafficPoliciesInputBuilder {
        crate::operation::list_traffic_policies::builders::ListTrafficPoliciesInputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_traffic_policies::ListTrafficPoliciesInput;
/// A builder for [`ListTrafficPoliciesInput`](crate::operation::list_traffic_policies::ListTrafficPoliciesInput).
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
pub struct ListTrafficPoliciesInputBuilder {
    pub(crate) traffic_policy_id_marker: std::option::Option<std::string::String>,
    pub(crate) max_items: std::option::Option<i32>,
}
impl ListTrafficPoliciesInputBuilder {
    /// <p>(Conditional) For your first request to <code>ListTrafficPolicies</code>, don't include the <code>TrafficPolicyIdMarker</code> parameter.</p>
    /// <p>If you have more traffic policies than the value of <code>MaxItems</code>, <code>ListTrafficPolicies</code> returns only the first <code>MaxItems</code> traffic policies. To get the next group of policies, submit another request to <code>ListTrafficPolicies</code>. For the value of <code>TrafficPolicyIdMarker</code>, specify the value of <code>TrafficPolicyIdMarker</code> that was returned in the previous response.</p>
    pub fn traffic_policy_id_marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.traffic_policy_id_marker = Some(input.into());
        self
    }
    /// <p>(Conditional) For your first request to <code>ListTrafficPolicies</code>, don't include the <code>TrafficPolicyIdMarker</code> parameter.</p>
    /// <p>If you have more traffic policies than the value of <code>MaxItems</code>, <code>ListTrafficPolicies</code> returns only the first <code>MaxItems</code> traffic policies. To get the next group of policies, submit another request to <code>ListTrafficPolicies</code>. For the value of <code>TrafficPolicyIdMarker</code>, specify the value of <code>TrafficPolicyIdMarker</code> that was returned in the previous response.</p>
    pub fn set_traffic_policy_id_marker(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.traffic_policy_id_marker = input;
        self
    }
    /// <p>(Optional) The maximum number of traffic policies that you want Amazon Route 53 to return in response to this request. If you have more than <code>MaxItems</code> traffic policies, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of <code>TrafficPolicyIdMarker</code> is the ID of the first traffic policy that Route 53 will return if you submit another request.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = Some(input);
        self
    }
    /// <p>(Optional) The maximum number of traffic policies that you want Amazon Route 53 to return in response to this request. If you have more than <code>MaxItems</code> traffic policies, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of <code>TrafficPolicyIdMarker</code> is the ID of the first traffic policy that Route 53 will return if you submit another request.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// Consumes the builder and constructs a [`ListTrafficPoliciesInput`](crate::operation::list_traffic_policies::ListTrafficPoliciesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_traffic_policies::ListTrafficPoliciesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::list_traffic_policies::ListTrafficPoliciesInput {
                traffic_policy_id_marker: self.traffic_policy_id_marker,
                max_items: self.max_items,
            },
        )
    }
}