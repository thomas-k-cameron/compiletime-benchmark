// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSigningCertificate`](crate::operation::update_signing_certificate::builders::UpdateSigningCertificateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::update_signing_certificate::builders::UpdateSigningCertificateFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::update_signing_certificate::builders::UpdateSigningCertificateFluentBuilder::set_user_name): <p>The name of the IAM user the signing certificate belongs to.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`certificate_id(impl Into<String>)`](crate::operation::update_signing_certificate::builders::UpdateSigningCertificateFluentBuilder::certificate_id) / [`set_certificate_id(Option<String>)`](crate::operation::update_signing_certificate::builders::UpdateSigningCertificateFluentBuilder::set_certificate_id): <p>The ID of the signing certificate you want to update.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
    ///   - [`status(StatusType)`](crate::operation::update_signing_certificate::builders::UpdateSigningCertificateFluentBuilder::status) / [`set_status(Option<StatusType>)`](crate::operation::update_signing_certificate::builders::UpdateSigningCertificateFluentBuilder::set_status): <p> The status you want to assign to the certificate. <code>Active</code> means that the certificate can be used for programmatic calls to Amazon Web Services <code>Inactive</code> means that the certificate cannot be used.</p>
    /// - On success, responds with [`UpdateSigningCertificateOutput`](crate::operation::update_signing_certificate::UpdateSigningCertificateOutput)
    /// - On failure, responds with [`SdkError<UpdateSigningCertificateError>`](crate::operation::update_signing_certificate::UpdateSigningCertificateError)
    pub fn update_signing_certificate(
        &self,
    ) -> crate::operation::update_signing_certificate::builders::UpdateSigningCertificateFluentBuilder
    {
        crate::operation::update_signing_certificate::builders::UpdateSigningCertificateFluentBuilder::new(self.handle.clone())
    }
}
