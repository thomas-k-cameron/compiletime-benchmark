// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the response information for the request.</p>
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
pub struct ListHostedZonesByNameOutput {
    /// <p>A complex type that contains general information about the hosted zone.</p>
    #[doc(hidden)]
    pub hosted_zones: std::option::Option<std::vec::Vec<crate::types::HostedZone>>,
    /// <p>For the second and subsequent calls to <code>ListHostedZonesByName</code>, <code>DNSName</code> is the value that you specified for the <code>dnsname</code> parameter in the request that produced the current response.</p>
    #[doc(hidden)]
    pub dns_name: std::option::Option<std::string::String>,
    /// <p>The ID that Amazon Route 53 assigned to the hosted zone when you created it.</p>
    #[doc(hidden)]
    pub hosted_zone_id: std::option::Option<std::string::String>,
    /// <p>A flag that indicates whether there are more hosted zones to be listed. If the response was truncated, you can get the next group of <code>maxitems</code> hosted zones by calling <code>ListHostedZonesByName</code> again and specifying the values of <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the <code>dnsname</code> and <code>hostedzoneid</code> parameters.</p>
    #[doc(hidden)]
    pub is_truncated: bool,
    /// <p>If <code>IsTruncated</code> is true, the value of <code>NextDNSName</code> is the name of the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    #[doc(hidden)]
    pub next_dns_name: std::option::Option<std::string::String>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextHostedZoneId</code> identifies the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    #[doc(hidden)]
    pub next_hosted_zone_id: std::option::Option<std::string::String>,
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZonesByName</code> that produced the current response.</p>
    #[doc(hidden)]
    pub max_items: std::option::Option<i32>,
    _request_id: Option<String>,
}
impl ListHostedZonesByNameOutput {
    /// <p>A complex type that contains general information about the hosted zone.</p>
    pub fn hosted_zones(&self) -> std::option::Option<&[crate::types::HostedZone]> {
        self.hosted_zones.as_deref()
    }
    /// <p>For the second and subsequent calls to <code>ListHostedZonesByName</code>, <code>DNSName</code> is the value that you specified for the <code>dnsname</code> parameter in the request that produced the current response.</p>
    pub fn dns_name(&self) -> std::option::Option<&str> {
        self.dns_name.as_deref()
    }
    /// <p>The ID that Amazon Route 53 assigned to the hosted zone when you created it.</p>
    pub fn hosted_zone_id(&self) -> std::option::Option<&str> {
        self.hosted_zone_id.as_deref()
    }
    /// <p>A flag that indicates whether there are more hosted zones to be listed. If the response was truncated, you can get the next group of <code>maxitems</code> hosted zones by calling <code>ListHostedZonesByName</code> again and specifying the values of <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the <code>dnsname</code> and <code>hostedzoneid</code> parameters.</p>
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }
    /// <p>If <code>IsTruncated</code> is true, the value of <code>NextDNSName</code> is the name of the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub fn next_dns_name(&self) -> std::option::Option<&str> {
        self.next_dns_name.as_deref()
    }
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextHostedZoneId</code> identifies the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub fn next_hosted_zone_id(&self) -> std::option::Option<&str> {
        self.next_hosted_zone_id.as_deref()
    }
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZonesByName</code> that produced the current response.</p>
    pub fn max_items(&self) -> std::option::Option<i32> {
        self.max_items
    }
}
impl aws_http::request_id::RequestId for ListHostedZonesByNameOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListHostedZonesByNameOutput {
    /// Creates a new builder-style object to manufacture [`ListHostedZonesByNameOutput`](crate::operation::list_hosted_zones_by_name::ListHostedZonesByNameOutput).
    pub fn builder(
    ) -> crate::operation::list_hosted_zones_by_name::builders::ListHostedZonesByNameOutputBuilder
    {
        crate::operation::list_hosted_zones_by_name::builders::ListHostedZonesByNameOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_hosted_zones_by_name::ListHostedZonesByNameOutput;
/// A builder for [`ListHostedZonesByNameOutput`](crate::operation::list_hosted_zones_by_name::ListHostedZonesByNameOutput).
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
pub struct ListHostedZonesByNameOutputBuilder {
    pub(crate) hosted_zones: std::option::Option<std::vec::Vec<crate::types::HostedZone>>,
    pub(crate) dns_name: std::option::Option<std::string::String>,
    pub(crate) hosted_zone_id: std::option::Option<std::string::String>,
    pub(crate) is_truncated: std::option::Option<bool>,
    pub(crate) next_dns_name: std::option::Option<std::string::String>,
    pub(crate) next_hosted_zone_id: std::option::Option<std::string::String>,
    pub(crate) max_items: std::option::Option<i32>,
    _request_id: Option<String>,
}
impl ListHostedZonesByNameOutputBuilder {
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
    /// <p>For the second and subsequent calls to <code>ListHostedZonesByName</code>, <code>DNSName</code> is the value that you specified for the <code>dnsname</code> parameter in the request that produced the current response.</p>
    pub fn dns_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.dns_name = Some(input.into());
        self
    }
    /// <p>For the second and subsequent calls to <code>ListHostedZonesByName</code>, <code>DNSName</code> is the value that you specified for the <code>dnsname</code> parameter in the request that produced the current response.</p>
    pub fn set_dns_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.dns_name = input;
        self
    }
    /// <p>The ID that Amazon Route 53 assigned to the hosted zone when you created it.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.hosted_zone_id = Some(input.into());
        self
    }
    /// <p>The ID that Amazon Route 53 assigned to the hosted zone when you created it.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.hosted_zone_id = input;
        self
    }
    /// <p>A flag that indicates whether there are more hosted zones to be listed. If the response was truncated, you can get the next group of <code>maxitems</code> hosted zones by calling <code>ListHostedZonesByName</code> again and specifying the values of <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the <code>dnsname</code> and <code>hostedzoneid</code> parameters.</p>
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = Some(input);
        self
    }
    /// <p>A flag that indicates whether there are more hosted zones to be listed. If the response was truncated, you can get the next group of <code>maxitems</code> hosted zones by calling <code>ListHostedZonesByName</code> again and specifying the values of <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the <code>dnsname</code> and <code>hostedzoneid</code> parameters.</p>
    pub fn set_is_truncated(mut self, input: std::option::Option<bool>) -> Self {
        self.is_truncated = input;
        self
    }
    /// <p>If <code>IsTruncated</code> is true, the value of <code>NextDNSName</code> is the name of the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub fn next_dns_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_dns_name = Some(input.into());
        self
    }
    /// <p>If <code>IsTruncated</code> is true, the value of <code>NextDNSName</code> is the name of the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub fn set_next_dns_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_dns_name = input;
        self
    }
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextHostedZoneId</code> identifies the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub fn next_hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_hosted_zone_id = Some(input.into());
        self
    }
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextHostedZoneId</code> identifies the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p>
    /// <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub fn set_next_hosted_zone_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.next_hosted_zone_id = input;
        self
    }
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZonesByName</code> that produced the current response.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = Some(input);
        self
    }
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZonesByName</code> that produced the current response.</p>
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
    /// Consumes the builder and constructs a [`ListHostedZonesByNameOutput`](crate::operation::list_hosted_zones_by_name::ListHostedZonesByNameOutput).
    pub fn build(self) -> crate::operation::list_hosted_zones_by_name::ListHostedZonesByNameOutput {
        crate::operation::list_hosted_zones_by_name::ListHostedZonesByNameOutput {
            hosted_zones: self.hosted_zones,
            dns_name: self.dns_name,
            hosted_zone_id: self.hosted_zone_id,
            is_truncated: self.is_truncated.unwrap_or_default(),
            next_dns_name: self.next_dns_name,
            next_hosted_zone_id: self.next_hosted_zone_id,
            max_items: self.max_items,
            _request_id: self._request_id,
        }
    }
}