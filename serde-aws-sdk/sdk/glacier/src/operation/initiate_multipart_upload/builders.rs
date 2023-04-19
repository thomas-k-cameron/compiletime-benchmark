// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::initiate_multipart_upload::_initiate_multipart_upload_output::InitiateMultipartUploadOutputBuilder;

pub use crate::operation::initiate_multipart_upload::_initiate_multipart_upload_input::InitiateMultipartUploadInputBuilder;

/// Fluent builder constructing a request to `InitiateMultipartUpload`.
///
/// <p>This operation initiates a multipart upload. Amazon S3 Glacier creates a multipart upload resource and returns its ID in the response. The multipart upload ID is used in subsequent requests to upload parts of an archive (see <code>UploadMultipartPart</code>).</p>
/// <p>When you initiate a multipart upload, you specify the part size in number of bytes. The part size must be a megabyte (1024 KB) multiplied by a power of 2-for example, 1048576 (1 MB), 2097152 (2 MB), 4194304 (4 MB), 8388608 (8 MB), and so on. The minimum allowable part size is 1 MB, and the maximum is 4 GB.</p>
/// <p>Every part you upload to this resource (see <code>UploadMultipartPart</code>), except the last one, must have the same size. The last one can be the same size or smaller. For example, suppose you want to upload a 16.2 MB file. If you initiate the multipart upload with a part size of 4 MB, you will upload four parts of 4 MB each and one part of 0.2 MB. </p> <note>
/// <p>You don't need to know the size of the archive when you start a multipart upload because Amazon S3 Glacier does not require you to specify the overall archive size.</p>
/// </note>
/// <p>After you complete the multipart upload, Amazon S3 Glacier (Glacier) removes the multipart upload resource referenced by the ID. Glacier also removes the multipart upload resource if you cancel the multipart upload or it may be removed if there is no activity for a period of 24 hours.</p>
/// <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p>
/// <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-initiate-upload.html">Initiate Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct InitiateMultipartUploadFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::initiate_multipart_upload::builders::InitiateMultipartUploadInputBuilder,
}
impl InitiateMultipartUploadFluentBuilder {
    /// Creates a new `InitiateMultipartUpload`.
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
            crate::operation::initiate_multipart_upload::InitiateMultipartUpload,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::initiate_multipart_upload::InitiateMultipartUploadError,
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
        crate::operation::initiate_multipart_upload::InitiateMultipartUploadOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::initiate_multipart_upload::InitiateMultipartUploadError,
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
    ///     let deserialized_parameters: crate::operation::initiate_multipart_upload::builders::InitiateMultipartUploadInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.initiate_multipart_upload().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::initiate_multipart_upload::builders::InitiateMultipartUploadInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vault_name(input.into());
        self
    }
    /// <p>The name of the vault.</p>
    pub fn set_vault_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_vault_name(input);
        self
    }
    /// <p>The archive description that you are uploading in parts.</p>
    /// <p>The part size must be a megabyte (1024 KB) multiplied by a power of 2, for example 1048576 (1 MB), 2097152 (2 MB), 4194304 (4 MB), 8388608 (8 MB), and so on. The minimum allowable part size is 1 MB, and the maximum is 4 GB (4096 MB).</p>
    pub fn archive_description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.archive_description(input.into());
        self
    }
    /// <p>The archive description that you are uploading in parts.</p>
    /// <p>The part size must be a megabyte (1024 KB) multiplied by a power of 2, for example 1048576 (1 MB), 2097152 (2 MB), 4194304 (4 MB), 8388608 (8 MB), and so on. The minimum allowable part size is 1 MB, and the maximum is 4 GB (4096 MB).</p>
    pub fn set_archive_description(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_archive_description(input);
        self
    }
    /// <p>The size of each part except the last, in bytes. The last part can be smaller than this part size.</p>
    pub fn part_size(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.part_size(input.into());
        self
    }
    /// <p>The size of each part except the last, in bytes. The last part can be smaller than this part size.</p>
    pub fn set_part_size(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_part_size(input);
        self
    }
}
