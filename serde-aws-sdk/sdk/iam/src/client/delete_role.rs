// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRole`](crate::operation::delete_role::builders::DeleteRoleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`role_name(impl Into<String>)`](crate::operation::delete_role::builders::DeleteRoleFluentBuilder::role_name) / [`set_role_name(Option<String>)`](crate::operation::delete_role::builders::DeleteRoleFluentBuilder::set_role_name): <p>The name of the role to delete.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    /// - On success, responds with [`DeleteRoleOutput`](crate::operation::delete_role::DeleteRoleOutput)
    /// - On failure, responds with [`SdkError<DeleteRoleError>`](crate::operation::delete_role::DeleteRoleError)
    pub fn delete_role(&self) -> crate::operation::delete_role::builders::DeleteRoleFluentBuilder {
        crate::operation::delete_role::builders::DeleteRoleFluentBuilder::new(self.handle.clone())
    }
}