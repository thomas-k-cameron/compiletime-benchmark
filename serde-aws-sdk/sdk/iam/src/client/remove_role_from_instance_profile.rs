// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RemoveRoleFromInstanceProfile`](crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_profile_name(impl Into<String>)`](crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileFluentBuilder::instance_profile_name) / [`set_instance_profile_name(Option<String>)`](crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileFluentBuilder::set_instance_profile_name): <p>The name of the instance profile to update.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`role_name(impl Into<String>)`](crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileFluentBuilder::role_name) / [`set_role_name(Option<String>)`](crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileFluentBuilder::set_role_name): <p>The name of the role to remove.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    /// - On success, responds with [`RemoveRoleFromInstanceProfileOutput`](crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileOutput)
    /// - On failure, responds with [`SdkError<RemoveRoleFromInstanceProfileError>`](crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError)
    pub fn remove_role_from_instance_profile(&self) -> crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileFluentBuilder{
        crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileFluentBuilder::new(self.handle.clone())
    }
}