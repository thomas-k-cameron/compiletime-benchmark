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
pub struct ListGrantsInput {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p>
    /// <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[doc(hidden)]
    pub limit: std::option::Option<i32>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
    /// <p>Returns only grants for the specified KMS key. This parameter is required.</p>
    /// <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    #[doc(hidden)]
    pub key_id: std::option::Option<std::string::String>,
    /// <p>Returns only the grant with the specified grant ID. The grant ID uniquely identifies the grant. </p>
    #[doc(hidden)]
    pub grant_id: std::option::Option<std::string::String>,
    /// <p>Returns only grants where the specified principal is the grantee principal for the grant.</p>
    #[doc(hidden)]
    pub grantee_principal: std::option::Option<std::string::String>,
}
impl ListGrantsInput {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p>
    /// <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    pub fn limit(&self) -> std::option::Option<i32> {
        self.limit
    }
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    pub fn marker(&self) -> std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>Returns only grants for the specified KMS key. This parameter is required.</p>
    /// <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn key_id(&self) -> std::option::Option<&str> {
        self.key_id.as_deref()
    }
    /// <p>Returns only the grant with the specified grant ID. The grant ID uniquely identifies the grant. </p>
    pub fn grant_id(&self) -> std::option::Option<&str> {
        self.grant_id.as_deref()
    }
    /// <p>Returns only grants where the specified principal is the grantee principal for the grant.</p>
    pub fn grantee_principal(&self) -> std::option::Option<&str> {
        self.grantee_principal.as_deref()
    }
}
impl ListGrantsInput {
    /// Creates a new builder-style object to manufacture [`ListGrantsInput`](crate::operation::list_grants::ListGrantsInput).
    pub fn builder() -> crate::operation::list_grants::builders::ListGrantsInputBuilder {
        crate::operation::list_grants::builders::ListGrantsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_grants::ListGrantsInput;
/// A builder for [`ListGrantsInput`](crate::operation::list_grants::ListGrantsInput).
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
pub struct ListGrantsInputBuilder {
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) marker: std::option::Option<std::string::String>,
    pub(crate) key_id: std::option::Option<std::string::String>,
    pub(crate) grant_id: std::option::Option<std::string::String>,
    pub(crate) grantee_principal: std::option::Option<std::string::String>,
}
impl ListGrantsInputBuilder {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p>
    /// <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p>
    /// <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.marker = Some(input.into());
        self
    }
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>Returns only grants for the specified KMS key. This parameter is required.</p>
    /// <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.key_id = Some(input.into());
        self
    }
    /// <p>Returns only grants for the specified KMS key. This parameter is required.</p>
    /// <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn set_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// <p>Returns only the grant with the specified grant ID. The grant ID uniquely identifies the grant. </p>
    pub fn grant_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.grant_id = Some(input.into());
        self
    }
    /// <p>Returns only the grant with the specified grant ID. The grant ID uniquely identifies the grant. </p>
    pub fn set_grant_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.grant_id = input;
        self
    }
    /// <p>Returns only grants where the specified principal is the grantee principal for the grant.</p>
    pub fn grantee_principal(mut self, input: impl Into<std::string::String>) -> Self {
        self.grantee_principal = Some(input.into());
        self
    }
    /// <p>Returns only grants where the specified principal is the grantee principal for the grant.</p>
    pub fn set_grantee_principal(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.grantee_principal = input;
        self
    }
    /// Consumes the builder and constructs a [`ListGrantsInput`](crate::operation::list_grants::ListGrantsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_grants::ListGrantsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::list_grants::ListGrantsInput {
            limit: self.limit,
            marker: self.marker,
            key_id: self.key_id,
            grant_id: self.grant_id,
            grantee_principal: self.grantee_principal,
        })
    }
}