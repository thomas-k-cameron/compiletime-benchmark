// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_custom_key_store_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_custom_key_store::UpdateCustomKeyStoreInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.custom_key_store_id {
        object.key("CustomKeyStoreId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.new_custom_key_store_name {
        object.key("NewCustomKeyStoreName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.key_store_password {
        object.key("KeyStorePassword").string(var_3.as_str());
    }
    if let Some(var_4) = &input.cloud_hsm_cluster_id {
        object.key("CloudHsmClusterId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.xks_proxy_uri_endpoint {
        object.key("XksProxyUriEndpoint").string(var_5.as_str());
    }
    if let Some(var_6) = &input.xks_proxy_uri_path {
        object.key("XksProxyUriPath").string(var_6.as_str());
    }
    if let Some(var_7) = &input.xks_proxy_vpc_endpoint_service_name {
        object
            .key("XksProxyVpcEndpointServiceName")
            .string(var_7.as_str());
    }
    if let Some(var_8) = &input.xks_proxy_authentication_credential {
        #[allow(unused_mut)]
        let mut object_9 = object
            .key("XksProxyAuthenticationCredential")
            .start_object();
        crate::protocol_serde::shape_xks_proxy_authentication_credential_type::ser_xks_proxy_authentication_credential_type(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.xks_proxy_connectivity {
        object.key("XksProxyConnectivity").string(var_10.as_str());
    }
    Ok(())
}
