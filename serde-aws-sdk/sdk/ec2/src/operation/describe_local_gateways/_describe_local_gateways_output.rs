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
pub struct DescribeLocalGatewaysOutput {
    /// <p>Information about the local gateways.</p>
    #[doc(hidden)]
    pub local_gateways: std::option::Option<std::vec::Vec<crate::types::LocalGateway>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLocalGatewaysOutput {
    /// <p>Information about the local gateways.</p>
    pub fn local_gateways(&self) -> std::option::Option<&[crate::types::LocalGateway]> {
        self.local_gateways.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeLocalGatewaysOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeLocalGatewaysOutput {
    /// Creates a new builder-style object to manufacture [`DescribeLocalGatewaysOutput`](crate::operation::describe_local_gateways::DescribeLocalGatewaysOutput).
    pub fn builder(
    ) -> crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysOutputBuilder
    {
        crate::operation::describe_local_gateways::builders::DescribeLocalGatewaysOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_local_gateways::DescribeLocalGatewaysOutput;
/// A builder for [`DescribeLocalGatewaysOutput`](crate::operation::describe_local_gateways::DescribeLocalGatewaysOutput).
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
pub struct DescribeLocalGatewaysOutputBuilder {
    pub(crate) local_gateways: std::option::Option<std::vec::Vec<crate::types::LocalGateway>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLocalGatewaysOutputBuilder {
    /// Appends an item to `local_gateways`.
    ///
    /// To override the contents of this collection use [`set_local_gateways`](Self::set_local_gateways).
    ///
    /// <p>Information about the local gateways.</p>
    pub fn local_gateways(mut self, input: crate::types::LocalGateway) -> Self {
        let mut v = self.local_gateways.unwrap_or_default();
        v.push(input);
        self.local_gateways = Some(v);
        self
    }
    /// <p>Information about the local gateways.</p>
    pub fn set_local_gateways(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::LocalGateway>>,
    ) -> Self {
        self.local_gateways = input;
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
    /// Consumes the builder and constructs a [`DescribeLocalGatewaysOutput`](crate::operation::describe_local_gateways::DescribeLocalGatewaysOutput).
    pub fn build(self) -> crate::operation::describe_local_gateways::DescribeLocalGatewaysOutput {
        crate::operation::describe_local_gateways::DescribeLocalGatewaysOutput {
            local_gateways: self.local_gateways,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
