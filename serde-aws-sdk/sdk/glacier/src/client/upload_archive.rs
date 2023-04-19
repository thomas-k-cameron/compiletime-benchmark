// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UploadArchive`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vault_name(impl Into<String>)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::set_vault_name): <p>The name of the vault.</p>
    ///   - [`account_id(impl Into<String>)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    ///   - [`archive_description(impl Into<String>)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::archive_description) / [`set_archive_description(Option<String>)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::set_archive_description): <p>The optional description of the archive you are uploading.</p>
    ///   - [`checksum(impl Into<String>)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::checksum) / [`set_checksum(Option<String>)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::set_checksum): <p>The SHA256 tree hash of the data being uploaded.</p>
    ///   - [`body(ByteStream)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::body) / [`set_body(ByteStream)`](crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::set_body): <p>The data to upload.</p>
    /// - On success, responds with [`UploadArchiveOutput`](crate::operation::upload_archive::UploadArchiveOutput) with field(s):
    ///   - [`location(Option<String>)`](crate::operation::upload_archive::UploadArchiveOutput::location): <p>The relative URI path of the newly added archive resource.</p>
    ///   - [`checksum(Option<String>)`](crate::operation::upload_archive::UploadArchiveOutput::checksum): <p>The checksum of the archive computed by Amazon S3 Glacier.</p>
    ///   - [`archive_id(Option<String>)`](crate::operation::upload_archive::UploadArchiveOutput::archive_id): <p>The ID of the archive. This value is also included as part of the location.</p>
    /// - On failure, responds with [`SdkError<UploadArchiveError>`](crate::operation::upload_archive::UploadArchiveError)
    pub fn upload_archive(
        &self,
    ) -> crate::operation::upload_archive::builders::UploadArchiveFluentBuilder {
        crate::operation::upload_archive::builders::UploadArchiveFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
