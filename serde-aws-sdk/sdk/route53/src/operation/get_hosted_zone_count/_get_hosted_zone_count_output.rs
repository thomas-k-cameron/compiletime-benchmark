// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the response to a <code>GetHostedZoneCount</code> request.</p>
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
pub struct GetHostedZoneCountOutput {
    /// <p>The total number of public and private hosted zones that are associated with the current Amazon Web Services account.</p>
    #[doc(hidden)]
    pub hosted_zone_count: std::option::Option<i64>,
    _request_id: Option<String>,
}
impl GetHostedZoneCountOutput {
    /// <p>The total number of public and private hosted zones that are associated with the current Amazon Web Services account.</p>
    pub fn hosted_zone_count(&self) -> std::option::Option<i64> {
        self.hosted_zone_count
    }
}
impl aws_http::request_id::RequestId for GetHostedZoneCountOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetHostedZoneCountOutput {
    /// Creates a new builder-style object to manufacture [`GetHostedZoneCountOutput`](crate::operation::get_hosted_zone_count::GetHostedZoneCountOutput).
    pub fn builder(
    ) -> crate::operation::get_hosted_zone_count::builders::GetHostedZoneCountOutputBuilder {
        crate::operation::get_hosted_zone_count::builders::GetHostedZoneCountOutputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_hosted_zone_count::GetHostedZoneCountOutput;
/// A builder for [`GetHostedZoneCountOutput`](crate::operation::get_hosted_zone_count::GetHostedZoneCountOutput).
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
pub struct GetHostedZoneCountOutputBuilder {
    pub(crate) hosted_zone_count: std::option::Option<i64>,
    _request_id: Option<String>,
}
impl GetHostedZoneCountOutputBuilder {
    /// <p>The total number of public and private hosted zones that are associated with the current Amazon Web Services account.</p>
    pub fn hosted_zone_count(mut self, input: i64) -> Self {
        self.hosted_zone_count = Some(input);
        self
    }
    /// <p>The total number of public and private hosted zones that are associated with the current Amazon Web Services account.</p>
    pub fn set_hosted_zone_count(mut self, input: std::option::Option<i64>) -> Self {
        self.hosted_zone_count = input;
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
    /// Consumes the builder and constructs a [`GetHostedZoneCountOutput`](crate::operation::get_hosted_zone_count::GetHostedZoneCountOutput).
    pub fn build(self) -> crate::operation::get_hosted_zone_count::GetHostedZoneCountOutput {
        crate::operation::get_hosted_zone_count::GetHostedZoneCountOutput {
            hosted_zone_count: self.hosted_zone_count,
            _request_id: self._request_id,
        }
    }
}
