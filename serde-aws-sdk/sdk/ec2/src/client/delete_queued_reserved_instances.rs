// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteQueuedReservedInstances`](crate::operation::delete_queued_reserved_instances::builders::DeleteQueuedReservedInstancesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_queued_reserved_instances::builders::DeleteQueuedReservedInstancesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_queued_reserved_instances::builders::DeleteQueuedReservedInstancesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`reserved_instances_ids(Vec<String>)`](crate::operation::delete_queued_reserved_instances::builders::DeleteQueuedReservedInstancesFluentBuilder::reserved_instances_ids) / [`set_reserved_instances_ids(Option<Vec<String>>)`](crate::operation::delete_queued_reserved_instances::builders::DeleteQueuedReservedInstancesFluentBuilder::set_reserved_instances_ids): <p>The IDs of the Reserved Instances.</p>
    /// - On success, responds with [`DeleteQueuedReservedInstancesOutput`](crate::operation::delete_queued_reserved_instances::DeleteQueuedReservedInstancesOutput) with field(s):
    ///   - [`successful_queued_purchase_deletions(Option<Vec<SuccessfulQueuedPurchaseDeletion>>)`](crate::operation::delete_queued_reserved_instances::DeleteQueuedReservedInstancesOutput::successful_queued_purchase_deletions): <p>Information about the queued purchases that were successfully deleted.</p>
    ///   - [`failed_queued_purchase_deletions(Option<Vec<FailedQueuedPurchaseDeletion>>)`](crate::operation::delete_queued_reserved_instances::DeleteQueuedReservedInstancesOutput::failed_queued_purchase_deletions): <p>Information about the queued purchases that could not be deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteQueuedReservedInstancesError>`](crate::operation::delete_queued_reserved_instances::DeleteQueuedReservedInstancesError)
    pub fn delete_queued_reserved_instances(&self) -> crate::operation::delete_queued_reserved_instances::builders::DeleteQueuedReservedInstancesFluentBuilder{
        crate::operation::delete_queued_reserved_instances::builders::DeleteQueuedReservedInstancesFluentBuilder::new(self.handle.clone())
    }
}
