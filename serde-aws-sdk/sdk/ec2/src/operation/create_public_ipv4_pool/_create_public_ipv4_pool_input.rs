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
pub struct CreatePublicIpv4PoolInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p>
    #[doc(hidden)]
    pub tag_specifications: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
}
impl CreatePublicIpv4PoolInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p>
    pub fn tag_specifications(&self) -> std::option::Option<&[crate::types::TagSpecification]> {
        self.tag_specifications.as_deref()
    }
}
impl CreatePublicIpv4PoolInput {
    /// Creates a new builder-style object to manufacture [`CreatePublicIpv4PoolInput`](crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolInput).
    pub fn builder(
    ) -> crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolInputBuilder {
        crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolInput;
/// A builder for [`CreatePublicIpv4PoolInput`](crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolInput).
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
pub struct CreatePublicIpv4PoolInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) tag_specifications:
        std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
}
impl CreatePublicIpv4PoolInputBuilder {
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
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = Some(v);
        self
    }
    /// <p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.tag_specifications = input;
        self
    }
    /// Consumes the builder and constructs a [`CreatePublicIpv4PoolInput`](crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolInput {
                dry_run: self.dry_run,
                tag_specifications: self.tag_specifications,
            },
        )
    }
}
