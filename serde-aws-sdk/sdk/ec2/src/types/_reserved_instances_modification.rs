// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Reserved Instance modification.</p>
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
pub struct ReservedInstancesModification {
    /// <p>A unique, case-sensitive key supplied by the client to ensure that the request is idempotent. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>The time when the modification request was created.</p>
    #[doc(hidden)]
    pub create_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The time for the modification to become effective.</p>
    #[doc(hidden)]
    pub effective_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Contains target configurations along with their corresponding new Reserved Instance IDs.</p>
    #[doc(hidden)]
    pub modification_results:
        std::option::Option<std::vec::Vec<crate::types::ReservedInstancesModificationResult>>,
    /// <p>The IDs of one or more Reserved Instances.</p>
    #[doc(hidden)]
    pub reserved_instances_ids:
        std::option::Option<std::vec::Vec<crate::types::ReservedInstancesId>>,
    /// <p>A unique ID for the Reserved Instance modification.</p>
    #[doc(hidden)]
    pub reserved_instances_modification_id: std::option::Option<std::string::String>,
    /// <p>The status of the Reserved Instances modification request.</p>
    #[doc(hidden)]
    pub status: std::option::Option<std::string::String>,
    /// <p>The reason for the status.</p>
    #[doc(hidden)]
    pub status_message: std::option::Option<std::string::String>,
    /// <p>The time when the modification request was last updated.</p>
    #[doc(hidden)]
    pub update_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl ReservedInstancesModification {
    /// <p>A unique, case-sensitive key supplied by the client to ensure that the request is idempotent. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The time when the modification request was created.</p>
    pub fn create_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.create_date.as_ref()
    }
    /// <p>The time for the modification to become effective.</p>
    pub fn effective_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.effective_date.as_ref()
    }
    /// <p>Contains target configurations along with their corresponding new Reserved Instance IDs.</p>
    pub fn modification_results(
        &self,
    ) -> std::option::Option<&[crate::types::ReservedInstancesModificationResult]> {
        self.modification_results.as_deref()
    }
    /// <p>The IDs of one or more Reserved Instances.</p>
    pub fn reserved_instances_ids(
        &self,
    ) -> std::option::Option<&[crate::types::ReservedInstancesId]> {
        self.reserved_instances_ids.as_deref()
    }
    /// <p>A unique ID for the Reserved Instance modification.</p>
    pub fn reserved_instances_modification_id(&self) -> std::option::Option<&str> {
        self.reserved_instances_modification_id.as_deref()
    }
    /// <p>The status of the Reserved Instances modification request.</p>
    pub fn status(&self) -> std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The reason for the status.</p>
    pub fn status_message(&self) -> std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>The time when the modification request was last updated.</p>
    pub fn update_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.update_date.as_ref()
    }
}
impl ReservedInstancesModification {
    /// Creates a new builder-style object to manufacture [`ReservedInstancesModification`](crate::types::ReservedInstancesModification).
    pub fn builder() -> crate::types::builders::ReservedInstancesModificationBuilder {
        crate::types::builders::ReservedInstancesModificationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ReservedInstancesModification;
/// A builder for [`ReservedInstancesModification`](crate::types::ReservedInstancesModification).
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
pub struct ReservedInstancesModificationBuilder {
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) create_date: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) effective_date: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) modification_results:
        std::option::Option<std::vec::Vec<crate::types::ReservedInstancesModificationResult>>,
    pub(crate) reserved_instances_ids:
        std::option::Option<std::vec::Vec<crate::types::ReservedInstancesId>>,
    pub(crate) reserved_instances_modification_id: std::option::Option<std::string::String>,
    pub(crate) status: std::option::Option<std::string::String>,
    pub(crate) status_message: std::option::Option<std::string::String>,
    pub(crate) update_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl ReservedInstancesModificationBuilder {
    /// <p>A unique, case-sensitive key supplied by the client to ensure that the request is idempotent. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive key supplied by the client to ensure that the request is idempotent. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The time when the modification request was created.</p>
    pub fn create_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.create_date = Some(input);
        self
    }
    /// <p>The time when the modification request was created.</p>
    pub fn set_create_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_date = input;
        self
    }
    /// <p>The time for the modification to become effective.</p>
    pub fn effective_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.effective_date = Some(input);
        self
    }
    /// <p>The time for the modification to become effective.</p>
    pub fn set_effective_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.effective_date = input;
        self
    }
    /// Appends an item to `modification_results`.
    ///
    /// To override the contents of this collection use [`set_modification_results`](Self::set_modification_results).
    ///
    /// <p>Contains target configurations along with their corresponding new Reserved Instance IDs.</p>
    pub fn modification_results(
        mut self,
        input: crate::types::ReservedInstancesModificationResult,
    ) -> Self {
        let mut v = self.modification_results.unwrap_or_default();
        v.push(input);
        self.modification_results = Some(v);
        self
    }
    /// <p>Contains target configurations along with their corresponding new Reserved Instance IDs.</p>
    pub fn set_modification_results(
        mut self,
        input: std::option::Option<
            std::vec::Vec<crate::types::ReservedInstancesModificationResult>,
        >,
    ) -> Self {
        self.modification_results = input;
        self
    }
    /// Appends an item to `reserved_instances_ids`.
    ///
    /// To override the contents of this collection use [`set_reserved_instances_ids`](Self::set_reserved_instances_ids).
    ///
    /// <p>The IDs of one or more Reserved Instances.</p>
    pub fn reserved_instances_ids(mut self, input: crate::types::ReservedInstancesId) -> Self {
        let mut v = self.reserved_instances_ids.unwrap_or_default();
        v.push(input);
        self.reserved_instances_ids = Some(v);
        self
    }
    /// <p>The IDs of one or more Reserved Instances.</p>
    pub fn set_reserved_instances_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ReservedInstancesId>>,
    ) -> Self {
        self.reserved_instances_ids = input;
        self
    }
    /// <p>A unique ID for the Reserved Instance modification.</p>
    pub fn reserved_instances_modification_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.reserved_instances_modification_id = Some(input.into());
        self
    }
    /// <p>A unique ID for the Reserved Instance modification.</p>
    pub fn set_reserved_instances_modification_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.reserved_instances_modification_id = input;
        self
    }
    /// <p>The status of the Reserved Instances modification request.</p>
    pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
        self.status = Some(input.into());
        self
    }
    /// <p>The status of the Reserved Instances modification request.</p>
    pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The reason for the status.</p>
    pub fn status_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.status_message = Some(input.into());
        self
    }
    /// <p>The reason for the status.</p>
    pub fn set_status_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// <p>The time when the modification request was last updated.</p>
    pub fn update_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.update_date = Some(input);
        self
    }
    /// <p>The time when the modification request was last updated.</p>
    pub fn set_update_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.update_date = input;
        self
    }
    /// Consumes the builder and constructs a [`ReservedInstancesModification`](crate::types::ReservedInstancesModification).
    pub fn build(self) -> crate::types::ReservedInstancesModification {
        crate::types::ReservedInstancesModification {
            client_token: self.client_token,
            create_date: self.create_date,
            effective_date: self.effective_date,
            modification_results: self.modification_results,
            reserved_instances_ids: self.reserved_instances_ids,
            reserved_instances_modification_id: self.reserved_instances_modification_id,
            status: self.status,
            status_message: self.status_message,
            update_date: self.update_date,
        }
    }
}
