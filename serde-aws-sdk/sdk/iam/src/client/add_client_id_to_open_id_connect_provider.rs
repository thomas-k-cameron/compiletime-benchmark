// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddClientIDToOpenIDConnectProvider`](crate::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIDToOpenIDConnectProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`open_id_connect_provider_arn(impl Into<String>)`](crate::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIDToOpenIDConnectProviderFluentBuilder::open_id_connect_provider_arn) / [`set_open_id_connect_provider_arn(Option<String>)`](crate::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIDToOpenIDConnectProviderFluentBuilder::set_open_id_connect_provider_arn): <p>The Amazon Resource Name (ARN) of the IAM OpenID Connect (OIDC) provider resource to add the client ID to. You can get a list of OIDC provider ARNs by using the <code>ListOpenIDConnectProviders</code> operation.</p>
    ///   - [`client_id(impl Into<String>)`](crate::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIDToOpenIDConnectProviderFluentBuilder::client_id) / [`set_client_id(Option<String>)`](crate::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIDToOpenIDConnectProviderFluentBuilder::set_client_id): <p>The client ID (also known as audience) to add to the IAM OpenID Connect provider resource.</p>
    /// - On success, responds with [`AddClientIdToOpenIdConnectProviderOutput`](crate::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderOutput)
    /// - On failure, responds with [`SdkError<AddClientIDToOpenIDConnectProviderError>`](crate::operation::add_client_id_to_open_id_connect_provider::AddClientIDToOpenIDConnectProviderError)
    pub fn add_client_id_to_open_id_connect_provider(&self) -> crate::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIDToOpenIDConnectProviderFluentBuilder{
        crate::operation::add_client_id_to_open_id_connect_provider::builders::AddClientIDToOpenIDConnectProviderFluentBuilder::new(self.handle.clone())
    }
}
