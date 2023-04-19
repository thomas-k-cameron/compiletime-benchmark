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
pub struct GetIpamDiscoveredResourceCidrsInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>A resource discovery ID.</p>
    #[doc(hidden)]
    pub ipam_resource_discovery_id: std::option::Option<std::string::String>,
    /// <p>A resource Region.</p>
    #[doc(hidden)]
    pub resource_region: std::option::Option<std::string::String>,
    /// <p>Filters.</p>
    #[doc(hidden)]
    pub filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of discovered resource CIDRs to return in one page of results.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
}
impl GetIpamDiscoveredResourceCidrsInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>A resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(&self) -> std::option::Option<&str> {
        self.ipam_resource_discovery_id.as_deref()
    }
    /// <p>A resource Region.</p>
    pub fn resource_region(&self) -> std::option::Option<&str> {
        self.resource_region.as_deref()
    }
    /// <p>Filters.</p>
    pub fn filters(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of discovered resource CIDRs to return in one page of results.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
}
impl GetIpamDiscoveredResourceCidrsInput {
    /// Creates a new builder-style object to manufacture [`GetIpamDiscoveredResourceCidrsInput`](crate::operation::get_ipam_discovered_resource_cidrs::GetIpamDiscoveredResourceCidrsInput).
    pub fn builder() -> crate::operation::get_ipam_discovered_resource_cidrs::builders::GetIpamDiscoveredResourceCidrsInputBuilder{
        crate::operation::get_ipam_discovered_resource_cidrs::builders::GetIpamDiscoveredResourceCidrsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_ipam_discovered_resource_cidrs::GetIpamDiscoveredResourceCidrsInput;
/// A builder for [`GetIpamDiscoveredResourceCidrsInput`](crate::operation::get_ipam_discovered_resource_cidrs::GetIpamDiscoveredResourceCidrsInput).
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
pub struct GetIpamDiscoveredResourceCidrsInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) ipam_resource_discovery_id: std::option::Option<std::string::String>,
    pub(crate) resource_region: std::option::Option<std::string::String>,
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) max_results: std::option::Option<i32>,
}
impl GetIpamDiscoveredResourceCidrsInputBuilder {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>A resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipam_resource_discovery_id = Some(input.into());
        self
    }
    /// <p>A resource discovery ID.</p>
    pub fn set_ipam_resource_discovery_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.ipam_resource_discovery_id = input;
        self
    }
    /// <p>A resource Region.</p>
    pub fn resource_region(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_region = Some(input.into());
        self
    }
    /// <p>A resource Region.</p>
    pub fn set_resource_region(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_region = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Filters.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = Some(v);
        self
    }
    /// <p>Filters.</p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of discovered resource CIDRs to return in one page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of discovered resource CIDRs to return in one page of results.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`GetIpamDiscoveredResourceCidrsInput`](crate::operation::get_ipam_discovered_resource_cidrs::GetIpamDiscoveredResourceCidrsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_ipam_discovered_resource_cidrs::GetIpamDiscoveredResourceCidrsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_ipam_discovered_resource_cidrs::GetIpamDiscoveredResourceCidrsInput {
                dry_run: self.dry_run
                ,
                ipam_resource_discovery_id: self.ipam_resource_discovery_id
                ,
                resource_region: self.resource_region
                ,
                filters: self.filters
                ,
                next_token: self.next_token
                ,
                max_results: self.max_results
                ,
            }
        )
    }
}
