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
pub struct DescribePlacementGroupsOutput {
    /// <p>Information about the placement groups.</p>
    #[doc(hidden)]
    pub placement_groups: std::option::Option<std::vec::Vec<crate::types::PlacementGroup>>,
    _request_id: Option<String>,
}
impl DescribePlacementGroupsOutput {
    /// <p>Information about the placement groups.</p>
    pub fn placement_groups(&self) -> std::option::Option<&[crate::types::PlacementGroup]> {
        self.placement_groups.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribePlacementGroupsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribePlacementGroupsOutput {
    /// Creates a new builder-style object to manufacture [`DescribePlacementGroupsOutput`](crate::operation::describe_placement_groups::DescribePlacementGroupsOutput).
    pub fn builder(
    ) -> crate::operation::describe_placement_groups::builders::DescribePlacementGroupsOutputBuilder
    {
        crate::operation::describe_placement_groups::builders::DescribePlacementGroupsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_placement_groups::DescribePlacementGroupsOutput;
/// A builder for [`DescribePlacementGroupsOutput`](crate::operation::describe_placement_groups::DescribePlacementGroupsOutput).
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
pub struct DescribePlacementGroupsOutputBuilder {
    pub(crate) placement_groups: std::option::Option<std::vec::Vec<crate::types::PlacementGroup>>,
    _request_id: Option<String>,
}
impl DescribePlacementGroupsOutputBuilder {
    /// Appends an item to `placement_groups`.
    ///
    /// To override the contents of this collection use [`set_placement_groups`](Self::set_placement_groups).
    ///
    /// <p>Information about the placement groups.</p>
    pub fn placement_groups(mut self, input: crate::types::PlacementGroup) -> Self {
        let mut v = self.placement_groups.unwrap_or_default();
        v.push(input);
        self.placement_groups = Some(v);
        self
    }
    /// <p>Information about the placement groups.</p>
    pub fn set_placement_groups(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PlacementGroup>>,
    ) -> Self {
        self.placement_groups = input;
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
    /// Consumes the builder and constructs a [`DescribePlacementGroupsOutput`](crate::operation::describe_placement_groups::DescribePlacementGroupsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_placement_groups::DescribePlacementGroupsOutput {
        crate::operation::describe_placement_groups::DescribePlacementGroupsOutput {
            placement_groups: self.placement_groups,
            _request_id: self._request_id,
        }
    }
}
