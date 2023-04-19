// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The CPU options for the instance.</p>
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
pub struct LaunchTemplateCpuOptions {
    /// <p>The number of CPU cores for the instance.</p>
    #[doc(hidden)]
    pub core_count: std::option::Option<i32>,
    /// <p>The number of threads per CPU core.</p>
    #[doc(hidden)]
    pub threads_per_core: std::option::Option<i32>,
}
impl LaunchTemplateCpuOptions {
    /// <p>The number of CPU cores for the instance.</p>
    pub fn core_count(&self) -> std::option::Option<i32> {
        self.core_count
    }
    /// <p>The number of threads per CPU core.</p>
    pub fn threads_per_core(&self) -> std::option::Option<i32> {
        self.threads_per_core
    }
}
impl LaunchTemplateCpuOptions {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateCpuOptions`](crate::types::LaunchTemplateCpuOptions).
    pub fn builder() -> crate::types::builders::LaunchTemplateCpuOptionsBuilder {
        crate::types::builders::LaunchTemplateCpuOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LaunchTemplateCpuOptions;
/// A builder for [`LaunchTemplateCpuOptions`](crate::types::LaunchTemplateCpuOptions).
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
pub struct LaunchTemplateCpuOptionsBuilder {
    pub(crate) core_count: std::option::Option<i32>,
    pub(crate) threads_per_core: std::option::Option<i32>,
}
impl LaunchTemplateCpuOptionsBuilder {
    /// <p>The number of CPU cores for the instance.</p>
    pub fn core_count(mut self, input: i32) -> Self {
        self.core_count = Some(input);
        self
    }
    /// <p>The number of CPU cores for the instance.</p>
    pub fn set_core_count(mut self, input: std::option::Option<i32>) -> Self {
        self.core_count = input;
        self
    }
    /// <p>The number of threads per CPU core.</p>
    pub fn threads_per_core(mut self, input: i32) -> Self {
        self.threads_per_core = Some(input);
        self
    }
    /// <p>The number of threads per CPU core.</p>
    pub fn set_threads_per_core(mut self, input: std::option::Option<i32>) -> Self {
        self.threads_per_core = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateCpuOptions`](crate::types::LaunchTemplateCpuOptions).
    pub fn build(self) -> crate::types::LaunchTemplateCpuOptions {
        crate::types::LaunchTemplateCpuOptions {
            core_count: self.core_count,
            threads_per_core: self.threads_per_core,
        }
    }
}
