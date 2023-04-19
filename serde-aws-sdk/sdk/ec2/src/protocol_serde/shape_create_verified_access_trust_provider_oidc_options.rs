// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_create_verified_access_trust_provider_oidc_options(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::CreateVerifiedAccessTrustProviderOidcOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Issuer");
    if let Some(var_2) = &input.issuer {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AuthorizationEndpoint");
    if let Some(var_4) = &input.authorization_endpoint {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TokenEndpoint");
    if let Some(var_6) = &input.token_endpoint {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("UserInfoEndpoint");
    if let Some(var_8) = &input.user_info_endpoint {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("ClientId");
    if let Some(var_10) = &input.client_id {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("ClientSecret");
    if let Some(var_12) = &input.client_secret {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("Scope");
    if let Some(var_14) = &input.scope {
        scope_13.string(var_14);
    }
    Ok(())
}
