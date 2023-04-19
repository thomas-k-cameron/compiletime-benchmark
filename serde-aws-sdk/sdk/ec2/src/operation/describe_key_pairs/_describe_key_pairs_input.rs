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
pub struct DescribeKeyPairsInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>key-pair-id</code> - The ID of the key pair.</p> </li>
    /// <li> <p> <code>fingerprint</code> - The fingerprint of the key pair.</p> </li>
    /// <li> <p> <code>key-name</code> - The name of the key pair.</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
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
    /// <p>The key pair names.</p>
    /// <p>Default: Describes all of your key pairs.</p>
    #[doc(hidden)]
    pub key_names: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The IDs of the key pairs.</p>
    #[doc(hidden)]
    pub key_pair_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>If <code>true</code>, the public key material is included in the response.</p>
    /// <p>Default: <code>false</code> </p>
    #[doc(hidden)]
    pub include_public_key: std::option::Option<bool>,
}
impl DescribeKeyPairsInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>key-pair-id</code> - The ID of the key pair.</p> </li>
    /// <li> <p> <code>fingerprint</code> - The fingerprint of the key pair.</p> </li>
    /// <li> <p> <code>key-name</code> - The name of the key pair.</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
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
    /// <p>The key pair names.</p>
    /// <p>Default: Describes all of your key pairs.</p>
    pub fn key_names(&self) -> std::option::Option<&[std::string::String]> {
        self.key_names.as_deref()
    }
    /// <p>The IDs of the key pairs.</p>
    pub fn key_pair_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.key_pair_ids.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>If <code>true</code>, the public key material is included in the response.</p>
    /// <p>Default: <code>false</code> </p>
    pub fn include_public_key(&self) -> std::option::Option<bool> {
        self.include_public_key
    }
}
impl DescribeKeyPairsInput {
    /// Creates a new builder-style object to manufacture [`DescribeKeyPairsInput`](crate::operation::describe_key_pairs::DescribeKeyPairsInput).
    pub fn builder() -> crate::operation::describe_key_pairs::builders::DescribeKeyPairsInputBuilder
    {
        crate::operation::describe_key_pairs::builders::DescribeKeyPairsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_key_pairs::DescribeKeyPairsInput;
/// A builder for [`DescribeKeyPairsInput`](crate::operation::describe_key_pairs::DescribeKeyPairsInput).
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
pub struct DescribeKeyPairsInputBuilder {
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) key_names: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) key_pair_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) include_public_key: std::option::Option<bool>,
}
impl DescribeKeyPairsInputBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>key-pair-id</code> - The ID of the key pair.</p> </li>
    /// <li> <p> <code>fingerprint</code> - The fingerprint of the key pair.</p> </li>
    /// <li> <p> <code>key-name</code> - The name of the key pair.</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
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
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>key-pair-id</code> - The ID of the key pair.</p> </li>
    /// <li> <p> <code>fingerprint</code> - The fingerprint of the key pair.</p> </li>
    /// <li> <p> <code>key-name</code> - The name of the key pair.</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
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
    /// Appends an item to `key_names`.
    ///
    /// To override the contents of this collection use [`set_key_names`](Self::set_key_names).
    ///
    /// <p>The key pair names.</p>
    /// <p>Default: Describes all of your key pairs.</p>
    pub fn key_names(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.key_names.unwrap_or_default();
        v.push(input.into());
        self.key_names = Some(v);
        self
    }
    /// <p>The key pair names.</p>
    /// <p>Default: Describes all of your key pairs.</p>
    pub fn set_key_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.key_names = input;
        self
    }
    /// Appends an item to `key_pair_ids`.
    ///
    /// To override the contents of this collection use [`set_key_pair_ids`](Self::set_key_pair_ids).
    ///
    /// <p>The IDs of the key pairs.</p>
    pub fn key_pair_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.key_pair_ids.unwrap_or_default();
        v.push(input.into());
        self.key_pair_ids = Some(v);
        self
    }
    /// <p>The IDs of the key pairs.</p>
    pub fn set_key_pair_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.key_pair_ids = input;
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
    /// <p>If <code>true</code>, the public key material is included in the response.</p>
    /// <p>Default: <code>false</code> </p>
    pub fn include_public_key(mut self, input: bool) -> Self {
        self.include_public_key = Some(input);
        self
    }
    /// <p>If <code>true</code>, the public key material is included in the response.</p>
    /// <p>Default: <code>false</code> </p>
    pub fn set_include_public_key(mut self, input: std::option::Option<bool>) -> Self {
        self.include_public_key = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeKeyPairsInput`](crate::operation::describe_key_pairs::DescribeKeyPairsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_key_pairs::DescribeKeyPairsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_key_pairs::DescribeKeyPairsInput {
                filters: self.filters,
                key_names: self.key_names,
                key_pair_ids: self.key_pair_ids,
                dry_run: self.dry_run,
                include_public_key: self.include_public_key,
            },
        )
    }
}