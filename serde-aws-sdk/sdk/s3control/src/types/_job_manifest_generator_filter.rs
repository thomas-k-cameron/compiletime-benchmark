// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The filter used to describe a set of objects for the job's manifest.</p>
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
pub struct JobManifestGeneratorFilter {
    /// <p>Include objects in the generated manifest only if they are eligible for replication according to the Replication configuration on the source bucket.</p>
    #[doc(hidden)]
    pub eligible_for_replication: std::option::Option<bool>,
    /// <p>If provided, the generated manifest should include only source bucket objects that were created after this time.</p>
    #[doc(hidden)]
    pub created_after: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>If provided, the generated manifest should include only source bucket objects that were created before this time.</p>
    #[doc(hidden)]
    pub created_before: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>If provided, the generated manifest should include only source bucket objects that have one of the specified Replication statuses.</p>
    #[doc(hidden)]
    pub object_replication_statuses:
        std::option::Option<std::vec::Vec<crate::types::ReplicationStatus>>,
}
impl JobManifestGeneratorFilter {
    /// <p>Include objects in the generated manifest only if they are eligible for replication according to the Replication configuration on the source bucket.</p>
    pub fn eligible_for_replication(&self) -> std::option::Option<bool> {
        self.eligible_for_replication
    }
    /// <p>If provided, the generated manifest should include only source bucket objects that were created after this time.</p>
    pub fn created_after(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.created_after.as_ref()
    }
    /// <p>If provided, the generated manifest should include only source bucket objects that were created before this time.</p>
    pub fn created_before(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.created_before.as_ref()
    }
    /// <p>If provided, the generated manifest should include only source bucket objects that have one of the specified Replication statuses.</p>
    pub fn object_replication_statuses(
        &self,
    ) -> std::option::Option<&[crate::types::ReplicationStatus]> {
        self.object_replication_statuses.as_deref()
    }
}
impl JobManifestGeneratorFilter {
    /// Creates a new builder-style object to manufacture [`JobManifestGeneratorFilter`](crate::types::JobManifestGeneratorFilter).
    pub fn builder() -> crate::types::builders::JobManifestGeneratorFilterBuilder {
        crate::types::builders::JobManifestGeneratorFilterBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::JobManifestGeneratorFilter;
/// A builder for [`JobManifestGeneratorFilter`](crate::types::JobManifestGeneratorFilter).
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
pub struct JobManifestGeneratorFilterBuilder {
    pub(crate) eligible_for_replication: std::option::Option<bool>,
    pub(crate) created_after: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) created_before: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) object_replication_statuses:
        std::option::Option<std::vec::Vec<crate::types::ReplicationStatus>>,
}
impl JobManifestGeneratorFilterBuilder {
    /// <p>Include objects in the generated manifest only if they are eligible for replication according to the Replication configuration on the source bucket.</p>
    pub fn eligible_for_replication(mut self, input: bool) -> Self {
        self.eligible_for_replication = Some(input);
        self
    }
    /// <p>Include objects in the generated manifest only if they are eligible for replication according to the Replication configuration on the source bucket.</p>
    pub fn set_eligible_for_replication(mut self, input: std::option::Option<bool>) -> Self {
        self.eligible_for_replication = input;
        self
    }
    /// <p>If provided, the generated manifest should include only source bucket objects that were created after this time.</p>
    pub fn created_after(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.created_after = Some(input);
        self
    }
    /// <p>If provided, the generated manifest should include only source bucket objects that were created after this time.</p>
    pub fn set_created_after(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_after = input;
        self
    }
    /// <p>If provided, the generated manifest should include only source bucket objects that were created before this time.</p>
    pub fn created_before(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.created_before = Some(input);
        self
    }
    /// <p>If provided, the generated manifest should include only source bucket objects that were created before this time.</p>
    pub fn set_created_before(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_before = input;
        self
    }
    /// Appends an item to `object_replication_statuses`.
    ///
    /// To override the contents of this collection use [`set_object_replication_statuses`](Self::set_object_replication_statuses).
    ///
    /// <p>If provided, the generated manifest should include only source bucket objects that have one of the specified Replication statuses.</p>
    pub fn object_replication_statuses(mut self, input: crate::types::ReplicationStatus) -> Self {
        let mut v = self.object_replication_statuses.unwrap_or_default();
        v.push(input);
        self.object_replication_statuses = Some(v);
        self
    }
    /// <p>If provided, the generated manifest should include only source bucket objects that have one of the specified Replication statuses.</p>
    pub fn set_object_replication_statuses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ReplicationStatus>>,
    ) -> Self {
        self.object_replication_statuses = input;
        self
    }
    /// Consumes the builder and constructs a [`JobManifestGeneratorFilter`](crate::types::JobManifestGeneratorFilter).
    pub fn build(self) -> crate::types::JobManifestGeneratorFilter {
        crate::types::JobManifestGeneratorFilter {
            eligible_for_replication: self.eligible_for_replication,
            created_after: self.created_after,
            created_before: self.created_before,
            object_replication_statuses: self.object_replication_statuses,
        }
    }
}