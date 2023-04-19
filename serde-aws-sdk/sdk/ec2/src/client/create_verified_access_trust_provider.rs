// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVerifiedAccessTrustProvider`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`trust_provider_type(TrustProviderType)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::trust_provider_type) / [`set_trust_provider_type(Option<TrustProviderType>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_trust_provider_type): <p>The type of trust provider can be either user or device-based.</p>
    ///   - [`user_trust_provider_type(UserTrustProviderType)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::user_trust_provider_type) / [`set_user_trust_provider_type(Option<UserTrustProviderType>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_user_trust_provider_type): <p>The type of user-based trust provider.</p>
    ///   - [`device_trust_provider_type(DeviceTrustProviderType)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::device_trust_provider_type) / [`set_device_trust_provider_type(Option<DeviceTrustProviderType>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_device_trust_provider_type): <p>The type of device-based trust provider.</p>
    ///   - [`oidc_options(CreateVerifiedAccessTrustProviderOidcOptions)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::oidc_options) / [`set_oidc_options(Option<CreateVerifiedAccessTrustProviderOidcOptions>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_oidc_options): <p>The OpenID Connect details for an <code>oidc</code>-type, user-identity based trust provider.</p>
    ///   - [`device_options(CreateVerifiedAccessTrustProviderDeviceOptions)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::device_options) / [`set_device_options(Option<CreateVerifiedAccessTrustProviderDeviceOptions>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_device_options): <p>The options for device identity based trust providers.</p>
    ///   - [`policy_reference_name(impl Into<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::policy_reference_name) / [`set_policy_reference_name(Option<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_policy_reference_name): <p>The identifier to be used when working with policy rules.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_description): <p>A description for the Amazon Web Services Verified Access trust provider.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_tag_specifications): <p>The tags to assign to the Amazon Web Services Verified Access trust provider.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_client_token): <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`CreateVerifiedAccessTrustProviderOutput`](crate::operation::create_verified_access_trust_provider::CreateVerifiedAccessTrustProviderOutput) with field(s):
    ///   - [`verified_access_trust_provider(Option<VerifiedAccessTrustProvider>)`](crate::operation::create_verified_access_trust_provider::CreateVerifiedAccessTrustProviderOutput::verified_access_trust_provider): <p>The ID of the Amazon Web Services Verified Access trust provider.</p>
    /// - On failure, responds with [`SdkError<CreateVerifiedAccessTrustProviderError>`](crate::operation::create_verified_access_trust_provider::CreateVerifiedAccessTrustProviderError)
    pub fn create_verified_access_trust_provider(&self) -> crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder{
        crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::new(self.handle.clone())
    }
}
