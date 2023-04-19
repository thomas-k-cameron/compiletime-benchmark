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
pub struct DescribeTransitGatewayConnectsOutput {
    /// <p>Information about the Connect attachments.</p>
    #[doc(hidden)]
    pub transit_gateway_connects:
        std::option::Option<std::vec::Vec<crate::types::TransitGatewayConnect>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTransitGatewayConnectsOutput {
    /// <p>Information about the Connect attachments.</p>
    pub fn transit_gateway_connects(
        &self,
    ) -> std::option::Option<&[crate::types::TransitGatewayConnect]> {
        self.transit_gateway_connects.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeTransitGatewayConnectsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeTransitGatewayConnectsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTransitGatewayConnectsOutput`](crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput).
    pub fn builder() -> crate::operation::describe_transit_gateway_connects::builders::DescribeTransitGatewayConnectsOutputBuilder{
        crate::operation::describe_transit_gateway_connects::builders::DescribeTransitGatewayConnectsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput;
/// A builder for [`DescribeTransitGatewayConnectsOutput`](crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput).
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
pub struct DescribeTransitGatewayConnectsOutputBuilder {
    pub(crate) transit_gateway_connects:
        std::option::Option<std::vec::Vec<crate::types::TransitGatewayConnect>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTransitGatewayConnectsOutputBuilder {
    /// Appends an item to `transit_gateway_connects`.
    ///
    /// To override the contents of this collection use [`set_transit_gateway_connects`](Self::set_transit_gateway_connects).
    ///
    /// <p>Information about the Connect attachments.</p>
    pub fn transit_gateway_connects(mut self, input: crate::types::TransitGatewayConnect) -> Self {
        let mut v = self.transit_gateway_connects.unwrap_or_default();
        v.push(input);
        self.transit_gateway_connects = Some(v);
        self
    }
    /// <p>Information about the Connect attachments.</p>
    pub fn set_transit_gateway_connects(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TransitGatewayConnect>>,
    ) -> Self {
        self.transit_gateway_connects = input;
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
    /// Consumes the builder and constructs a [`DescribeTransitGatewayConnectsOutput`](crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput
    {
        crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput {
            transit_gateway_connects: self.transit_gateway_connects,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}