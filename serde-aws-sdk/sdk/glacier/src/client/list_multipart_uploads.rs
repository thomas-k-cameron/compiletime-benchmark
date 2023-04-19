// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListMultipartUploads`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    ///   - [`vault_name(impl Into<String>)`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::set_vault_name): <p>The name of the vault.</p>
    ///   - [`limit(i32)`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::set_limit): <p>Specifies the maximum number of uploads returned in the response body. If this value is not specified, the List Uploads operation returns up to 50 uploads.</p>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::set_marker): <p>An opaque string used for pagination. This value specifies the upload at which the listing of uploads should begin. Get the marker value from a previous List Uploads response. You need only include the marker if you are continuing the pagination of results started in a previous List Uploads request.</p>
    /// - On success, responds with [`ListMultipartUploadsOutput`](crate::operation::list_multipart_uploads::ListMultipartUploadsOutput) with field(s):
    ///   - [`uploads_list(Option<Vec<UploadListElement>>)`](crate::operation::list_multipart_uploads::ListMultipartUploadsOutput::uploads_list): <p>A list of in-progress multipart uploads.</p>
    ///   - [`marker(Option<String>)`](crate::operation::list_multipart_uploads::ListMultipartUploadsOutput::marker): <p>An opaque string that represents where to continue pagination of the results. You use the marker in a new List Multipart Uploads request to obtain more uploads in the list. If there are no more uploads, this value is <code>null</code>.</p>
    /// - On failure, responds with [`SdkError<ListMultipartUploadsError>`](crate::operation::list_multipart_uploads::ListMultipartUploadsError)
    pub fn list_multipart_uploads(
        &self,
    ) -> crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder {
        crate::operation::list_multipart_uploads::builders::ListMultipartUploadsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}