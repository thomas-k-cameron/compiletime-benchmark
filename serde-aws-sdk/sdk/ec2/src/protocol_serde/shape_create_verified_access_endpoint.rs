// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_verified_access_endpoint_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointOutput,
    crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_verified_access_endpoint_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointOutput,
    crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_verified_access_endpoint::builders::CreateVerifiedAccessEndpointOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_verified_access_endpoint::de_create_verified_access_endpoint(response.body().as_ref(), output).map_err(crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_verified_access_endpoint(inp: &[u8], mut builder: crate::operation::create_verified_access_endpoint::builders::CreateVerifiedAccessEndpointOutputBuilder) -> Result<crate::operation::create_verified_access_endpoint::builders::CreateVerifiedAccessEndpointOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateVerifiedAccessEndpointResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateVerifiedAccessEndpointResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("verifiedAccessEndpoint") /* VerifiedAccessEndpoint com.amazonaws.ec2.synthetic#CreateVerifiedAccessEndpointOutput$VerifiedAccessEndpoint */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_verified_access_endpoint::de_verified_access_endpoint(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_verified_access_endpoint(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}