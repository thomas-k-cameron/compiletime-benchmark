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
pub struct GetAssociatedIpv6PoolCidrsOutput {
    /// <p>Information about the IPv6 CIDR block associations.</p>
    #[doc(hidden)]
    pub ipv6_cidr_associations:
        std::option::Option<std::vec::Vec<crate::types::Ipv6CidrAssociation>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetAssociatedIpv6PoolCidrsOutput {
    /// <p>Information about the IPv6 CIDR block associations.</p>
    pub fn ipv6_cidr_associations(
        &self,
    ) -> std::option::Option<&[crate::types::Ipv6CidrAssociation]> {
        self.ipv6_cidr_associations.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetAssociatedIpv6PoolCidrsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAssociatedIpv6PoolCidrsOutput {
    /// Creates a new builder-style object to manufacture [`GetAssociatedIpv6PoolCidrsOutput`](crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput).
    pub fn builder() -> crate::operation::get_associated_ipv6_pool_cidrs::builders::GetAssociatedIpv6PoolCidrsOutputBuilder{
        crate::operation::get_associated_ipv6_pool_cidrs::builders::GetAssociatedIpv6PoolCidrsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput;
/// A builder for [`GetAssociatedIpv6PoolCidrsOutput`](crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput).
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
pub struct GetAssociatedIpv6PoolCidrsOutputBuilder {
    pub(crate) ipv6_cidr_associations:
        std::option::Option<std::vec::Vec<crate::types::Ipv6CidrAssociation>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetAssociatedIpv6PoolCidrsOutputBuilder {
    /// Appends an item to `ipv6_cidr_associations`.
    ///
    /// To override the contents of this collection use [`set_ipv6_cidr_associations`](Self::set_ipv6_cidr_associations).
    ///
    /// <p>Information about the IPv6 CIDR block associations.</p>
    pub fn ipv6_cidr_associations(mut self, input: crate::types::Ipv6CidrAssociation) -> Self {
        let mut v = self.ipv6_cidr_associations.unwrap_or_default();
        v.push(input);
        self.ipv6_cidr_associations = Some(v);
        self
    }
    /// <p>Information about the IPv6 CIDR block associations.</p>
    pub fn set_ipv6_cidr_associations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Ipv6CidrAssociation>>,
    ) -> Self {
        self.ipv6_cidr_associations = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
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
    /// Consumes the builder and constructs a [`GetAssociatedIpv6PoolCidrsOutput`](crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput {
        crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput {
            ipv6_cidr_associations: self.ipv6_cidr_associations,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
