// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GenerateDataKey`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_key_id): <p>Specifies the symmetric encryption KMS key that encrypts the data key. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p>  <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>   <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    ///   - [`encryption_context(HashMap<String, String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::encryption_context) / [`set_encryption_context(Option<HashMap<String, String>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_encryption_context): <p>Specifies the encryption context that will be used when encrypting the data key.</p>  <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p>
    ///   - [`number_of_bytes(i32)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::number_of_bytes) / [`set_number_of_bytes(Option<i32>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_number_of_bytes): <p>Specifies the length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For 128-bit (16-byte) and 256-bit (32-byte) data keys, use the <code>KeySpec</code> parameter.</p>  <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    ///   - [`key_spec(DataKeySpec)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::key_spec) / [`set_key_spec(Option<DataKeySpec>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_key_spec): <p>Specifies the length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>  <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    ///   - [`grant_tokens(Vec<String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec<String>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_grant_tokens): <p>A list of grant tokens.</p>  <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// - On success, responds with [`GenerateDataKeyOutput`](crate::operation::generate_data_key::GenerateDataKeyOutput) with field(s):
    ///   - [`ciphertext_blob(Option<Blob>)`](crate::operation::generate_data_key::GenerateDataKeyOutput::ciphertext_blob): <p>The encrypted copy of the data key. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    ///   - [`plaintext(Option<Blob>)`](crate::operation::generate_data_key::GenerateDataKeyOutput::plaintext): <p>The plaintext data key. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded. Use this data key to encrypt your data outside of KMS. Then, remove it from memory as soon as possible.</p>
    ///   - [`key_id(Option<String>)`](crate::operation::generate_data_key::GenerateDataKeyOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key that encrypted the data key.</p>
    /// - On failure, responds with [`SdkError<GenerateDataKeyError>`](crate::operation::generate_data_key::GenerateDataKeyError)
    pub fn generate_data_key(
        &self,
    ) -> crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder {
        crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
