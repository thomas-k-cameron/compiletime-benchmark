// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeKey`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::set_key_id): <p>Describes the specified KMS key. </p>  <p>If you specify a predefined Amazon Web Services alias (an Amazon Web Services alias with no key ID), KMS associates the alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html##aws-managed-cmk">Amazon Web Services managed key</a> and returns its <code>KeyId</code> and <code>Arn</code> in the response.</p>  <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>   <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    ///   - [`grant_tokens(Vec<String>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec<String>>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::set_grant_tokens): <p>A list of grant tokens.</p>  <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// - On success, responds with [`DescribeKeyOutput`](crate::operation::describe_key::DescribeKeyOutput) with field(s):
    ///   - [`key_metadata(Option<KeyMetadata>)`](crate::operation::describe_key::DescribeKeyOutput::key_metadata): <p>Metadata associated with the key.</p>
    /// - On failure, responds with [`SdkError<DescribeKeyError>`](crate::operation::describe_key::DescribeKeyError)
    pub fn describe_key(
        &self,
    ) -> crate::operation::describe_key::builders::DescribeKeyFluentBuilder {
        crate::operation::describe_key::builders::DescribeKeyFluentBuilder::new(self.handle.clone())
    }
}
