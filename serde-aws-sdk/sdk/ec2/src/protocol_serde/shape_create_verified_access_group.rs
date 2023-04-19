// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_verified_access_group_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_verified_access_group::CreateVerifiedAccessGroupOutput,
    crate::operation::create_verified_access_group::CreateVerifiedAccessGroupError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::create_verified_access_group::CreateVerifiedAccessGroupError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(
        crate::operation::create_verified_access_group::CreateVerifiedAccessGroupError::generic(
            generic,
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_verified_access_group_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_verified_access_group::CreateVerifiedAccessGroupOutput,
    crate::operation::create_verified_access_group::CreateVerifiedAccessGroupError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_verified_access_group::de_create_verified_access_group(response.body().as_ref(), output).map_err(crate::operation::create_verified_access_group::CreateVerifiedAccessGroupError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_verified_access_group(inp: &[u8], mut builder: crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupOutputBuilder) -> Result<crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateVerifiedAccessGroupResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateVerifiedAccessGroupResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("verifiedAccessGroup") /* VerifiedAccessGroup com.amazonaws.ec2.synthetic#CreateVerifiedAccessGroupOutput$VerifiedAccessGroup */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_verified_access_group::de_verified_access_group(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_verified_access_group(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
