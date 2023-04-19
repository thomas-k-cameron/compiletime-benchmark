// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_verified_access_trust_provider_input_input(
    input: &crate::operation::modify_verified_access_trust_provider::ModifyVerifiedAccessTrustProviderInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(
        &mut out,
        "ModifyVerifiedAccessTrustProvider",
        "2016-11-15",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("VerifiedAccessTrustProviderId");
    if let Some(var_2) = &input.verified_access_trust_provider_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("OidcOptions");
    if let Some(var_4) = &input.oidc_options {
        crate::protocol_serde::shape_modify_verified_access_trust_provider_oidc_options::ser_modify_verified_access_trust_provider_oidc_options(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Description");
    if let Some(var_6) = &input.description {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DryRun");
    if let Some(var_8) = &input.dry_run {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("ClientToken");
    if let Some(var_10) = &input.client_token {
        scope_9.string(var_10);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
