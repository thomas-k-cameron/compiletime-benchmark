// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GenerateDataKeyPair`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`encryption_context(HashMap<String, String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::encryption_context) / [`set_encryption_context(Option<HashMap<String, String>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_encryption_context): <p>Specifies the encryption context that will be used when encrypting the private key in the data key pair.</p>  <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p>
    ///   - [`key_id(impl Into<String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_key_id): <p>Specifies the symmetric encryption KMS key that encrypts the private key in the data key pair. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p>  <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>   <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    ///   - [`key_pair_spec(DataKeyPairSpec)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::key_pair_spec) / [`set_key_pair_spec(Option<DataKeyPairSpec>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_key_pair_spec): <p>Determines the type of data key pair that is generated. </p>  <p>The KMS rule that restricts the use of asymmetric RSA and SM2 KMS keys to encrypt and decrypt or to sign and verify (but not both), and the rule that permits you to use ECC KMS keys only to sign and verify, are not effective on data key pairs, which are used outside of KMS. The SM2 key spec is only available in China Regions.</p>
    ///   - [`grant_tokens(Vec<String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec<String>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_grant_tokens): <p>A list of grant tokens.</p>  <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// - On success, responds with [`GenerateDataKeyPairOutput`](crate::operation::generate_data_key_pair::GenerateDataKeyPairOutput) with field(s):
    ///   - [`private_key_ciphertext_blob(Option<Blob>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairOutput::private_key_ciphertext_blob): <p>The encrypted copy of the private key. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    ///   - [`private_key_plaintext(Option<Blob>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairOutput::private_key_plaintext): <p>The plaintext copy of the private key. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    ///   - [`public_key(Option<Blob>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairOutput::public_key): <p>The public key (in plaintext). When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    ///   - [`key_id(Option<String>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key that encrypted the private key.</p>
    ///   - [`key_pair_spec(Option<DataKeyPairSpec>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairOutput::key_pair_spec): <p>The type of data key pair that was generated.</p>
    /// - On failure, responds with [`SdkError<GenerateDataKeyPairError>`](crate::operation::generate_data_key_pair::GenerateDataKeyPairError)
    pub fn generate_data_key_pair(
        &self,
    ) -> crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder {
        crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
