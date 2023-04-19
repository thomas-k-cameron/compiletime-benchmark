// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyInstanceCreditSpecification`](crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationFluentBuilder::set_client_token): <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    ///   - [`instance_credit_specifications(Vec<InstanceCreditSpecificationRequest>)`](crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationFluentBuilder::instance_credit_specifications) / [`set_instance_credit_specifications(Option<Vec<InstanceCreditSpecificationRequest>>)`](crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationFluentBuilder::set_instance_credit_specifications): <p>Information about the credit option for CPU usage.</p>
    /// - On success, responds with [`ModifyInstanceCreditSpecificationOutput`](crate::operation::modify_instance_credit_specification::ModifyInstanceCreditSpecificationOutput) with field(s):
    ///   - [`successful_instance_credit_specifications(Option<Vec<SuccessfulInstanceCreditSpecificationItem>>)`](crate::operation::modify_instance_credit_specification::ModifyInstanceCreditSpecificationOutput::successful_instance_credit_specifications): <p>Information about the instances whose credit option for CPU usage was successfully modified.</p>
    ///   - [`unsuccessful_instance_credit_specifications(Option<Vec<UnsuccessfulInstanceCreditSpecificationItem>>)`](crate::operation::modify_instance_credit_specification::ModifyInstanceCreditSpecificationOutput::unsuccessful_instance_credit_specifications): <p>Information about the instances whose credit option for CPU usage was not modified.</p>
    /// - On failure, responds with [`SdkError<ModifyInstanceCreditSpecificationError>`](crate::operation::modify_instance_credit_specification::ModifyInstanceCreditSpecificationError)
    pub fn modify_instance_credit_specification(&self) -> crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationFluentBuilder{
        crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationFluentBuilder::new(self.handle.clone())
    }
}
