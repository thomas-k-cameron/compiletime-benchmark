// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains change information for the resource record set.</p>
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
pub struct ChangeResourceRecordSetsInput {
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to change.</p>
    #[doc(hidden)]
    pub hosted_zone_id: std::option::Option<std::string::String>,
    /// <p>A complex type that contains an optional comment and the <code>Changes</code> element.</p>
    #[doc(hidden)]
    pub change_batch: std::option::Option<crate::types::ChangeBatch>,
}
impl ChangeResourceRecordSetsInput {
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to change.</p>
    pub fn hosted_zone_id(&self) -> std::option::Option<&str> {
        self.hosted_zone_id.as_deref()
    }
    /// <p>A complex type that contains an optional comment and the <code>Changes</code> element.</p>
    pub fn change_batch(&self) -> std::option::Option<&crate::types::ChangeBatch> {
        self.change_batch.as_ref()
    }
}
impl ChangeResourceRecordSetsInput {
    /// Creates a new builder-style object to manufacture [`ChangeResourceRecordSetsInput`](crate::operation::change_resource_record_sets::ChangeResourceRecordSetsInput).
    pub fn builder(
    ) -> crate::operation::change_resource_record_sets::builders::ChangeResourceRecordSetsInputBuilder
    {
        crate::operation::change_resource_record_sets::builders::ChangeResourceRecordSetsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::change_resource_record_sets::ChangeResourceRecordSetsInput;
/// A builder for [`ChangeResourceRecordSetsInput`](crate::operation::change_resource_record_sets::ChangeResourceRecordSetsInput).
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
pub struct ChangeResourceRecordSetsInputBuilder {
    pub(crate) hosted_zone_id: std::option::Option<std::string::String>,
    pub(crate) change_batch: std::option::Option<crate::types::ChangeBatch>,
}
impl ChangeResourceRecordSetsInputBuilder {
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to change.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.hosted_zone_id = Some(input.into());
        self
    }
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to change.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.hosted_zone_id = input;
        self
    }
    /// <p>A complex type that contains an optional comment and the <code>Changes</code> element.</p>
    pub fn change_batch(mut self, input: crate::types::ChangeBatch) -> Self {
        self.change_batch = Some(input);
        self
    }
    /// <p>A complex type that contains an optional comment and the <code>Changes</code> element.</p>
    pub fn set_change_batch(
        mut self,
        input: std::option::Option<crate::types::ChangeBatch>,
    ) -> Self {
        self.change_batch = input;
        self
    }
    /// Consumes the builder and constructs a [`ChangeResourceRecordSetsInput`](crate::operation::change_resource_record_sets::ChangeResourceRecordSetsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::change_resource_record_sets::ChangeResourceRecordSetsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::change_resource_record_sets::ChangeResourceRecordSetsInput {
                hosted_zone_id: self.hosted_zone_id,
                change_batch: self.change_batch,
            },
        )
    }
}
