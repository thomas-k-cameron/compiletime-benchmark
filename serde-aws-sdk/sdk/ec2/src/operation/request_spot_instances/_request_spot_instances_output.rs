// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of RequestSpotInstances.</p>
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
pub struct RequestSpotInstancesOutput {
    /// <p>One or more Spot Instance requests.</p>
    #[doc(hidden)]
    pub spot_instance_requests:
        std::option::Option<std::vec::Vec<crate::types::SpotInstanceRequest>>,
    _request_id: Option<String>,
}
impl RequestSpotInstancesOutput {
    /// <p>One or more Spot Instance requests.</p>
    pub fn spot_instance_requests(
        &self,
    ) -> std::option::Option<&[crate::types::SpotInstanceRequest]> {
        self.spot_instance_requests.as_deref()
    }
}
impl aws_http::request_id::RequestId for RequestSpotInstancesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RequestSpotInstancesOutput {
    /// Creates a new builder-style object to manufacture [`RequestSpotInstancesOutput`](crate::operation::request_spot_instances::RequestSpotInstancesOutput).
    pub fn builder(
    ) -> crate::operation::request_spot_instances::builders::RequestSpotInstancesOutputBuilder {
        crate::operation::request_spot_instances::builders::RequestSpotInstancesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::request_spot_instances::RequestSpotInstancesOutput;
/// A builder for [`RequestSpotInstancesOutput`](crate::operation::request_spot_instances::RequestSpotInstancesOutput).
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
pub struct RequestSpotInstancesOutputBuilder {
    pub(crate) spot_instance_requests:
        std::option::Option<std::vec::Vec<crate::types::SpotInstanceRequest>>,
    _request_id: Option<String>,
}
impl RequestSpotInstancesOutputBuilder {
    /// Appends an item to `spot_instance_requests`.
    ///
    /// To override the contents of this collection use [`set_spot_instance_requests`](Self::set_spot_instance_requests).
    ///
    /// <p>One or more Spot Instance requests.</p>
    pub fn spot_instance_requests(mut self, input: crate::types::SpotInstanceRequest) -> Self {
        let mut v = self.spot_instance_requests.unwrap_or_default();
        v.push(input);
        self.spot_instance_requests = Some(v);
        self
    }
    /// <p>One or more Spot Instance requests.</p>
    pub fn set_spot_instance_requests(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::SpotInstanceRequest>>,
    ) -> Self {
        self.spot_instance_requests = input;
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
    /// Consumes the builder and constructs a [`RequestSpotInstancesOutput`](crate::operation::request_spot_instances::RequestSpotInstancesOutput).
    pub fn build(self) -> crate::operation::request_spot_instances::RequestSpotInstancesOutput {
        crate::operation::request_spot_instances::RequestSpotInstancesOutput {
            spot_instance_requests: self.spot_instance_requests,
            _request_id: self._request_id,
        }
    }
}
