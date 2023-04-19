// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TagInstanceProfile`](crate::operation::tag_instance_profile::builders::TagInstanceProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_profile_name(impl Into<String>)`](crate::operation::tag_instance_profile::builders::TagInstanceProfileFluentBuilder::instance_profile_name) / [`set_instance_profile_name(Option<String>)`](crate::operation::tag_instance_profile::builders::TagInstanceProfileFluentBuilder::set_instance_profile_name): <p>The name of the IAM instance profile to which you want to add tags.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::tag_instance_profile::builders::TagInstanceProfileFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::tag_instance_profile::builders::TagInstanceProfileFluentBuilder::set_tags): <p>The list of tags that you want to attach to the IAM instance profile. Each tag consists of a key name and an associated value.</p>
    /// - On success, responds with [`TagInstanceProfileOutput`](crate::operation::tag_instance_profile::TagInstanceProfileOutput)
    /// - On failure, responds with [`SdkError<TagInstanceProfileError>`](crate::operation::tag_instance_profile::TagInstanceProfileError)
    pub fn tag_instance_profile(
        &self,
    ) -> crate::operation::tag_instance_profile::builders::TagInstanceProfileFluentBuilder {
        crate::operation::tag_instance_profile::builders::TagInstanceProfileFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
