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
pub struct DescribeConformancePacksOutput {
    /// <p>Returns a list of <code>ConformancePackDetail</code> objects.</p>
    #[doc(hidden)]
    pub conformance_pack_details:
        std::option::Option<std::vec::Vec<crate::types::ConformancePackDetail>>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeConformancePacksOutput {
    /// <p>Returns a list of <code>ConformancePackDetail</code> objects.</p>
    pub fn conformance_pack_details(
        &self,
    ) -> std::option::Option<&[crate::types::ConformancePackDetail]> {
        self.conformance_pack_details.as_deref()
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeConformancePacksOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeConformancePacksOutput {
    /// Creates a new builder-style object to manufacture [`DescribeConformancePacksOutput`](crate::operation::describe_conformance_packs::DescribeConformancePacksOutput).
    pub fn builder(
    ) -> crate::operation::describe_conformance_packs::builders::DescribeConformancePacksOutputBuilder
    {
        crate::operation::describe_conformance_packs::builders::DescribeConformancePacksOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_conformance_packs::DescribeConformancePacksOutput;
/// A builder for [`DescribeConformancePacksOutput`](crate::operation::describe_conformance_packs::DescribeConformancePacksOutput).
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
pub struct DescribeConformancePacksOutputBuilder {
    pub(crate) conformance_pack_details:
        std::option::Option<std::vec::Vec<crate::types::ConformancePackDetail>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeConformancePacksOutputBuilder {
    /// Appends an item to `conformance_pack_details`.
    ///
    /// To override the contents of this collection use [`set_conformance_pack_details`](Self::set_conformance_pack_details).
    ///
    /// <p>Returns a list of <code>ConformancePackDetail</code> objects.</p>
    pub fn conformance_pack_details(mut self, input: crate::types::ConformancePackDetail) -> Self {
        let mut v = self.conformance_pack_details.unwrap_or_default();
        v.push(input);
        self.conformance_pack_details = Some(v);
        self
    }
    /// <p>Returns a list of <code>ConformancePackDetail</code> objects.</p>
    pub fn set_conformance_pack_details(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ConformancePackDetail>>,
    ) -> Self {
        self.conformance_pack_details = input;
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
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
    /// Consumes the builder and constructs a [`DescribeConformancePacksOutput`](crate::operation::describe_conformance_packs::DescribeConformancePacksOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_conformance_packs::DescribeConformancePacksOutput {
        crate::operation::describe_conformance_packs::DescribeConformancePacksOutput {
            conformance_pack_details: self.conformance_pack_details,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
