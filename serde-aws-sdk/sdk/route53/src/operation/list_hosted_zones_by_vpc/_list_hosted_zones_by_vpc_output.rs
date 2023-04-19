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
pub struct ListHostedZonesByVpcOutput {
    /// <p>A list that contains one <code>HostedZoneSummary</code> element for each hosted zone that the specified Amazon VPC is associated with. Each <code>HostedZoneSummary</code> element contains the hosted zone name and ID, and information about who owns the hosted zone.</p>
    #[doc(hidden)]
    pub hosted_zone_summaries: std::option::Option<std::vec::Vec<crate::types::HostedZoneSummary>>,
    /// <p>The value that you specified for <code>MaxItems</code> in the most recent <code>ListHostedZonesByVPC</code> request.</p>
    #[doc(hidden)]
    pub max_items: std::option::Option<i32>,
    /// <p>The value that you will use for <code>NextToken</code> in the next <code>ListHostedZonesByVPC</code> request.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListHostedZonesByVpcOutput {
    /// <p>A list that contains one <code>HostedZoneSummary</code> element for each hosted zone that the specified Amazon VPC is associated with. Each <code>HostedZoneSummary</code> element contains the hosted zone name and ID, and information about who owns the hosted zone.</p>
    pub fn hosted_zone_summaries(&self) -> std::option::Option<&[crate::types::HostedZoneSummary]> {
        self.hosted_zone_summaries.as_deref()
    }
    /// <p>The value that you specified for <code>MaxItems</code> in the most recent <code>ListHostedZonesByVPC</code> request.</p>
    pub fn max_items(&self) -> std::option::Option<i32> {
        self.max_items
    }
    /// <p>The value that you will use for <code>NextToken</code> in the next <code>ListHostedZonesByVPC</code> request.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListHostedZonesByVpcOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListHostedZonesByVpcOutput {
    /// Creates a new builder-style object to manufacture [`ListHostedZonesByVpcOutput`](crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVpcOutput).
    pub fn builder(
    ) -> crate::operation::list_hosted_zones_by_vpc::builders::ListHostedZonesByVpcOutputBuilder
    {
        crate::operation::list_hosted_zones_by_vpc::builders::ListHostedZonesByVpcOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVpcOutput;
/// A builder for [`ListHostedZonesByVpcOutput`](crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVpcOutput).
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
pub struct ListHostedZonesByVpcOutputBuilder {
    pub(crate) hosted_zone_summaries:
        std::option::Option<std::vec::Vec<crate::types::HostedZoneSummary>>,
    pub(crate) max_items: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListHostedZonesByVpcOutputBuilder {
    /// Appends an item to `hosted_zone_summaries`.
    ///
    /// To override the contents of this collection use [`set_hosted_zone_summaries`](Self::set_hosted_zone_summaries).
    ///
    /// <p>A list that contains one <code>HostedZoneSummary</code> element for each hosted zone that the specified Amazon VPC is associated with. Each <code>HostedZoneSummary</code> element contains the hosted zone name and ID, and information about who owns the hosted zone.</p>
    pub fn hosted_zone_summaries(mut self, input: crate::types::HostedZoneSummary) -> Self {
        let mut v = self.hosted_zone_summaries.unwrap_or_default();
        v.push(input);
        self.hosted_zone_summaries = Some(v);
        self
    }
    /// <p>A list that contains one <code>HostedZoneSummary</code> element for each hosted zone that the specified Amazon VPC is associated with. Each <code>HostedZoneSummary</code> element contains the hosted zone name and ID, and information about who owns the hosted zone.</p>
    pub fn set_hosted_zone_summaries(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::HostedZoneSummary>>,
    ) -> Self {
        self.hosted_zone_summaries = input;
        self
    }
    /// <p>The value that you specified for <code>MaxItems</code> in the most recent <code>ListHostedZonesByVPC</code> request.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = Some(input);
        self
    }
    /// <p>The value that you specified for <code>MaxItems</code> in the most recent <code>ListHostedZonesByVPC</code> request.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// <p>The value that you will use for <code>NextToken</code> in the next <code>ListHostedZonesByVPC</code> request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The value that you will use for <code>NextToken</code> in the next <code>ListHostedZonesByVPC</code> request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListHostedZonesByVpcOutput`](crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVpcOutput).
    pub fn build(self) -> crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVpcOutput {
        crate::operation::list_hosted_zones_by_vpc::ListHostedZonesByVpcOutput {
            hosted_zone_summaries: self.hosted_zone_summaries,
            max_items: self.max_items,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
