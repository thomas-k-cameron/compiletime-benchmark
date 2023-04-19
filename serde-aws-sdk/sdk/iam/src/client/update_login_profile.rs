// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateLoginProfile`](crate::operation::update_login_profile::builders::UpdateLoginProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::update_login_profile::builders::UpdateLoginProfileFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::update_login_profile::builders::UpdateLoginProfileFluentBuilder::set_user_name): <p>The name of the user whose password you want to update.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`password(impl Into<String>)`](crate::operation::update_login_profile::builders::UpdateLoginProfileFluentBuilder::password) / [`set_password(Option<String>)`](crate::operation::update_login_profile::builders::UpdateLoginProfileFluentBuilder::set_password): <p>The new password for the specified IAM user.</p>  <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>  <ul>   <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>   <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>   <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>  </ul>  <p>However, the format can be further restricted by the account administrator by setting a password policy on the Amazon Web Services account. For more information, see <code>UpdateAccountPasswordPolicy</code>.</p>
    ///   - [`password_reset_required(bool)`](crate::operation::update_login_profile::builders::UpdateLoginProfileFluentBuilder::password_reset_required) / [`set_password_reset_required(Option<bool>)`](crate::operation::update_login_profile::builders::UpdateLoginProfileFluentBuilder::set_password_reset_required): <p>Allows this new password to be used only once by requiring the specified IAM user to set a new password on next sign-in.</p>
    /// - On success, responds with [`UpdateLoginProfileOutput`](crate::operation::update_login_profile::UpdateLoginProfileOutput)
    /// - On failure, responds with [`SdkError<UpdateLoginProfileError>`](crate::operation::update_login_profile::UpdateLoginProfileError)
    pub fn update_login_profile(
        &self,
    ) -> crate::operation::update_login_profile::builders::UpdateLoginProfileFluentBuilder {
        crate::operation::update_login_profile::builders::UpdateLoginProfileFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
