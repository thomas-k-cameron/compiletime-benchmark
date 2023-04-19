// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the details of the replica.</p>
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
pub struct ReplicaDescription {
    /// <p>The name of the Region.</p>
    #[doc(hidden)]
    pub region_name: std::option::Option<std::string::String>,
    /// <p>The current state of the replica:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The replica is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The replica is being updated.</p> </li>
    /// <li> <p> <code>DELETING</code> - The replica is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The replica is ready for use.</p> </li>
    /// <li> <p> <code>REGION_DISABLED</code> - The replica is inaccessible because the Amazon Web Services Region has been disabled.</p> <note>
    /// <p>If the Amazon Web Services Region remains inaccessible for more than 20 hours, DynamoDB will remove this replica from the replication group. The replica will not be deleted and replication will stop from and to this region.</p>
    /// </note> </li>
    /// <li> <p> <code>INACCESSIBLE_ENCRYPTION_CREDENTIALS </code> - The KMS key used to encrypt the table is inaccessible.</p> <note>
    /// <p>If the KMS key remains inaccessible for more than 20 hours, DynamoDB will remove this replica from the replication group. The replica will not be deleted and replication will stop from and to this region.</p>
    /// </note> </li>
    /// </ul>
    #[doc(hidden)]
    pub replica_status: std::option::Option<crate::types::ReplicaStatus>,
    /// <p>Detailed information about the replica status.</p>
    #[doc(hidden)]
    pub replica_status_description: std::option::Option<std::string::String>,
    /// <p>Specifies the progress of a Create, Update, or Delete action on the replica as a percentage.</p>
    #[doc(hidden)]
    pub replica_status_percent_progress: std::option::Option<std::string::String>,
    /// <p>The KMS key of the replica that will be used for KMS encryption.</p>
    #[doc(hidden)]
    pub kms_master_key_id: std::option::Option<std::string::String>,
    /// <p>Replica-specific provisioned throughput. If not described, uses the source table's provisioned throughput settings.</p>
    #[doc(hidden)]
    pub provisioned_throughput_override:
        std::option::Option<crate::types::ProvisionedThroughputOverride>,
    /// <p>Replica-specific global secondary index settings.</p>
    #[doc(hidden)]
    pub global_secondary_indexes:
        std::option::Option<std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexDescription>>,
    /// <p>The time at which the replica was first detected as inaccessible. To determine cause of inaccessibility check the <code>ReplicaStatus</code> property.</p>
    #[doc(hidden)]
    pub replica_inaccessible_date_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Contains details of the table class.</p>
    #[doc(hidden)]
    pub replica_table_class_summary: std::option::Option<crate::types::TableClassSummary>,
}
impl ReplicaDescription {
    /// <p>The name of the Region.</p>
    pub fn region_name(&self) -> std::option::Option<&str> {
        self.region_name.as_deref()
    }
    /// <p>The current state of the replica:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The replica is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The replica is being updated.</p> </li>
    /// <li> <p> <code>DELETING</code> - The replica is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The replica is ready for use.</p> </li>
    /// <li> <p> <code>REGION_DISABLED</code> - The replica is inaccessible because the Amazon Web Services Region has been disabled.</p> <note>
    /// <p>If the Amazon Web Services Region remains inaccessible for more than 20 hours, DynamoDB will remove this replica from the replication group. The replica will not be deleted and replication will stop from and to this region.</p>
    /// </note> </li>
    /// <li> <p> <code>INACCESSIBLE_ENCRYPTION_CREDENTIALS </code> - The KMS key used to encrypt the table is inaccessible.</p> <note>
    /// <p>If the KMS key remains inaccessible for more than 20 hours, DynamoDB will remove this replica from the replication group. The replica will not be deleted and replication will stop from and to this region.</p>
    /// </note> </li>
    /// </ul>
    pub fn replica_status(&self) -> std::option::Option<&crate::types::ReplicaStatus> {
        self.replica_status.as_ref()
    }
    /// <p>Detailed information about the replica status.</p>
    pub fn replica_status_description(&self) -> std::option::Option<&str> {
        self.replica_status_description.as_deref()
    }
    /// <p>Specifies the progress of a Create, Update, or Delete action on the replica as a percentage.</p>
    pub fn replica_status_percent_progress(&self) -> std::option::Option<&str> {
        self.replica_status_percent_progress.as_deref()
    }
    /// <p>The KMS key of the replica that will be used for KMS encryption.</p>
    pub fn kms_master_key_id(&self) -> std::option::Option<&str> {
        self.kms_master_key_id.as_deref()
    }
    /// <p>Replica-specific provisioned throughput. If not described, uses the source table's provisioned throughput settings.</p>
    pub fn provisioned_throughput_override(
        &self,
    ) -> std::option::Option<&crate::types::ProvisionedThroughputOverride> {
        self.provisioned_throughput_override.as_ref()
    }
    /// <p>Replica-specific global secondary index settings.</p>
    pub fn global_secondary_indexes(
        &self,
    ) -> std::option::Option<&[crate::types::ReplicaGlobalSecondaryIndexDescription]> {
        self.global_secondary_indexes.as_deref()
    }
    /// <p>The time at which the replica was first detected as inaccessible. To determine cause of inaccessibility check the <code>ReplicaStatus</code> property.</p>
    pub fn replica_inaccessible_date_time(
        &self,
    ) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.replica_inaccessible_date_time.as_ref()
    }
    /// <p>Contains details of the table class.</p>
    pub fn replica_table_class_summary(
        &self,
    ) -> std::option::Option<&crate::types::TableClassSummary> {
        self.replica_table_class_summary.as_ref()
    }
}
impl ReplicaDescription {
    /// Creates a new builder-style object to manufacture [`ReplicaDescription`](crate::types::ReplicaDescription).
    pub fn builder() -> crate::types::builders::ReplicaDescriptionBuilder {
        crate::types::builders::ReplicaDescriptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ReplicaDescription;
/// A builder for [`ReplicaDescription`](crate::types::ReplicaDescription).
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
pub struct ReplicaDescriptionBuilder {
    pub(crate) region_name: std::option::Option<std::string::String>,
    pub(crate) replica_status: std::option::Option<crate::types::ReplicaStatus>,
    pub(crate) replica_status_description: std::option::Option<std::string::String>,
    pub(crate) replica_status_percent_progress: std::option::Option<std::string::String>,
    pub(crate) kms_master_key_id: std::option::Option<std::string::String>,
    pub(crate) provisioned_throughput_override:
        std::option::Option<crate::types::ProvisionedThroughputOverride>,
    pub(crate) global_secondary_indexes:
        std::option::Option<std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexDescription>>,
    pub(crate) replica_inaccessible_date_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) replica_table_class_summary: std::option::Option<crate::types::TableClassSummary>,
}
impl ReplicaDescriptionBuilder {
    /// <p>The name of the Region.</p>
    pub fn region_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.region_name = Some(input.into());
        self
    }
    /// <p>The name of the Region.</p>
    pub fn set_region_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.region_name = input;
        self
    }
    /// <p>The current state of the replica:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The replica is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The replica is being updated.</p> </li>
    /// <li> <p> <code>DELETING</code> - The replica is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The replica is ready for use.</p> </li>
    /// <li> <p> <code>REGION_DISABLED</code> - The replica is inaccessible because the Amazon Web Services Region has been disabled.</p> <note>
    /// <p>If the Amazon Web Services Region remains inaccessible for more than 20 hours, DynamoDB will remove this replica from the replication group. The replica will not be deleted and replication will stop from and to this region.</p>
    /// </note> </li>
    /// <li> <p> <code>INACCESSIBLE_ENCRYPTION_CREDENTIALS </code> - The KMS key used to encrypt the table is inaccessible.</p> <note>
    /// <p>If the KMS key remains inaccessible for more than 20 hours, DynamoDB will remove this replica from the replication group. The replica will not be deleted and replication will stop from and to this region.</p>
    /// </note> </li>
    /// </ul>
    pub fn replica_status(mut self, input: crate::types::ReplicaStatus) -> Self {
        self.replica_status = Some(input);
        self
    }
    /// <p>The current state of the replica:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The replica is being created.</p> </li>
    /// <li> <p> <code>UPDATING</code> - The replica is being updated.</p> </li>
    /// <li> <p> <code>DELETING</code> - The replica is being deleted.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The replica is ready for use.</p> </li>
    /// <li> <p> <code>REGION_DISABLED</code> - The replica is inaccessible because the Amazon Web Services Region has been disabled.</p> <note>
    /// <p>If the Amazon Web Services Region remains inaccessible for more than 20 hours, DynamoDB will remove this replica from the replication group. The replica will not be deleted and replication will stop from and to this region.</p>
    /// </note> </li>
    /// <li> <p> <code>INACCESSIBLE_ENCRYPTION_CREDENTIALS </code> - The KMS key used to encrypt the table is inaccessible.</p> <note>
    /// <p>If the KMS key remains inaccessible for more than 20 hours, DynamoDB will remove this replica from the replication group. The replica will not be deleted and replication will stop from and to this region.</p>
    /// </note> </li>
    /// </ul>
    pub fn set_replica_status(
        mut self,
        input: std::option::Option<crate::types::ReplicaStatus>,
    ) -> Self {
        self.replica_status = input;
        self
    }
    /// <p>Detailed information about the replica status.</p>
    pub fn replica_status_description(mut self, input: impl Into<std::string::String>) -> Self {
        self.replica_status_description = Some(input.into());
        self
    }
    /// <p>Detailed information about the replica status.</p>
    pub fn set_replica_status_description(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.replica_status_description = input;
        self
    }
    /// <p>Specifies the progress of a Create, Update, or Delete action on the replica as a percentage.</p>
    pub fn replica_status_percent_progress(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.replica_status_percent_progress = Some(input.into());
        self
    }
    /// <p>Specifies the progress of a Create, Update, or Delete action on the replica as a percentage.</p>
    pub fn set_replica_status_percent_progress(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.replica_status_percent_progress = input;
        self
    }
    /// <p>The KMS key of the replica that will be used for KMS encryption.</p>
    pub fn kms_master_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.kms_master_key_id = Some(input.into());
        self
    }
    /// <p>The KMS key of the replica that will be used for KMS encryption.</p>
    pub fn set_kms_master_key_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.kms_master_key_id = input;
        self
    }
    /// <p>Replica-specific provisioned throughput. If not described, uses the source table's provisioned throughput settings.</p>
    pub fn provisioned_throughput_override(
        mut self,
        input: crate::types::ProvisionedThroughputOverride,
    ) -> Self {
        self.provisioned_throughput_override = Some(input);
        self
    }
    /// <p>Replica-specific provisioned throughput. If not described, uses the source table's provisioned throughput settings.</p>
    pub fn set_provisioned_throughput_override(
        mut self,
        input: std::option::Option<crate::types::ProvisionedThroughputOverride>,
    ) -> Self {
        self.provisioned_throughput_override = input;
        self
    }
    /// Appends an item to `global_secondary_indexes`.
    ///
    /// To override the contents of this collection use [`set_global_secondary_indexes`](Self::set_global_secondary_indexes).
    ///
    /// <p>Replica-specific global secondary index settings.</p>
    pub fn global_secondary_indexes(
        mut self,
        input: crate::types::ReplicaGlobalSecondaryIndexDescription,
    ) -> Self {
        let mut v = self.global_secondary_indexes.unwrap_or_default();
        v.push(input);
        self.global_secondary_indexes = Some(v);
        self
    }
    /// <p>Replica-specific global secondary index settings.</p>
    pub fn set_global_secondary_indexes(
        mut self,
        input: std::option::Option<
            std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexDescription>,
        >,
    ) -> Self {
        self.global_secondary_indexes = input;
        self
    }
    /// <p>The time at which the replica was first detected as inaccessible. To determine cause of inaccessibility check the <code>ReplicaStatus</code> property.</p>
    pub fn replica_inaccessible_date_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.replica_inaccessible_date_time = Some(input);
        self
    }
    /// <p>The time at which the replica was first detected as inaccessible. To determine cause of inaccessibility check the <code>ReplicaStatus</code> property.</p>
    pub fn set_replica_inaccessible_date_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.replica_inaccessible_date_time = input;
        self
    }
    /// <p>Contains details of the table class.</p>
    pub fn replica_table_class_summary(mut self, input: crate::types::TableClassSummary) -> Self {
        self.replica_table_class_summary = Some(input);
        self
    }
    /// <p>Contains details of the table class.</p>
    pub fn set_replica_table_class_summary(
        mut self,
        input: std::option::Option<crate::types::TableClassSummary>,
    ) -> Self {
        self.replica_table_class_summary = input;
        self
    }
    /// Consumes the builder and constructs a [`ReplicaDescription`](crate::types::ReplicaDescription).
    pub fn build(self) -> crate::types::ReplicaDescription {
        crate::types::ReplicaDescription {
            region_name: self.region_name,
            replica_status: self.replica_status,
            replica_status_description: self.replica_status_description,
            replica_status_percent_progress: self.replica_status_percent_progress,
            kms_master_key_id: self.kms_master_key_id,
            provisioned_throughput_override: self.provisioned_throughput_override,
            global_secondary_indexes: self.global_secondary_indexes,
            replica_inaccessible_date_time: self.replica_inaccessible_date_time,
            replica_table_class_summary: self.replica_table_class_summary,
        }
    }
}