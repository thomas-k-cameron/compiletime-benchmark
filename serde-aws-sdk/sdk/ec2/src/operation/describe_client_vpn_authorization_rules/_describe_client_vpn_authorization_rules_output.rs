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
pub struct DescribeClientVpnAuthorizationRulesOutput {
    /// <p>Information about the authorization rules.</p>
    #[doc(hidden)]
    pub authorization_rules: std::option::Option<std::vec::Vec<crate::types::AuthorizationRule>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeClientVpnAuthorizationRulesOutput {
    /// <p>Information about the authorization rules.</p>
    pub fn authorization_rules(&self) -> std::option::Option<&[crate::types::AuthorizationRule]> {
        self.authorization_rules.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeClientVpnAuthorizationRulesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeClientVpnAuthorizationRulesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeClientVpnAuthorizationRulesOutput`](crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesOutput).
    pub fn builder() -> crate::operation::describe_client_vpn_authorization_rules::builders::DescribeClientVpnAuthorizationRulesOutputBuilder{
        crate::operation::describe_client_vpn_authorization_rules::builders::DescribeClientVpnAuthorizationRulesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesOutput;
/// A builder for [`DescribeClientVpnAuthorizationRulesOutput`](crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesOutput).
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
pub struct DescribeClientVpnAuthorizationRulesOutputBuilder {
    pub(crate) authorization_rules:
        std::option::Option<std::vec::Vec<crate::types::AuthorizationRule>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeClientVpnAuthorizationRulesOutputBuilder {
    /// Appends an item to `authorization_rules`.
    ///
    /// To override the contents of this collection use [`set_authorization_rules`](Self::set_authorization_rules).
    ///
    /// <p>Information about the authorization rules.</p>
    pub fn authorization_rules(mut self, input: crate::types::AuthorizationRule) -> Self {
        let mut v = self.authorization_rules.unwrap_or_default();
        v.push(input);
        self.authorization_rules = Some(v);
        self
    }
    /// <p>Information about the authorization rules.</p>
    pub fn set_authorization_rules(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AuthorizationRule>>,
    ) -> Self {
        self.authorization_rules = input;
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
    /// Consumes the builder and constructs a [`DescribeClientVpnAuthorizationRulesOutput`](crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesOutput).
    pub fn build(self) -> crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesOutput{
        crate::operation::describe_client_vpn_authorization_rules::DescribeClientVpnAuthorizationRulesOutput {
            authorization_rules: self.authorization_rules
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
