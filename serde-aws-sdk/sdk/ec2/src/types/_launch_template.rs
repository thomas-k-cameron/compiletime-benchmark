// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a launch template.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct LaunchTemplate {
    /// <p>The ID of the launch template.</p>
    #[doc(hidden)]
    pub launch_template_id: std::option::Option<std::string::String>,
    /// <p>The name of the launch template.</p>
    #[doc(hidden)]
    pub launch_template_name: std::option::Option<std::string::String>,
    /// <p>The time launch template was created.</p>
    #[doc(hidden)]
    pub create_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The principal that created the launch template. </p>
    #[doc(hidden)]
    pub created_by: std::option::Option<std::string::String>,
    /// <p>The version number of the default version of the launch template.</p>
    #[doc(hidden)]
    pub default_version_number: std::option::Option<i64>,
    /// <p>The version number of the latest version of the launch template.</p>
    #[doc(hidden)]
    pub latest_version_number: std::option::Option<i64>,
    /// <p>The tags for the launch template.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl LaunchTemplate {
    /// <p>The ID of the launch template.</p>
    pub fn launch_template_id(&self) -> std::option::Option<&str> {
        self.launch_template_id.as_deref()
    }
    /// <p>The name of the launch template.</p>
    pub fn launch_template_name(&self) -> std::option::Option<&str> {
        self.launch_template_name.as_deref()
    }
    /// <p>The time launch template was created.</p>
    pub fn create_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.create_time.as_ref()
    }
    /// <p>The principal that created the launch template. </p>
    pub fn created_by(&self) -> std::option::Option<&str> {
        self.created_by.as_deref()
    }
    /// <p>The version number of the default version of the launch template.</p>
    pub fn default_version_number(&self) -> std::option::Option<i64> {
        self.default_version_number
    }
    /// <p>The version number of the latest version of the launch template.</p>
    pub fn latest_version_number(&self) -> std::option::Option<i64> {
        self.latest_version_number
    }
    /// <p>The tags for the launch template.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl LaunchTemplate {
    /// Creates a new builder-style object to manufacture [`LaunchTemplate`](crate::types::LaunchTemplate).
    pub fn builder() -> crate::types::builders::LaunchTemplateBuilder {
        crate::types::builders::LaunchTemplateBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LaunchTemplate;
/// A builder for [`LaunchTemplate`](crate::types::LaunchTemplate).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct LaunchTemplateBuilder {
    pub(crate) launch_template_id: std::option::Option<std::string::String>,
    pub(crate) launch_template_name: std::option::Option<std::string::String>,
    pub(crate) create_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) created_by: std::option::Option<std::string::String>,
    pub(crate) default_version_number: std::option::Option<i64>,
    pub(crate) latest_version_number: std::option::Option<i64>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl LaunchTemplateBuilder {
    /// <p>The ID of the launch template.</p>
    pub fn launch_template_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.launch_template_id = Some(input.into());
        self
    }
    /// <p>The ID of the launch template.</p>
    pub fn set_launch_template_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.launch_template_id = input;
        self
    }
    /// <p>The name of the launch template.</p>
    pub fn launch_template_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.launch_template_name = Some(input.into());
        self
    }
    /// <p>The name of the launch template.</p>
    pub fn set_launch_template_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.launch_template_name = input;
        self
    }
    /// <p>The time launch template was created.</p>
    pub fn create_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.create_time = Some(input);
        self
    }
    /// <p>The time launch template was created.</p>
    pub fn set_create_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_time = input;
        self
    }
    /// <p>The principal that created the launch template. </p>
    pub fn created_by(mut self, input: impl Into<std::string::String>) -> Self {
        self.created_by = Some(input.into());
        self
    }
    /// <p>The principal that created the launch template. </p>
    pub fn set_created_by(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.created_by = input;
        self
    }
    /// <p>The version number of the default version of the launch template.</p>
    pub fn default_version_number(mut self, input: i64) -> Self {
        self.default_version_number = Some(input);
        self
    }
    /// <p>The version number of the default version of the launch template.</p>
    pub fn set_default_version_number(mut self, input: std::option::Option<i64>) -> Self {
        self.default_version_number = input;
        self
    }
    /// <p>The version number of the latest version of the launch template.</p>
    pub fn latest_version_number(mut self, input: i64) -> Self {
        self.latest_version_number = Some(input);
        self
    }
    /// <p>The version number of the latest version of the launch template.</p>
    pub fn set_latest_version_number(mut self, input: std::option::Option<i64>) -> Self {
        self.latest_version_number = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the launch template.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags for the launch template.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplate`](crate::types::LaunchTemplate).
    pub fn build(self) -> crate::types::LaunchTemplate {
        crate::types::LaunchTemplate {
            launch_template_id: self.launch_template_id,
            launch_template_name: self.launch_template_name,
            create_time: self.create_time,
            created_by: self.created_by,
            default_version_number: self.default_version_number,
            latest_version_number: self.latest_version_number,
            tags: self.tags,
        }
    }
}