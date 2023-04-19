// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::restore_table_from_backup::_restore_table_from_backup_output::RestoreTableFromBackupOutputBuilder;

pub use crate::operation::restore_table_from_backup::_restore_table_from_backup_input::RestoreTableFromBackupInputBuilder;

/// Fluent builder constructing a request to `RestoreTableFromBackup`.
///
/// <p>Creates a new table from an existing backup. Any number of users can execute up to 4 concurrent restores (any type of restore) in a given account. </p>
/// <p>You can call <code>RestoreTableFromBackup</code> at a maximum rate of 10 times per second.</p>
/// <p>You must manually set up the following on the restored table:</p>
/// <ul>
/// <li> <p>Auto scaling policies</p> </li>
/// <li> <p>IAM policies</p> </li>
/// <li> <p>Amazon CloudWatch metrics and alarms</p> </li>
/// <li> <p>Tags</p> </li>
/// <li> <p>Stream settings</p> </li>
/// <li> <p>Time to Live (TTL) settings</p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RestoreTableFromBackupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInputBuilder,
}
impl RestoreTableFromBackupFluentBuilder {
    /// Creates a new `RestoreTableFromBackup`.
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
            crate::operation::restore_table_from_backup::RestoreTableFromBackup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
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
        crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
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
    ///     let deserialized_parameters: crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.restore_table_from_backup().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the new table to which the backup must be restored.</p>
    pub fn target_table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_table_name(input.into());
        self
    }
    /// <p>The name of the new table to which the backup must be restored.</p>
    pub fn set_target_table_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_target_table_name(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    pub fn backup_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.backup_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    pub fn set_backup_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_backup_arn(input);
        self
    }
    /// <p>The billing mode of the restored table.</p>
    pub fn billing_mode_override(mut self, input: crate::types::BillingMode) -> Self {
        self.inner = self.inner.billing_mode_override(input);
        self
    }
    /// <p>The billing mode of the restored table.</p>
    pub fn set_billing_mode_override(
        mut self,
        input: std::option::Option<crate::types::BillingMode>,
    ) -> Self {
        self.inner = self.inner.set_billing_mode_override(input);
        self
    }
    /// Appends an item to `GlobalSecondaryIndexOverride`.
    ///
    /// To override the contents of this collection use [`set_global_secondary_index_override`](Self::set_global_secondary_index_override).
    ///
    /// <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn global_secondary_index_override(
        mut self,
        input: crate::types::GlobalSecondaryIndex,
    ) -> Self {
        self.inner = self.inner.global_secondary_index_override(input);
        self
    }
    /// <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn set_global_secondary_index_override(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::GlobalSecondaryIndex>>,
    ) -> Self {
        self.inner = self.inner.set_global_secondary_index_override(input);
        self
    }
    /// Appends an item to `LocalSecondaryIndexOverride`.
    ///
    /// To override the contents of this collection use [`set_local_secondary_index_override`](Self::set_local_secondary_index_override).
    ///
    /// <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn local_secondary_index_override(
        mut self,
        input: crate::types::LocalSecondaryIndex,
    ) -> Self {
        self.inner = self.inner.local_secondary_index_override(input);
        self
    }
    /// <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn set_local_secondary_index_override(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::LocalSecondaryIndex>>,
    ) -> Self {
        self.inner = self.inner.set_local_secondary_index_override(input);
        self
    }
    /// <p>Provisioned throughput settings for the restored table.</p>
    pub fn provisioned_throughput_override(
        mut self,
        input: crate::types::ProvisionedThroughput,
    ) -> Self {
        self.inner = self.inner.provisioned_throughput_override(input);
        self
    }
    /// <p>Provisioned throughput settings for the restored table.</p>
    pub fn set_provisioned_throughput_override(
        mut self,
        input: std::option::Option<crate::types::ProvisionedThroughput>,
    ) -> Self {
        self.inner = self.inner.set_provisioned_throughput_override(input);
        self
    }
    /// <p>The new server-side encryption settings for the restored table.</p>
    pub fn sse_specification_override(mut self, input: crate::types::SseSpecification) -> Self {
        self.inner = self.inner.sse_specification_override(input);
        self
    }
    /// <p>The new server-side encryption settings for the restored table.</p>
    pub fn set_sse_specification_override(
        mut self,
        input: std::option::Option<crate::types::SseSpecification>,
    ) -> Self {
        self.inner = self.inner.set_sse_specification_override(input);
        self
    }
}
