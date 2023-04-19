// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_group::_update_group_output::UpdateGroupOutputBuilder;

pub use crate::operation::update_group::_update_group_input::UpdateGroupInputBuilder;

/// Fluent builder constructing a request to `UpdateGroup`.
///
/// <p>Updates the name and/or the path of the specified IAM group.</p> <important>
/// <p> You should understand the implications of changing a group's path or name. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_WorkingWithGroupsAndUsers.html">Renaming users and groups</a> in the <i>IAM User Guide</i>.</p>
/// </important> <note>
/// <p>The person making the request (the principal), must have permission to change the role group with the old name and the new name. For example, to change the group named <code>Managers</code> to <code>MGRs</code>, the principal must have a policy that allows them to update both groups. If the principal has permission to update the <code>Managers</code> group, but not the <code>MGRs</code> group, then the update fails. For more information about permissions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access management</a>. </p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateGroupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_group::builders::UpdateGroupInputBuilder,
}
impl UpdateGroupFluentBuilder {
    /// Creates a new `UpdateGroup`.
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
            crate::operation::update_group::UpdateGroup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_group::UpdateGroupError>,
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
        crate::operation::update_group::UpdateGroupOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_group::UpdateGroupError>,
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
    ///     let deserialized_parameters: crate::operation::update_group::builders::UpdateGroupInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.update_group().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::update_group::builders::UpdateGroupInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Name of the IAM group to update. If you're changing the name of the group, this is the original name.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.group_name(input.into());
        self
    }
    /// <p>Name of the IAM group to update. If you're changing the name of the group, this is the original name.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_group_name(input);
        self
    }
    /// <p>New path for the IAM group. Only include this if changing the group's path.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of either a forward slash (/) by itself or a string that must begin and end with forward slashes. In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including most punctuation characters, digits, and upper and lowercased letters.</p>
    pub fn new_path(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.new_path(input.into());
        self
    }
    /// <p>New path for the IAM group. Only include this if changing the group's path.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of either a forward slash (/) by itself or a string that must begin and end with forward slashes. In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including most punctuation characters, digits, and upper and lowercased letters.</p>
    pub fn set_new_path(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_new_path(input);
        self
    }
    /// <p>New name for the IAM group. Only include this if changing the group's name.</p>
    /// <p>IAM user, group, role, and policy names must be unique within the account. Names are not distinguished by case. For example, you cannot create resources named both "MyResource" and "myresource".</p>
    pub fn new_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.new_group_name(input.into());
        self
    }
    /// <p>New name for the IAM group. Only include this if changing the group's name.</p>
    /// <p>IAM user, group, role, and policy names must be unique within the account. Names are not distinguished by case. For example, you cannot create resources named both "MyResource" and "myresource".</p>
    pub fn set_new_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_new_group_name(input);
        self
    }
}
