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
pub struct DescribeSecurityGroupRulesInput {
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>group-id</code> - The ID of the security group.</p> </li>
    /// <li> <p> <code>security-group-rule-id</code> - The ID of the security group rule.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>The IDs of the security group rules.</p>
    #[doc(hidden)]
    pub security_group_rule_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. This value can be between 5 and 1000. If this parameter is not specified, then all items are returned. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
}
impl DescribeSecurityGroupRulesInput {
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>group-id</code> - The ID of the security group.</p> </li>
    /// <li> <p> <code>security-group-rule-id</code> - The ID of the security group rule.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// </ul>
    pub fn filters(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The IDs of the security group rules.</p>
    pub fn security_group_rule_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.security_group_rule_ids.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. This value can be between 5 and 1000. If this parameter is not specified, then all items are returned. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
}
impl DescribeSecurityGroupRulesInput {
    /// Creates a new builder-style object to manufacture [`DescribeSecurityGroupRulesInput`](crate::operation::describe_security_group_rules::DescribeSecurityGroupRulesInput).
    pub fn builder() -> crate::operation::describe_security_group_rules::builders::DescribeSecurityGroupRulesInputBuilder{
        crate::operation::describe_security_group_rules::builders::DescribeSecurityGroupRulesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_security_group_rules::DescribeSecurityGroupRulesInput;
/// A builder for [`DescribeSecurityGroupRulesInput`](crate::operation::describe_security_group_rules::DescribeSecurityGroupRulesInput).
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
pub struct DescribeSecurityGroupRulesInputBuilder {
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) security_group_rule_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) max_results: std::option::Option<i32>,
}
impl DescribeSecurityGroupRulesInputBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>group-id</code> - The ID of the security group.</p> </li>
    /// <li> <p> <code>security-group-rule-id</code> - The ID of the security group rule.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = Some(v);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>group-id</code> - The ID of the security group.</p> </li>
    /// <li> <p> <code>security-group-rule-id</code> - The ID of the security group rule.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// Appends an item to `security_group_rule_ids`.
    ///
    /// To override the contents of this collection use [`set_security_group_rule_ids`](Self::set_security_group_rule_ids).
    ///
    /// <p>The IDs of the security group rules.</p>
    pub fn security_group_rule_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.security_group_rule_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_rule_ids = Some(v);
        self
    }
    /// <p>The IDs of the security group rules.</p>
    pub fn set_security_group_rule_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.security_group_rule_ids = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. This value can be between 5 and 1000. If this parameter is not specified, then all items are returned. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. This value can be between 5 and 1000. If this parameter is not specified, then all items are returned. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeSecurityGroupRulesInput`](crate::operation::describe_security_group_rules::DescribeSecurityGroupRulesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_security_group_rules::DescribeSecurityGroupRulesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_security_group_rules::DescribeSecurityGroupRulesInput {
                filters: self.filters,
                security_group_rule_ids: self.security_group_rule_ids,
                dry_run: self.dry_run,
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}
