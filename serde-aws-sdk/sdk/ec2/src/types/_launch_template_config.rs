// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a launch template and overrides.</p>
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
pub struct LaunchTemplateConfig {
    /// <p>The launch template.</p>
    #[doc(hidden)]
    pub launch_template_specification:
        std::option::Option<crate::types::FleetLaunchTemplateSpecification>,
    /// <p>Any parameters that you specify override the same parameters in the launch template.</p>
    #[doc(hidden)]
    pub overrides: std::option::Option<std::vec::Vec<crate::types::LaunchTemplateOverrides>>,
}
impl LaunchTemplateConfig {
    /// <p>The launch template.</p>
    pub fn launch_template_specification(
        &self,
    ) -> std::option::Option<&crate::types::FleetLaunchTemplateSpecification> {
        self.launch_template_specification.as_ref()
    }
    /// <p>Any parameters that you specify override the same parameters in the launch template.</p>
    pub fn overrides(&self) -> std::option::Option<&[crate::types::LaunchTemplateOverrides]> {
        self.overrides.as_deref()
    }
}
impl LaunchTemplateConfig {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateConfig`](crate::types::LaunchTemplateConfig).
    pub fn builder() -> crate::types::builders::LaunchTemplateConfigBuilder {
        crate::types::builders::LaunchTemplateConfigBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LaunchTemplateConfig;
/// A builder for [`LaunchTemplateConfig`](crate::types::LaunchTemplateConfig).
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
pub struct LaunchTemplateConfigBuilder {
    pub(crate) launch_template_specification:
        std::option::Option<crate::types::FleetLaunchTemplateSpecification>,
    pub(crate) overrides: std::option::Option<std::vec::Vec<crate::types::LaunchTemplateOverrides>>,
}
impl LaunchTemplateConfigBuilder {
    /// <p>The launch template.</p>
    pub fn launch_template_specification(
        mut self,
        input: crate::types::FleetLaunchTemplateSpecification,
    ) -> Self {
        self.launch_template_specification = Some(input);
        self
    }
    /// <p>The launch template.</p>
    pub fn set_launch_template_specification(
        mut self,
        input: std::option::Option<crate::types::FleetLaunchTemplateSpecification>,
    ) -> Self {
        self.launch_template_specification = input;
        self
    }
    /// Appends an item to `overrides`.
    ///
    /// To override the contents of this collection use [`set_overrides`](Self::set_overrides).
    ///
    /// <p>Any parameters that you specify override the same parameters in the launch template.</p>
    pub fn overrides(mut self, input: crate::types::LaunchTemplateOverrides) -> Self {
        let mut v = self.overrides.unwrap_or_default();
        v.push(input);
        self.overrides = Some(v);
        self
    }
    /// <p>Any parameters that you specify override the same parameters in the launch template.</p>
    pub fn set_overrides(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::LaunchTemplateOverrides>>,
    ) -> Self {
        self.overrides = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateConfig`](crate::types::LaunchTemplateConfig).
    pub fn build(self) -> crate::types::LaunchTemplateConfig {
        crate::types::LaunchTemplateConfig {
            launch_template_specification: self.launch_template_specification,
            overrides: self.overrides,
        }
    }
}