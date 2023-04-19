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
pub struct ListHostedZonesOutput {
    /// <p>A complex type that contains general information about the hosted zone.</p>
    #[doc(hidden)]
    pub hosted_zones: std::option::Option<std::vec::Vec<crate::types::HostedZone>>,
    /// <p>For the second and subsequent calls to <code>ListHostedZones</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the request that produced the current response.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
    /// <p>A flag indicating whether there are more hosted zones to be listed. If the response was truncated, you can get more hosted zones by submitting another <code>ListHostedZones</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    #[doc(hidden)]
    pub is_truncated: bool,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first hosted zone in the next group of hosted zones. Submit another <code>ListHostedZones</code> request, and specify the value of <code>NextMarker</code> from the response in the <code>marker</code> parameter.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    #[doc(hidden)]
    pub next_marker: std::option::Option<std::string::String>,
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZones</code> that produced the current response.</p>
    #[doc(hidden)]
    pub max_items: std::option::Option<i32>,
    _request_id: Option<String>,
}
impl ListHostedZonesOutput {
    /// <p>A complex type that contains general information about the hosted zone.</p>
    pub fn hosted_zones(&self) -> std::option::Option<&[crate::types::HostedZone]> {
        self.hosted_zones.as_deref()
    }
    /// <p>For the second and subsequent calls to <code>ListHostedZones</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the request that produced the current response.</p>
    pub fn marker(&self) -> std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>A flag indicating whether there are more hosted zones to be listed. If the response was truncated, you can get more hosted zones by submitting another <code>ListHostedZones</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first hosted zone in the next group of hosted zones. Submit another <code>ListHostedZones</code> request, and specify the value of <code>NextMarker</code> from the response in the <code>marker</code> parameter.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub fn next_marker(&self) -> std::option::Option<&str> {
        self.next_marker.as_deref()
    }
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZones</code> that produced the current response.</p>
    pub fn max_items(&self) -> std::option::Option<i32> {
        self.max_items
    }
}
impl aws_http::request_id::RequestId for ListHostedZonesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListHostedZonesOutput {
    /// Creates a new builder-style object to manufacture [`ListHostedZonesOutput`](crate::operation::list_hosted_zones::ListHostedZonesOutput).
    pub fn builder() -> crate::operation::list_hosted_zones::builders::ListHostedZonesOutputBuilder
    {
        crate::operation::list_hosted_zones::builders::ListHostedZonesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_hosted_zones::ListHostedZonesOutput;
/// A builder for [`ListHostedZonesOutput`](crate::operation::list_hosted_zones::ListHostedZonesOutput).
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
pub struct ListHostedZonesOutputBuilder {
    pub(crate) hosted_zones: std::option::Option<std::vec::Vec<crate::types::HostedZone>>,
    pub(crate) marker: std::option::Option<std::string::String>,
    pub(crate) is_truncated: std::option::Option<bool>,
    pub(crate) next_marker: std::option::Option<std::string::String>,
    pub(crate) max_items: std::option::Option<i32>,
    _request_id: Option<String>,
}
impl ListHostedZonesOutputBuilder {
    /// Appends an item to `hosted_zones`.
    ///
    /// To override the contents of this collection use [`set_hosted_zones`](Self::set_hosted_zones).
    ///
    /// <p>A complex type that contains general information about the hosted zone.</p>
    pub fn hosted_zones(mut self, input: crate::types::HostedZone) -> Self {
        let mut v = self.hosted_zones.unwrap_or_default();
        v.push(input);
        self.hosted_zones = Some(v);
        self
    }
    /// <p>A complex type that contains general information about the hosted zone.</p>
    pub fn set_hosted_zones(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::HostedZone>>,
    ) -> Self {
        self.hosted_zones = input;
        self
    }
    /// <p>For the second and subsequent calls to <code>ListHostedZones</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the request that produced the current response.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.marker = Some(input.into());
        self
    }
    /// <p>For the second and subsequent calls to <code>ListHostedZones</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the request that produced the current response.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>A flag indicating whether there are more hosted zones to be listed. If the response was truncated, you can get more hosted zones by submitting another <code>ListHostedZones</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = Some(input);
        self
    }
    /// <p>A flag indicating whether there are more hosted zones to be listed. If the response was truncated, you can get more hosted zones by submitting another <code>ListHostedZones</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub fn set_is_truncated(mut self, input: std::option::Option<bool>) -> Self {
        self.is_truncated = input;
        self
    }
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first hosted zone in the next group of hosted zones. Submit another <code>ListHostedZones</code> request, and specify the value of <code>NextMarker</code> from the response in the <code>marker</code> parameter.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub fn next_marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_marker = Some(input.into());
        self
    }
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first hosted zone in the next group of hosted zones. Submit another <code>ListHostedZones</code> request, and specify the value of <code>NextMarker</code> from the response in the <code>marker</code> parameter.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub fn set_next_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZones</code> that produced the current response.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = Some(input);
        self
    }
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZones</code> that produced the current response.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.max_items = input;
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
    /// Consumes the builder and constructs a [`ListHostedZonesOutput`](crate::operation::list_hosted_zones::ListHostedZonesOutput).
    pub fn build(self) -> crate::operation::list_hosted_zones::ListHostedZonesOutput {
        crate::operation::list_hosted_zones::ListHostedZonesOutput {
            hosted_zones: self.hosted_zones,
            marker: self.marker,
            is_truncated: self.is_truncated.unwrap_or_default(),
            next_marker: self.next_marker,
            max_items: self.max_items,
            _request_id: self._request_id,
        }
    }
}