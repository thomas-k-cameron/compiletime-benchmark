// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_verified_access_group_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupOutput,
    crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(
        crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupError::generic(
            generic,
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_verified_access_group_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupOutput,
    crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_verified_access_group::builders::ModifyVerifiedAccessGroupOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_verified_access_group::de_modify_verified_access_group(response.body().as_ref(), output).map_err(crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_verified_access_group(inp: &[u8], mut builder: crate::operation::modify_verified_access_group::builders::ModifyVerifiedAccessGroupOutputBuilder) -> Result<crate::operation::modify_verified_access_group::builders::ModifyVerifiedAccessGroupOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyVerifiedAccessGroupResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyVerifiedAccessGroupResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("verifiedAccessGroup") /* VerifiedAccessGroup com.amazonaws.ec2.synthetic#ModifyVerifiedAccessGroupOutput$VerifiedAccessGroup */ =>  {
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
