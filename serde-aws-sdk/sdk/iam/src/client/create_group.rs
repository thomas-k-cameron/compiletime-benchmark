// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateGroup`](crate::operation::create_group::builders::CreateGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`path(impl Into<String>)`](crate::operation::create_group::builders::CreateGroupFluentBuilder::path) / [`set_path(Option<String>)`](crate::operation::create_group::builders::CreateGroupFluentBuilder::set_path): <p> The path to the group. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>  <p>This parameter is optional. If it is not included, it defaults to a slash (/).</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of either a forward slash (/) by itself or a string that must begin and end with forward slashes. In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including most punctuation characters, digits, and upper and lowercased letters.</p>
    ///   - [`group_name(impl Into<String>)`](crate::operation::create_group::builders::CreateGroupFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::create_group::builders::CreateGroupFluentBuilder::set_group_name): <p>The name of the group to create. Do not include the path in this value.</p>  <p>IAM user, group, role, and policy names must be unique within the account. Names are not distinguished by case. For example, you cannot create resources named both "MyResource" and "myresource".</p>
    /// - On success, responds with [`CreateGroupOutput`](crate::operation::create_group::CreateGroupOutput) with field(s):
    ///   - [`group(Option<Group>)`](crate::operation::create_group::CreateGroupOutput::group): <p>A structure containing details about the new group.</p>
    /// - On failure, responds with [`SdkError<CreateGroupError>`](crate::operation::create_group::CreateGroupError)
    pub fn create_group(
        &self,
    ) -> crate::operation::create_group::builders::CreateGroupFluentBuilder {
        crate::operation::create_group::builders::CreateGroupFluentBuilder::new(self.handle.clone())
    }
}
