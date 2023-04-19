// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_global_table_settings::_update_global_table_settings_output::UpdateGlobalTableSettingsOutputBuilder;

pub use crate::operation::update_global_table_settings::_update_global_table_settings_input::UpdateGlobalTableSettingsInputBuilder;

/// Fluent builder constructing a request to `UpdateGlobalTableSettings`.
///
/// <p>Updates settings for a global table.</p> <important>
/// <p>This operation only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V1.html">Version 2017.11.29 (Legacy)</a> of global tables. We recommend using <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21 (Current)</a> when creating new global tables, as it provides greater flexibility, higher efficiency and consumes less write capacity than 2017.11.29 (Legacy). To determine which version you are using, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.DetermineVersion.html">Determining the version</a>. To update existing global tables from version 2017.11.29 (Legacy) to version 2019.11.21 (Current), see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/V2globaltables_upgrade.html"> Updating global tables</a>. </p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateGlobalTableSettingsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsInputBuilder
            }
impl UpdateGlobalTableSettingsFluentBuilder {
    /// Creates a new `UpdateGlobalTableSettings`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_global_table_settings::UpdateGlobalTableSettings,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_global_table_settings::UpdateGlobalTableSettingsError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::update_global_table_settings::UpdateGlobalTableSettingsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_global_table_settings::UpdateGlobalTableSettingsError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.update_global_table_settings().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the global table</p>
    pub fn global_table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.global_table_name(input.into());
        self
    }
    /// <p>The name of the global table</p>
    pub fn set_global_table_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_table_name(input);
        self
    }
    /// <p>The billing mode of the global table. If <code>GlobalTableBillingMode</code> is not specified, the global table defaults to <code>PROVISIONED</code> capacity billing mode.</p>
    /// <ul>
    /// <li> <p> <code>PROVISIONED</code> - We recommend using <code>PROVISIONED</code> for predictable workloads. <code>PROVISIONED</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.ProvisionedThroughput.Manual">Provisioned Mode</a>.</p> </li>
    /// <li> <p> <code>PAY_PER_REQUEST</code> - We recommend using <code>PAY_PER_REQUEST</code> for unpredictable workloads. <code>PAY_PER_REQUEST</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.OnDemand">On-Demand Mode</a>. </p> </li>
    /// </ul>
    pub fn global_table_billing_mode(mut self, input: crate::types::BillingMode) -> Self {
        self.inner = self.inner.global_table_billing_mode(input);
        self
    }
    /// <p>The billing mode of the global table. If <code>GlobalTableBillingMode</code> is not specified, the global table defaults to <code>PROVISIONED</code> capacity billing mode.</p>
    /// <ul>
    /// <li> <p> <code>PROVISIONED</code> - We recommend using <code>PROVISIONED</code> for predictable workloads. <code>PROVISIONED</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.ProvisionedThroughput.Manual">Provisioned Mode</a>.</p> </li>
    /// <li> <p> <code>PAY_PER_REQUEST</code> - We recommend using <code>PAY_PER_REQUEST</code> for unpredictable workloads. <code>PAY_PER_REQUEST</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.OnDemand">On-Demand Mode</a>. </p> </li>
    /// </ul>
    pub fn set_global_table_billing_mode(
        mut self,
        input: std::option::Option<crate::types::BillingMode>,
    ) -> Self {
        self.inner = self.inner.set_global_table_billing_mode(input);
        self
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException.</code> </p>
    pub fn global_table_provisioned_write_capacity_units(mut self, input: i64) -> Self {
        self.inner = self
            .inner
            .global_table_provisioned_write_capacity_units(input);
        self
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException.</code> </p>
    pub fn set_global_table_provisioned_write_capacity_units(
        mut self,
        input: std::option::Option<i64>,
    ) -> Self {
        self.inner = self
            .inner
            .set_global_table_provisioned_write_capacity_units(input);
        self
    }
    /// <p>Auto scaling settings for managing provisioned write capacity for the global table.</p>
    pub fn global_table_provisioned_write_capacity_auto_scaling_settings_update(
        mut self,
        input: crate::types::AutoScalingSettingsUpdate,
    ) -> Self {
        self.inner = self
            .inner
            .global_table_provisioned_write_capacity_auto_scaling_settings_update(input);
        self
    }
    /// <p>Auto scaling settings for managing provisioned write capacity for the global table.</p>
    pub fn set_global_table_provisioned_write_capacity_auto_scaling_settings_update(
        mut self,
        input: std::option::Option<crate::types::AutoScalingSettingsUpdate>,
    ) -> Self {
        self.inner = self
            .inner
            .set_global_table_provisioned_write_capacity_auto_scaling_settings_update(input);
        self
    }
    /// Appends an item to `GlobalTableGlobalSecondaryIndexSettingsUpdate`.
    ///
    /// To override the contents of this collection use [`set_global_table_global_secondary_index_settings_update`](Self::set_global_table_global_secondary_index_settings_update).
    ///
    /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
    pub fn global_table_global_secondary_index_settings_update(
        mut self,
        input: crate::types::GlobalTableGlobalSecondaryIndexSettingsUpdate,
    ) -> Self {
        self.inner = self
            .inner
            .global_table_global_secondary_index_settings_update(input);
        self
    }
    /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
    pub fn set_global_table_global_secondary_index_settings_update(
        mut self,
        input: std::option::Option<
            std::vec::Vec<crate::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>,
        >,
    ) -> Self {
        self.inner = self
            .inner
            .set_global_table_global_secondary_index_settings_update(input);
        self
    }
    /// Appends an item to `ReplicaSettingsUpdate`.
    ///
    /// To override the contents of this collection use [`set_replica_settings_update`](Self::set_replica_settings_update).
    ///
    /// <p>Represents the settings for a global table in a Region that will be modified.</p>
    pub fn replica_settings_update(mut self, input: crate::types::ReplicaSettingsUpdate) -> Self {
        self.inner = self.inner.replica_settings_update(input);
        self
    }
    /// <p>Represents the settings for a global table in a Region that will be modified.</p>
    pub fn set_replica_settings_update(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ReplicaSettingsUpdate>>,
    ) -> Self {
        self.inner = self.inner.set_replica_settings_update(input);
        self
    }
}
