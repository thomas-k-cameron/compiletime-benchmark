// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVerifiedAccessInstance`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::set_description): <p>A description for the Amazon Web Services Verified Access instance.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::set_tag_specifications): <p>The tags to assign to the Amazon Web Services Verified Access instance.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::set_client_token): <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`CreateVerifiedAccessInstanceOutput`](crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceOutput) with field(s):
    ///   - [`verified_access_instance(Option<VerifiedAccessInstance>)`](crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceOutput::verified_access_instance): <p>The ID of the Amazon Web Services Verified Access instance.</p>
    /// - On failure, responds with [`SdkError<CreateVerifiedAccessInstanceError>`](crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceError)
    pub fn create_verified_access_instance(&self) -> crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder{
        crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceFluentBuilder::new(self.handle.clone())
    }
}
