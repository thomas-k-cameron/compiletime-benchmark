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
pub struct ListAggregateDiscoveredResourcesOutput {
    /// <p>Returns a list of <code>ResourceIdentifiers</code> objects.</p>
    #[doc(hidden)]
    pub resource_identifiers:
        std::option::Option<std::vec::Vec<crate::types::AggregateResourceIdentifier>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListAggregateDiscoveredResourcesOutput {
    /// <p>Returns a list of <code>ResourceIdentifiers</code> objects.</p>
    pub fn resource_identifiers(
        &self,
    ) -> std::option::Option<&[crate::types::AggregateResourceIdentifier]> {
        self.resource_identifiers.as_deref()
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListAggregateDiscoveredResourcesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListAggregateDiscoveredResourcesOutput {
    /// Creates a new builder-style object to manufacture [`ListAggregateDiscoveredResourcesOutput`](crate::operation::list_aggregate_discovered_resources::ListAggregateDiscoveredResourcesOutput).
    pub fn builder() -> crate::operation::list_aggregate_discovered_resources::builders::ListAggregateDiscoveredResourcesOutputBuilder{
        crate::operation::list_aggregate_discovered_resources::builders::ListAggregateDiscoveredResourcesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::list_aggregate_discovered_resources::ListAggregateDiscoveredResourcesOutput;
/// A builder for [`ListAggregateDiscoveredResourcesOutput`](crate::operation::list_aggregate_discovered_resources::ListAggregateDiscoveredResourcesOutput).
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
pub struct ListAggregateDiscoveredResourcesOutputBuilder {
    pub(crate) resource_identifiers:
        std::option::Option<std::vec::Vec<crate::types::AggregateResourceIdentifier>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListAggregateDiscoveredResourcesOutputBuilder {
    /// Appends an item to `resource_identifiers`.
    ///
    /// To override the contents of this collection use [`set_resource_identifiers`](Self::set_resource_identifiers).
    ///
    /// <p>Returns a list of <code>ResourceIdentifiers</code> objects.</p>
    pub fn resource_identifiers(
        mut self,
        input: crate::types::AggregateResourceIdentifier,
    ) -> Self {
        let mut v = self.resource_identifiers.unwrap_or_default();
        v.push(input);
        self.resource_identifiers = Some(v);
        self
    }
    /// <p>Returns a list of <code>ResourceIdentifiers</code> objects.</p>
    pub fn set_resource_identifiers(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AggregateResourceIdentifier>>,
    ) -> Self {
        self.resource_identifiers = input;
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
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
    /// Consumes the builder and constructs a [`ListAggregateDiscoveredResourcesOutput`](crate::operation::list_aggregate_discovered_resources::ListAggregateDiscoveredResourcesOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_aggregate_discovered_resources::ListAggregateDiscoveredResourcesOutput
    {
        crate::operation::list_aggregate_discovered_resources::ListAggregateDiscoveredResourcesOutput {
            resource_identifiers: self.resource_identifiers
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
