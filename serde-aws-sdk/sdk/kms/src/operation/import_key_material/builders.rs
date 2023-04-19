// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_key_material::_import_key_material_output::ImportKeyMaterialOutputBuilder;

pub use crate::operation::import_key_material::_import_key_material_input::ImportKeyMaterialInputBuilder;

/// Fluent builder constructing a request to `ImportKeyMaterial`.
///
/// <p>Imports key material into an existing symmetric encryption KMS key that was created without key material. After you successfully import key material into a KMS key, you can <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#reimport-key-material">reimport the same key material</a> into that KMS key, but you cannot import different key material. </p>
/// <p>You cannot perform this operation on an asymmetric KMS key, an HMAC KMS key, or on any KMS key in a different Amazon Web Services account. For more information about creating KMS keys with no key material and then importing key material, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p>Before using this operation, call <code>GetParametersForImport</code>. Its response includes a public key and an import token. Use the public key to encrypt the key material. Then, submit the import token from the same <code>GetParametersForImport</code> response.</p>
/// <p>When calling this operation, you must specify the following values:</p>
/// <ul>
/// <li> <p>The key ID or key ARN of a KMS key with no key material. Its <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>To create a KMS key with no key material, call <code>CreateKey</code> and set the value of its <code>Origin</code> parameter to <code>EXTERNAL</code>. To get the <code>Origin</code> of a KMS key, call <code>DescribeKey</code>.)</p> </li>
/// <li> <p>The encrypted key material. To get the public key to encrypt the key material, call <code>GetParametersForImport</code>.</p> </li>
/// <li> <p>The import token that <code>GetParametersForImport</code> returned. You must use a public key and token from the same <code>GetParametersForImport</code> response.</p> </li>
/// <li> <p>Whether the key material expires (<code>ExpirationModel</code>) and, if so, when (<code>ValidTo</code>). If you set an expiration date, on the specified date, KMS deletes the key material from the KMS key, making the KMS key unusable. To use the KMS key in cryptographic operations again, you must reimport the same key material. The only way to change the expiration model or expiration date is by reimporting the same key material and specifying a new expiration date. </p> </li>
/// </ul>
/// <p>When this operation is successful, the key state of the KMS key changes from <code>PendingImport</code> to <code>Enabled</code>, and you can use the KMS key.</p>
/// <p>If this operation fails, use the exception to help determine the problem. If the error is related to the key material, the import token, or wrapping key, use <code>GetParametersForImport</code> to get a new public key and import token for the KMS key and repeat the import procedure. For help, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#importing-keys-overview">How To Import Key Material</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p>The KMS key that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p> <b>Cross-account use</b>: No. You cannot perform this operation on a KMS key in a different Amazon Web Services account.</p>
/// <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:ImportKeyMaterial</a> (key policy)</p>
/// <p> <b>Related operations:</b> </p>
/// <ul>
/// <li> <p> <code>DeleteImportedKeyMaterial</code> </p> </li>
/// <li> <p> <code>GetParametersForImport</code> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ImportKeyMaterialFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_key_material::builders::ImportKeyMaterialInputBuilder,
}
impl ImportKeyMaterialFluentBuilder {
    /// Creates a new `ImportKeyMaterial`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::import_key_material::ImportKeyMaterial,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::import_key_material::ImportKeyMaterialError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::import_key_material::ImportKeyMaterialOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::import_key_material::ImportKeyMaterialError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::import_key_material::builders::ImportKeyMaterialInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.import_key_material().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::import_key_material::builders::ImportKeyMaterialInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The identifier of the symmetric encryption KMS key that receives the imported key material. This must be the same KMS key specified in the <code>KeyID</code> parameter of the corresponding <code>GetParametersForImport</code> request. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>. You cannot perform this operation on an asymmetric KMS key, an HMAC KMS key, a KMS key in a custom key store, or on a KMS key in a different Amazon Web Services account</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>The identifier of the symmetric encryption KMS key that receives the imported key material. This must be the same KMS key specified in the <code>KeyID</code> parameter of the corresponding <code>GetParametersForImport</code> request. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>. You cannot perform this operation on an asymmetric KMS key, an HMAC KMS key, a KMS key in a custom key store, or on a KMS key in a different Amazon Web Services account</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn set_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
    /// <p>The import token that you received in the response to a previous <code>GetParametersForImport</code> request. It must be from the same response that contained the public key that you used to encrypt the key material.</p>
    pub fn import_token(mut self, input: aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.import_token(input);
        self
    }
    /// <p>The import token that you received in the response to a previous <code>GetParametersForImport</code> request. It must be from the same response that contained the public key that you used to encrypt the key material.</p>
    pub fn set_import_token(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_import_token(input);
        self
    }
    /// <p>The encrypted key material to import. The key material must be encrypted with the public wrapping key that <code>GetParametersForImport</code> returned, using the wrapping algorithm that you specified in the same <code>GetParametersForImport</code> request.</p>
    pub fn encrypted_key_material(mut self, input: aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.encrypted_key_material(input);
        self
    }
    /// <p>The encrypted key material to import. The key material must be encrypted with the public wrapping key that <code>GetParametersForImport</code> returned, using the wrapping algorithm that you specified in the same <code>GetParametersForImport</code> request.</p>
    pub fn set_encrypted_key_material(
        mut self,
        input: std::option::Option<aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_encrypted_key_material(input);
        self
    }
    /// <p>The date and time when the imported key material expires. This parameter is required when the value of the <code>ExpirationModel</code> parameter is <code>KEY_MATERIAL_EXPIRES</code>. Otherwise it is not valid.</p>
    /// <p>The value of this parameter must be a future date and time. The maximum value is 365 days from the request date.</p>
    /// <p>When the key material expires, KMS deletes the key material from the KMS key. Without its key material, the KMS key is unusable. To use the KMS key in cryptographic operations, you must reimport the same key material.</p>
    /// <p>You cannot change the <code>ExpirationModel</code> or <code>ValidTo</code> values for the current import after the request completes. To change either value, you must delete (<code>DeleteImportedKeyMaterial</code>) and reimport the key material.</p>
    pub fn valid_to(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.valid_to(input);
        self
    }
    /// <p>The date and time when the imported key material expires. This parameter is required when the value of the <code>ExpirationModel</code> parameter is <code>KEY_MATERIAL_EXPIRES</code>. Otherwise it is not valid.</p>
    /// <p>The value of this parameter must be a future date and time. The maximum value is 365 days from the request date.</p>
    /// <p>When the key material expires, KMS deletes the key material from the KMS key. Without its key material, the KMS key is unusable. To use the KMS key in cryptographic operations, you must reimport the same key material.</p>
    /// <p>You cannot change the <code>ExpirationModel</code> or <code>ValidTo</code> values for the current import after the request completes. To change either value, you must delete (<code>DeleteImportedKeyMaterial</code>) and reimport the key material.</p>
    pub fn set_valid_to(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_valid_to(input);
        self
    }
    /// <p>Specifies whether the key material expires. The default is <code>KEY_MATERIAL_EXPIRES</code>.</p>
    /// <p>When the value of <code>ExpirationModel</code> is <code>KEY_MATERIAL_EXPIRES</code>, you must specify a value for the <code>ValidTo</code> parameter. When value is <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>, you must omit the <code>ValidTo</code> parameter.</p>
    /// <p>You cannot change the <code>ExpirationModel</code> or <code>ValidTo</code> values for the current import after the request completes. To change either value, you must delete (<code>DeleteImportedKeyMaterial</code>) and reimport the key material.</p>
    pub fn expiration_model(mut self, input: crate::types::ExpirationModelType) -> Self {
        self.inner = self.inner.expiration_model(input);
        self
    }
    /// <p>Specifies whether the key material expires. The default is <code>KEY_MATERIAL_EXPIRES</code>.</p>
    /// <p>When the value of <code>ExpirationModel</code> is <code>KEY_MATERIAL_EXPIRES</code>, you must specify a value for the <code>ValidTo</code> parameter. When value is <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>, you must omit the <code>ValidTo</code> parameter.</p>
    /// <p>You cannot change the <code>ExpirationModel</code> or <code>ValidTo</code> values for the current import after the request completes. To change either value, you must delete (<code>DeleteImportedKeyMaterial</code>) and reimport the key material.</p>
    pub fn set_expiration_model(
        mut self,
        input: std::option::Option<crate::types::ExpirationModelType>,
    ) -> Self {
        self.inner = self.inner.set_expiration_model(input);
        self
    }
}
