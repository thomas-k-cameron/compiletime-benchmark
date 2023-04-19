// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAccessKey`](crate::operation::update_access_key::builders::UpdateAccessKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::update_access_key::builders::UpdateAccessKeyFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::update_access_key::builders::UpdateAccessKeyFluentBuilder::set_user_name): <p>The name of the user whose key you want to update.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`access_key_id(impl Into<String>)`](crate::operation::update_access_key::builders::UpdateAccessKeyFluentBuilder::access_key_id) / [`set_access_key_id(Option<String>)`](crate::operation::update_access_key::builders::UpdateAccessKeyFluentBuilder::set_access_key_id): <p>The access key ID of the secret access key you want to update.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
    ///   - [`status(StatusType)`](crate::operation::update_access_key::builders::UpdateAccessKeyFluentBuilder::status) / [`set_status(Option<StatusType>)`](crate::operation::update_access_key::builders::UpdateAccessKeyFluentBuilder::set_status): <p> The status you want to assign to the secret access key. <code>Active</code> means that the key can be used for programmatic calls to Amazon Web Services, while <code>Inactive</code> means that the key cannot be used.</p>
    /// - On success, responds with [`UpdateAccessKeyOutput`](crate::operation::update_access_key::UpdateAccessKeyOutput)
    /// - On failure, responds with [`SdkError<UpdateAccessKeyError>`](crate::operation::update_access_key::UpdateAccessKeyError)
    pub fn update_access_key(
        &self,
    ) -> crate::operation::update_access_key::builders::UpdateAccessKeyFluentBuilder {
        crate::operation::update_access_key::builders::UpdateAccessKeyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}