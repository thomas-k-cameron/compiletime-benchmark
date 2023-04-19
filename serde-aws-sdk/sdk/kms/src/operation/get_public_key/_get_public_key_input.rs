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
pub struct GetPublicKeyInput {
    /// <p>Identifies the asymmetric KMS key that includes the public key.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    #[doc(hidden)]
    pub key_id: std::option::Option<std::string::String>,
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    #[doc(hidden)]
    pub grant_tokens: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GetPublicKeyInput {
    /// <p>Identifies the asymmetric KMS key that includes the public key.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    pub fn key_id(&self) -> std::option::Option<&str> {
        self.key_id.as_deref()
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_tokens(&self) -> std::option::Option<&[std::string::String]> {
        self.grant_tokens.as_deref()
    }
}
impl GetPublicKeyInput {
    /// Creates a new builder-style object to manufacture [`GetPublicKeyInput`](crate::operation::get_public_key::GetPublicKeyInput).
    pub fn builder() -> crate::operation::get_public_key::builders::GetPublicKeyInputBuilder {
        crate::operation::get_public_key::builders::GetPublicKeyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_public_key::GetPublicKeyInput;
/// A builder for [`GetPublicKeyInput`](crate::operation::get_public_key::GetPublicKeyInput).
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
pub struct GetPublicKeyInputBuilder {
    pub(crate) key_id: std::option::Option<std::string::String>,
    pub(crate) grant_tokens: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GetPublicKeyInputBuilder {
    /// <p>Identifies the asymmetric KMS key that includes the public key.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    pub fn key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.key_id = Some(input.into());
        self
    }
    /// <p>Identifies the asymmetric KMS key that includes the public key.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    pub fn set_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// Appends an item to `grant_tokens`.
    ///
    /// To override the contents of this collection use [`set_grant_tokens`](Self::set_grant_tokens).
    ///
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_tokens(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.grant_tokens.unwrap_or_default();
        v.push(input.into());
        self.grant_tokens = Some(v);
        self
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn set_grant_tokens(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.grant_tokens = input;
        self
    }
    /// Consumes the builder and constructs a [`GetPublicKeyInput`](crate::operation::get_public_key::GetPublicKeyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_public_key::GetPublicKeyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_public_key::GetPublicKeyInput {
            key_id: self.key_id,
            grant_tokens: self.grant_tokens,
        })
    }
}