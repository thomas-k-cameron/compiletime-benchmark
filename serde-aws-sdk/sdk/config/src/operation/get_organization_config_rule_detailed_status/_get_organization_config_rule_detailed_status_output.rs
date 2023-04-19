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
pub struct GetOrganizationConfigRuleDetailedStatusOutput {
    /// <p>A list of <code>MemberAccountStatus</code> objects.</p>
    #[doc(hidden)]
    pub organization_config_rule_detailed_status:
        std::option::Option<std::vec::Vec<crate::types::MemberAccountStatus>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetOrganizationConfigRuleDetailedStatusOutput {
    /// <p>A list of <code>MemberAccountStatus</code> objects.</p>
    pub fn organization_config_rule_detailed_status(
        &self,
    ) -> std::option::Option<&[crate::types::MemberAccountStatus]> {
        self.organization_config_rule_detailed_status.as_deref()
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetOrganizationConfigRuleDetailedStatusOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetOrganizationConfigRuleDetailedStatusOutput {
    /// Creates a new builder-style object to manufacture [`GetOrganizationConfigRuleDetailedStatusOutput`](crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput).
    pub fn builder() -> crate::operation::get_organization_config_rule_detailed_status::builders::GetOrganizationConfigRuleDetailedStatusOutputBuilder{
        crate::operation::get_organization_config_rule_detailed_status::builders::GetOrganizationConfigRuleDetailedStatusOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput;
/// A builder for [`GetOrganizationConfigRuleDetailedStatusOutput`](crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput).
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
pub struct GetOrganizationConfigRuleDetailedStatusOutputBuilder {
    pub(crate) organization_config_rule_detailed_status:
        std::option::Option<std::vec::Vec<crate::types::MemberAccountStatus>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetOrganizationConfigRuleDetailedStatusOutputBuilder {
    /// Appends an item to `organization_config_rule_detailed_status`.
    ///
    /// To override the contents of this collection use [`set_organization_config_rule_detailed_status`](Self::set_organization_config_rule_detailed_status).
    ///
    /// <p>A list of <code>MemberAccountStatus</code> objects.</p>
    pub fn organization_config_rule_detailed_status(
        mut self,
        input: crate::types::MemberAccountStatus,
    ) -> Self {
        let mut v = self
            .organization_config_rule_detailed_status
            .unwrap_or_default();
        v.push(input);
        self.organization_config_rule_detailed_status = Some(v);
        self
    }
    /// <p>A list of <code>MemberAccountStatus</code> objects.</p>
    pub fn set_organization_config_rule_detailed_status(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::MemberAccountStatus>>,
    ) -> Self {
        self.organization_config_rule_detailed_status = input;
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
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
    /// Consumes the builder and constructs a [`GetOrganizationConfigRuleDetailedStatusOutput`](crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput).
    pub fn build(self) -> crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput{
        crate::operation::get_organization_config_rule_detailed_status::GetOrganizationConfigRuleDetailedStatusOutput {
            organization_config_rule_detailed_status: self.organization_config_rule_detailed_status
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}