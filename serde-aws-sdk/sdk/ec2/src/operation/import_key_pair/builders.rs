// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_key_pair::_import_key_pair_output::ImportKeyPairOutputBuilder;

pub use crate::operation::import_key_pair::_import_key_pair_input::ImportKeyPairInputBuilder;

/// Fluent builder constructing a request to `ImportKeyPair`.
///
/// <p>Imports the public key from an RSA or ED25519 key pair that you created with a third-party tool. Compare this with <code>CreateKeyPair</code>, in which Amazon Web Services creates the key pair and gives the keys to you (Amazon Web Services keeps a copy of the public key). With ImportKeyPair, you create the key pair and give Amazon Web Services just the public key. The private key is never transferred between you and Amazon Web Services.</p>
/// <p>For more information about key pairs, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html">Amazon EC2 key pairs</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ImportKeyPairFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_key_pair::builders::ImportKeyPairInputBuilder,
}
impl ImportKeyPairFluentBuilder {
    /// Creates a new `ImportKeyPair`.
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
            crate::operation::import_key_pair::ImportKeyPair,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::import_key_pair::ImportKeyPairError>,
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
        crate::operation::import_key_pair::ImportKeyPairOutput,
        aws_smithy_http::result::SdkError<crate::operation::import_key_pair::ImportKeyPairError>,
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
    ///     let deserialized_parameters: crate::operation::import_key_pair::builders::ImportKeyPairInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.import_key_pair().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::import_key_pair::builders::ImportKeyPairInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>A unique name for the key pair.</p>
    pub fn key_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.key_name(input.into());
        self
    }
    /// <p>A unique name for the key pair.</p>
    pub fn set_key_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_key_name(input);
        self
    }
    /// <p>The public key. For API calls, the text must be base64-encoded. For command line tools, base64 encoding is performed for you.</p>
    pub fn public_key_material(mut self, input: aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.public_key_material(input);
        self
    }
    /// <p>The public key. For API calls, the text must be base64-encoded. For command line tools, base64 encoding is performed for you.</p>
    pub fn set_public_key_material(
        mut self,
        input: std::option::Option<aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_public_key_material(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the imported key pair.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the imported key pair.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
}
