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
pub struct CreateAliasInput {
    /// <p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. </p>
    /// <p>The <code>AliasName</code> value must be string of 1-256 characters. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">Amazon Web Services managed keys</a>.</p>
    #[doc(hidden)]
    pub alias_name: std::option::Option<std::string::String>,
    /// <p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed key</a>. The KMS key must be in the same Amazon Web Services Region. </p>
    /// <p>A valid key ID is required. If you supply a null or empty string value, this operation returns an error.</p>
    /// <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html#find-cmk-id-arn">Finding the Key ID and ARN</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    #[doc(hidden)]
    pub target_key_id: std::option::Option<std::string::String>,
}
impl CreateAliasInput {
    /// <p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. </p>
    /// <p>The <code>AliasName</code> value must be string of 1-256 characters. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">Amazon Web Services managed keys</a>.</p>
    pub fn alias_name(&self) -> std::option::Option<&str> {
        self.alias_name.as_deref()
    }
    /// <p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed key</a>. The KMS key must be in the same Amazon Web Services Region. </p>
    /// <p>A valid key ID is required. If you supply a null or empty string value, this operation returns an error.</p>
    /// <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html#find-cmk-id-arn">Finding the Key ID and ARN</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn target_key_id(&self) -> std::option::Option<&str> {
        self.target_key_id.as_deref()
    }
}
impl CreateAliasInput {
    /// Creates a new builder-style object to manufacture [`CreateAliasInput`](crate::operation::create_alias::CreateAliasInput).
    pub fn builder() -> crate::operation::create_alias::builders::CreateAliasInputBuilder {
        crate::operation::create_alias::builders::CreateAliasInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_alias::CreateAliasInput;
/// A builder for [`CreateAliasInput`](crate::operation::create_alias::CreateAliasInput).
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
pub struct CreateAliasInputBuilder {
    pub(crate) alias_name: std::option::Option<std::string::String>,
    pub(crate) target_key_id: std::option::Option<std::string::String>,
}
impl CreateAliasInputBuilder {
    /// <p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. </p>
    /// <p>The <code>AliasName</code> value must be string of 1-256 characters. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">Amazon Web Services managed keys</a>.</p>
    pub fn alias_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.alias_name = Some(input.into());
        self
    }
    /// <p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. </p>
    /// <p>The <code>AliasName</code> value must be string of 1-256 characters. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">Amazon Web Services managed keys</a>.</p>
    pub fn set_alias_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.alias_name = input;
        self
    }
    /// <p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed key</a>. The KMS key must be in the same Amazon Web Services Region. </p>
    /// <p>A valid key ID is required. If you supply a null or empty string value, this operation returns an error.</p>
    /// <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html#find-cmk-id-arn">Finding the Key ID and ARN</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn target_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.target_key_id = Some(input.into());
        self
    }
    /// <p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed key</a>. The KMS key must be in the same Amazon Web Services Region. </p>
    /// <p>A valid key ID is required. If you supply a null or empty string value, this operation returns an error.</p>
    /// <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html#find-cmk-id-arn">Finding the Key ID and ARN</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn set_target_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.target_key_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateAliasInput`](crate::operation::create_alias::CreateAliasInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_alias::CreateAliasInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::create_alias::CreateAliasInput {
            alias_name: self.alias_name,
            target_key_id: self.target_key_id,
        })
    }
}
