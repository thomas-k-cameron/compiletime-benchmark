// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PurchaseProvisionedCapacity`](crate::operation::purchase_provisioned_capacity::builders::PurchaseProvisionedCapacityFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::purchase_provisioned_capacity::builders::PurchaseProvisionedCapacityFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::purchase_provisioned_capacity::builders::PurchaseProvisionedCapacityFluentBuilder::set_account_id): <p>The AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '-' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, don't include any hyphens ('-') in the ID. </p>
    /// - On success, responds with [`PurchaseProvisionedCapacityOutput`](crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityOutput) with field(s):
    ///   - [`capacity_id(Option<String>)`](crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityOutput::capacity_id): <p>The ID that identifies the provisioned capacity unit.</p>
    /// - On failure, responds with [`SdkError<PurchaseProvisionedCapacityError>`](crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityError)
    pub fn purchase_provisioned_capacity(&self) -> crate::operation::purchase_provisioned_capacity::builders::PurchaseProvisionedCapacityFluentBuilder{
        crate::operation::purchase_provisioned_capacity::builders::PurchaseProvisionedCapacityFluentBuilder::new(self.handle.clone())
    }
}
