// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_address_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::associate_address::AssociateAddressOutput,
    crate::operation::associate_address::AssociateAddressError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::associate_address::AssociateAddressError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::associate_address::AssociateAddressError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_address_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::associate_address::AssociateAddressOutput,
    crate::operation::associate_address::AssociateAddressError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::associate_address::builders::AssociateAddressOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_associate_address::de_associate_address(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::associate_address::AssociateAddressError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_associate_address(
    inp: &[u8],
    mut builder: crate::operation::associate_address::builders::AssociateAddressOutputBuilder,
) -> Result<
    crate::operation::associate_address::builders::AssociateAddressOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("AssociateAddressResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected AssociateAddressResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("associationId") /* AssociationId com.amazonaws.ec2.synthetic#AssociateAddressOutput$AssociationId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_association_id(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
