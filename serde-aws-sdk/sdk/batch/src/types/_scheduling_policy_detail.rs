// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents a scheduling policy.</p>
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
pub struct SchedulingPolicyDetail {
    /// <p>The name of the scheduling policy.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. An example is <code>arn:<i>aws</i>:batch:<i>us-east-1</i>:<i>123456789012</i>:scheduling-policy/<i>HighPriority</i> </code>.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>The fair share policy for the scheduling policy.</p>
    #[doc(hidden)]
    pub fairshare_policy: std::option::Option<crate::types::FairsharePolicy>,
    /// <p>The tags that you apply to the scheduling policy to categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in <i>Amazon Web Services General Reference</i>.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl SchedulingPolicyDetail {
    /// <p>The name of the scheduling policy.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. An example is <code>arn:<i>aws</i>:batch:<i>us-east-1</i>:<i>123456789012</i>:scheduling-policy/<i>HighPriority</i> </code>.</p>
    pub fn arn(&self) -> std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The fair share policy for the scheduling policy.</p>
    pub fn fairshare_policy(&self) -> std::option::Option<&crate::types::FairsharePolicy> {
        self.fairshare_policy.as_ref()
    }
    /// <p>The tags that you apply to the scheduling policy to categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in <i>Amazon Web Services General Reference</i>.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl SchedulingPolicyDetail {
    /// Creates a new builder-style object to manufacture [`SchedulingPolicyDetail`](crate::types::SchedulingPolicyDetail).
    pub fn builder() -> crate::types::builders::SchedulingPolicyDetailBuilder {
        crate::types::builders::SchedulingPolicyDetailBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SchedulingPolicyDetail;
/// A builder for [`SchedulingPolicyDetail`](crate::types::SchedulingPolicyDetail).
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
pub struct SchedulingPolicyDetailBuilder {
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) arn: std::option::Option<std::string::String>,
    pub(crate) fairshare_policy: std::option::Option<crate::types::FairsharePolicy>,
    pub(crate) tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl SchedulingPolicyDetailBuilder {
    /// <p>The name of the scheduling policy.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the scheduling policy.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. An example is <code>arn:<i>aws</i>:batch:<i>us-east-1</i>:<i>123456789012</i>:scheduling-policy/<i>HighPriority</i> </code>.</p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. An example is <code>arn:<i>aws</i>:batch:<i>us-east-1</i>:<i>123456789012</i>:scheduling-policy/<i>HighPriority</i> </code>.</p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The fair share policy for the scheduling policy.</p>
    pub fn fairshare_policy(mut self, input: crate::types::FairsharePolicy) -> Self {
        self.fairshare_policy = Some(input);
        self
    }
    /// <p>The fair share policy for the scheduling policy.</p>
    pub fn set_fairshare_policy(
        mut self,
        input: std::option::Option<crate::types::FairsharePolicy>,
    ) -> Self {
        self.fairshare_policy = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags that you apply to the scheduling policy to categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in <i>Amazon Web Services General Reference</i>.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = Some(hash_map);
        self
    }
    /// <p>The tags that you apply to the scheduling policy to categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in <i>Amazon Web Services General Reference</i>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`SchedulingPolicyDetail`](crate::types::SchedulingPolicyDetail).
    pub fn build(self) -> crate::types::SchedulingPolicyDetail {
        crate::types::SchedulingPolicyDetail {
            name: self.name,
            arn: self.arn,
            fairshare_policy: self.fairshare_policy,
            tags: self.tags,
        }
    }
}