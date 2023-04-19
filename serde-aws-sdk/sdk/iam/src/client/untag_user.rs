// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UntagUser`](crate::operation::untag_user::builders::UntagUserFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::untag_user::builders::UntagUserFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::untag_user::builders::UntagUserFluentBuilder::set_user_name): <p>The name of the IAM user from which you want to remove tags.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`tag_keys(Vec<String>)`](crate::operation::untag_user::builders::UntagUserFluentBuilder::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::operation::untag_user::builders::UntagUserFluentBuilder::set_tag_keys): <p>A list of key names as a simple array of strings. The tags with matching keys are removed from the specified user.</p>
    /// - On success, responds with [`UntagUserOutput`](crate::operation::untag_user::UntagUserOutput)
    /// - On failure, responds with [`SdkError<UntagUserError>`](crate::operation::untag_user::UntagUserError)
    pub fn untag_user(&self) -> crate::operation::untag_user::builders::UntagUserFluentBuilder {
        crate::operation::untag_user::builders::UntagUserFluentBuilder::new(self.handle.clone())
    }
}