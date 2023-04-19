// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The entire transaction request was canceled.</p>
/// <p>DynamoDB cancels a <code>TransactWriteItems</code> request under the following circumstances:</p>
/// <ul>
/// <li> <p>A condition in one of the condition expressions is not met.</p> </li>
/// <li> <p>A table in the <code>TransactWriteItems</code> request is in a different account or region.</p> </li>
/// <li> <p>More than one action in the <code>TransactWriteItems</code> operation targets the same item.</p> </li>
/// <li> <p>There is insufficient provisioned capacity for the transaction to be completed.</p> </li>
/// <li> <p>An item size becomes too large (larger than 400 KB), or a local secondary index (LSI) becomes too large, or a similar validation error occurs because of changes made by the transaction.</p> </li>
/// <li> <p>There is a user error, such as an invalid data format.</p> </li>
/// </ul>
/// <p>DynamoDB cancels a <code>TransactGetItems</code> request under the following circumstances:</p>
/// <ul>
/// <li> <p>There is an ongoing <code>TransactGetItems</code> operation that conflicts with a concurrent <code>PutItem</code>, <code>UpdateItem</code>, <code>DeleteItem</code> or <code>TransactWriteItems</code> request. In this case the <code>TransactGetItems</code> operation fails with a <code>TransactionCanceledException</code>.</p> </li>
/// <li> <p>A table in the <code>TransactGetItems</code> request is in a different account or region.</p> </li>
/// <li> <p>There is insufficient provisioned capacity for the transaction to be completed.</p> </li>
/// <li> <p>There is a user error, such as an invalid data format.</p> </li>
/// </ul> <note>
/// <p>If using Java, DynamoDB lists the cancellation reasons on the <code>CancellationReasons</code> property. This property is not set for other languages. Transaction cancellation reasons are ordered in the order of requested items, if an item has no error it will have <code>None</code> code and <code>Null</code> message.</p>
/// </note>
/// <p>Cancellation reason codes and possible error messages:</p>
/// <ul>
/// <li> <p>No Errors:</p>
/// <ul>
/// <li> <p>Code: <code>None</code> </p> </li>
/// <li> <p>Message: <code>null</code> </p> </li>
/// </ul> </li>
/// <li> <p>Conditional Check Failed:</p>
/// <ul>
/// <li> <p>Code: <code>ConditionalCheckFailed</code> </p> </li>
/// <li> <p>Message: The conditional request failed. </p> </li>
/// </ul> </li>
/// <li> <p>Item Collection Size Limit Exceeded:</p>
/// <ul>
/// <li> <p>Code: <code>ItemCollectionSizeLimitExceeded</code> </p> </li>
/// <li> <p>Message: Collection size exceeded.</p> </li>
/// </ul> </li>
/// <li> <p>Transaction Conflict:</p>
/// <ul>
/// <li> <p>Code: <code>TransactionConflict</code> </p> </li>
/// <li> <p>Message: Transaction is ongoing for the item.</p> </li>
/// </ul> </li>
/// <li> <p>Provisioned Throughput Exceeded:</p>
/// <ul>
/// <li> <p>Code: <code>ProvisionedThroughputExceeded</code> </p> </li>
/// <li> <p>Messages:</p>
/// <ul>
/// <li> <p>The level of configured provisioned throughput for the table was exceeded. Consider increasing your provisioning level with the UpdateTable API.</p> <note>
/// <p>This Message is received when provisioned throughput is exceeded is on a provisioned DynamoDB table.</p>
/// </note> </li>
/// <li> <p>The level of configured provisioned throughput for one or more global secondary indexes of the table was exceeded. Consider increasing your provisioning level for the under-provisioned global secondary indexes with the UpdateTable API.</p> <note>
/// <p>This message is returned when provisioned throughput is exceeded is on a provisioned GSI.</p>
/// </note> </li>
/// </ul> </li>
/// </ul> </li>
/// <li> <p>Throttling Error:</p>
/// <ul>
/// <li> <p>Code: <code>ThrottlingError</code> </p> </li>
/// <li> <p>Messages: </p>
/// <ul>
/// <li> <p>Throughput exceeds the current capacity of your table or index. DynamoDB is automatically scaling your table or index so please try again shortly. If exceptions persist, check if you have a hot key: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/bp-partition-key-design.html.</p> <note>
/// <p>This message is returned when writes get throttled on an On-Demand table as DynamoDB is automatically scaling the table.</p>
/// </note> </li>
/// <li> <p>Throughput exceeds the current capacity for one or more global secondary indexes. DynamoDB is automatically scaling your index so please try again shortly.</p> <note>
/// <p>This message is returned when writes get throttled on an On-Demand GSI as DynamoDB is automatically scaling the GSI.</p>
/// </note> </li>
/// </ul> </li>
/// </ul> </li>
/// <li> <p>Validation Error:</p>
/// <ul>
/// <li> <p>Code: <code>ValidationError</code> </p> </li>
/// <li> <p>Messages: </p>
/// <ul>
/// <li> <p>One or more parameter values were invalid.</p> </li>
/// <li> <p>The update expression attempted to update the secondary index key beyond allowed size limits.</p> </li>
/// <li> <p>The update expression attempted to update the secondary index key to unsupported type.</p> </li>
/// <li> <p>An operand in the update expression has an incorrect data type.</p> </li>
/// <li> <p>Item size to update has exceeded the maximum allowed size.</p> </li>
/// <li> <p>Number overflow. Attempting to store a number with magnitude larger than supported range.</p> </li>
/// <li> <p>Type mismatch for attribute to update.</p> </li>
/// <li> <p>Nesting Levels have exceeded supported limits.</p> </li>
/// <li> <p>The document path provided in the update expression is invalid for update.</p> </li>
/// <li> <p>The provided expression refers to an attribute that does not exist in the item.</p> </li>
/// </ul> </li>
/// </ul> </li>
/// </ul>
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
pub struct TransactionCanceledException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    /// <p>A list of cancellation reasons.</p>
    #[doc(hidden)]
    pub cancellation_reasons: std::option::Option<std::vec::Vec<crate::types::CancellationReason>>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl TransactionCanceledException {
    /// <p>A list of cancellation reasons.</p>
    pub fn cancellation_reasons(&self) -> std::option::Option<&[crate::types::CancellationReason]> {
        self.cancellation_reasons.as_deref()
    }
}
impl TransactionCanceledException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for TransactionCanceledException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TransactionCanceledException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for TransactionCanceledException {}
impl aws_http::request_id::RequestId for crate::types::error::TransactionCanceledException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for TransactionCanceledException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl TransactionCanceledException {
    /// Creates a new builder-style object to manufacture [`TransactionCanceledException`](crate::types::error::TransactionCanceledException).
    pub fn builder() -> crate::types::error::builders::TransactionCanceledExceptionBuilder {
        crate::types::error::builders::TransactionCanceledExceptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::TransactionCanceledException;
/// A builder for [`TransactionCanceledException`](crate::types::error::TransactionCanceledException).
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
pub struct TransactionCanceledExceptionBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    pub(crate) cancellation_reasons:
        std::option::Option<std::vec::Vec<crate::types::CancellationReason>>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl TransactionCanceledExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Appends an item to `cancellation_reasons`.
    ///
    /// To override the contents of this collection use [`set_cancellation_reasons`](Self::set_cancellation_reasons).
    ///
    /// <p>A list of cancellation reasons.</p>
    pub fn cancellation_reasons(mut self, input: crate::types::CancellationReason) -> Self {
        let mut v = self.cancellation_reasons.unwrap_or_default();
        v.push(input);
        self.cancellation_reasons = Some(v);
        self
    }
    /// <p>A list of cancellation reasons.</p>
    pub fn set_cancellation_reasons(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::CancellationReason>>,
    ) -> Self {
        self.cancellation_reasons = input;
        self
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(
        &mut self,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`TransactionCanceledException`](crate::types::error::TransactionCanceledException).
    pub fn build(self) -> crate::types::error::TransactionCanceledException {
        crate::types::error::TransactionCanceledException {
            message: self.message,
            cancellation_reasons: self.cancellation_reasons,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
