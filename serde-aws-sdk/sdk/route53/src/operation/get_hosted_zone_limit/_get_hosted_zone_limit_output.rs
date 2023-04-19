// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the requested limit. </p>
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
pub struct GetHostedZoneLimitOutput {
    /// <p>The current setting for the specified limit. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Limit</code> is the maximum number of records that you can create in the specified hosted zone.</p>
    #[doc(hidden)]
    pub limit: std::option::Option<crate::types::HostedZoneLimit>,
    /// <p>The current number of entities that you have created of the specified type. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Count</code> is the current number of records that you have created in the specified hosted zone.</p>
    #[doc(hidden)]
    pub count: i64,
    _request_id: Option<String>,
}
impl GetHostedZoneLimitOutput {
    /// <p>The current setting for the specified limit. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Limit</code> is the maximum number of records that you can create in the specified hosted zone.</p>
    pub fn limit(&self) -> std::option::Option<&crate::types::HostedZoneLimit> {
        self.limit.as_ref()
    }
    /// <p>The current number of entities that you have created of the specified type. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Count</code> is the current number of records that you have created in the specified hosted zone.</p>
    pub fn count(&self) -> i64 {
        self.count
    }
}
impl aws_http::request_id::RequestId for GetHostedZoneLimitOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetHostedZoneLimitOutput {
    /// Creates a new builder-style object to manufacture [`GetHostedZoneLimitOutput`](crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput).
    pub fn builder(
    ) -> crate::operation::get_hosted_zone_limit::builders::GetHostedZoneLimitOutputBuilder {
        crate::operation::get_hosted_zone_limit::builders::GetHostedZoneLimitOutputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput;
/// A builder for [`GetHostedZoneLimitOutput`](crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput).
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
pub struct GetHostedZoneLimitOutputBuilder {
    pub(crate) limit: std::option::Option<crate::types::HostedZoneLimit>,
    pub(crate) count: std::option::Option<i64>,
    _request_id: Option<String>,
}
impl GetHostedZoneLimitOutputBuilder {
    /// <p>The current setting for the specified limit. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Limit</code> is the maximum number of records that you can create in the specified hosted zone.</p>
    pub fn limit(mut self, input: crate::types::HostedZoneLimit) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The current setting for the specified limit. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Limit</code> is the maximum number of records that you can create in the specified hosted zone.</p>
    pub fn set_limit(mut self, input: std::option::Option<crate::types::HostedZoneLimit>) -> Self {
        self.limit = input;
        self
    }
    /// <p>The current number of entities that you have created of the specified type. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Count</code> is the current number of records that you have created in the specified hosted zone.</p>
    pub fn count(mut self, input: i64) -> Self {
        self.count = Some(input);
        self
    }
    /// <p>The current number of entities that you have created of the specified type. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Count</code> is the current number of records that you have created in the specified hosted zone.</p>
    pub fn set_count(mut self, input: std::option::Option<i64>) -> Self {
        self.count = input;
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
    /// Consumes the builder and constructs a [`GetHostedZoneLimitOutput`](crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput).
    pub fn build(self) -> crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput {
        crate::operation::get_hosted_zone_limit::GetHostedZoneLimitOutput {
            limit: self.limit,
            count: self.count.unwrap_or_default(),
            _request_id: self._request_id,
        }
    }
}