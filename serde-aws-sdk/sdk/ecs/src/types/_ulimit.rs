// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The <code>ulimit</code> settings to pass to the container.</p>
/// <p>Amazon ECS tasks hosted on Fargate use the default resource limit values set by the operating system with the exception of the <code>nofile</code> resource limit parameter which Fargate overrides. The <code>nofile</code> resource limit sets a restriction on the number of open files that a container can use. The default <code>nofile</code> soft limit is <code>1024</code> and the default hard limit is <code>4096</code>.</p>
/// <p>You can specify the <code>ulimit</code> settings for a container in a task definition.</p>
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
pub struct Ulimit {
    /// <p>The <code>type</code> of the <code>ulimit</code>.</p>
    #[doc(hidden)]
    pub name: std::option::Option<crate::types::UlimitName>,
    /// <p>The soft limit for the <code>ulimit</code> type.</p>
    #[doc(hidden)]
    pub soft_limit: i32,
    /// <p>The hard limit for the <code>ulimit</code> type.</p>
    #[doc(hidden)]
    pub hard_limit: i32,
}
impl Ulimit {
    /// <p>The <code>type</code> of the <code>ulimit</code>.</p>
    pub fn name(&self) -> std::option::Option<&crate::types::UlimitName> {
        self.name.as_ref()
    }
    /// <p>The soft limit for the <code>ulimit</code> type.</p>
    pub fn soft_limit(&self) -> i32 {
        self.soft_limit
    }
    /// <p>The hard limit for the <code>ulimit</code> type.</p>
    pub fn hard_limit(&self) -> i32 {
        self.hard_limit
    }
}
impl Ulimit {
    /// Creates a new builder-style object to manufacture [`Ulimit`](crate::types::Ulimit).
    pub fn builder() -> crate::types::builders::UlimitBuilder {
        crate::types::builders::UlimitBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Ulimit;
/// A builder for [`Ulimit`](crate::types::Ulimit).
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
pub struct UlimitBuilder {
    pub(crate) name: std::option::Option<crate::types::UlimitName>,
    pub(crate) soft_limit: std::option::Option<i32>,
    pub(crate) hard_limit: std::option::Option<i32>,
}
impl UlimitBuilder {
    /// <p>The <code>type</code> of the <code>ulimit</code>.</p>
    pub fn name(mut self, input: crate::types::UlimitName) -> Self {
        self.name = Some(input);
        self
    }
    /// <p>The <code>type</code> of the <code>ulimit</code>.</p>
    pub fn set_name(mut self, input: std::option::Option<crate::types::UlimitName>) -> Self {
        self.name = input;
        self
    }
    /// <p>The soft limit for the <code>ulimit</code> type.</p>
    pub fn soft_limit(mut self, input: i32) -> Self {
        self.soft_limit = Some(input);
        self
    }
    /// <p>The soft limit for the <code>ulimit</code> type.</p>
    pub fn set_soft_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.soft_limit = input;
        self
    }
    /// <p>The hard limit for the <code>ulimit</code> type.</p>
    pub fn hard_limit(mut self, input: i32) -> Self {
        self.hard_limit = Some(input);
        self
    }
    /// <p>The hard limit for the <code>ulimit</code> type.</p>
    pub fn set_hard_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.hard_limit = input;
        self
    }
    /// Consumes the builder and constructs a [`Ulimit`](crate::types::Ulimit).
    pub fn build(self) -> crate::types::Ulimit {
        crate::types::Ulimit {
            name: self.name,
            soft_limit: self.soft_limit.unwrap_or_default(),
            hard_limit: self.hard_limit.unwrap_or_default(),
        }
    }
}
