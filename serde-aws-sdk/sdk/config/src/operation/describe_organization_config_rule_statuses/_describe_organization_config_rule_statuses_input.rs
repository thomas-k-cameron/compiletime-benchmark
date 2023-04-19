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
pub struct DescribeOrganizationConfigRuleStatusesInput {
    /// <p>The names of organization Config rules for which you want status details. If you do not specify any names, Config returns details for all your organization Config rules.</p>
    #[doc(hidden)]
    pub organization_config_rule_names: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The maximum number of <code>OrganizationConfigRuleStatuses</code> returned on each page. If you do no specify a number, Config uses the default. The default is 100.</p>
    #[doc(hidden)]
    pub limit: i32,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeOrganizationConfigRuleStatusesInput {
    /// <p>The names of organization Config rules for which you want status details. If you do not specify any names, Config returns details for all your organization Config rules.</p>
    pub fn organization_config_rule_names(&self) -> std::option::Option<&[std::string::String]> {
        self.organization_config_rule_names.as_deref()
    }
    /// <p>The maximum number of <code>OrganizationConfigRuleStatuses</code> returned on each page. If you do no specify a number, Config uses the default. The default is 100.</p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeOrganizationConfigRuleStatusesInput {
    /// Creates a new builder-style object to manufacture [`DescribeOrganizationConfigRuleStatusesInput`](crate::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesInput).
    pub fn builder() -> crate::operation::describe_organization_config_rule_statuses::builders::DescribeOrganizationConfigRuleStatusesInputBuilder{
        crate::operation::describe_organization_config_rule_statuses::builders::DescribeOrganizationConfigRuleStatusesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesInput;
/// A builder for [`DescribeOrganizationConfigRuleStatusesInput`](crate::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesInput).
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
pub struct DescribeOrganizationConfigRuleStatusesInputBuilder {
    pub(crate) organization_config_rule_names:
        std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl DescribeOrganizationConfigRuleStatusesInputBuilder {
    /// Appends an item to `organization_config_rule_names`.
    ///
    /// To override the contents of this collection use [`set_organization_config_rule_names`](Self::set_organization_config_rule_names).
    ///
    /// <p>The names of organization Config rules for which you want status details. If you do not specify any names, Config returns details for all your organization Config rules.</p>
    pub fn organization_config_rule_names(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.organization_config_rule_names.unwrap_or_default();
        v.push(input.into());
        self.organization_config_rule_names = Some(v);
        self
    }
    /// <p>The names of organization Config rules for which you want status details. If you do not specify any names, Config returns details for all your organization Config rules.</p>
    pub fn set_organization_config_rule_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.organization_config_rule_names = input;
        self
    }
    /// <p>The maximum number of <code>OrganizationConfigRuleStatuses</code> returned on each page. If you do no specify a number, Config uses the default. The default is 100.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The maximum number of <code>OrganizationConfigRuleStatuses</code> returned on each page. If you do no specify a number, Config uses the default. The default is 100.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.limit = input;
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
    /// Consumes the builder and constructs a [`DescribeOrganizationConfigRuleStatusesInput`](crate::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesInput).
    pub fn build(self) -> Result<crate::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::describe_organization_config_rule_statuses::DescribeOrganizationConfigRuleStatusesInput {
                organization_config_rule_names: self.organization_config_rule_names
                ,
                limit: self.limit
                    .unwrap_or_default()
                ,
                next_token: self.next_token
                ,
            }
        )
    }
}
