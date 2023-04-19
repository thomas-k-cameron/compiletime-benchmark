// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_verified_access_trust_provider_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput, crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_verified_access_trust_provider_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput, crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_verified_access_trust_provider::de_delete_verified_access_trust_provider(response.body().as_ref(), output).map_err(crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_verified_access_trust_provider(inp: &[u8], mut builder: crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderOutputBuilder) -> Result<crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteVerifiedAccessTrustProviderResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteVerifiedAccessTrustProviderResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("verifiedAccessTrustProvider") /* VerifiedAccessTrustProvider com.amazonaws.ec2.synthetic#DeleteVerifiedAccessTrustProviderOutput$VerifiedAccessTrustProvider */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_verified_access_trust_provider::de_verified_access_trust_provider(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_verified_access_trust_provider(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
