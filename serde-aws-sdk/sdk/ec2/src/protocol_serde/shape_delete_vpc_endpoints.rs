// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_vpc_endpoints_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsOutput,
    crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_vpc_endpoints_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsOutput,
    crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_vpc_endpoints::de_delete_vpc_endpoints(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_vpc_endpoints(
    inp: &[u8],
    mut builder: crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsOutputBuilder,
) -> Result<
    crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteVpcEndpointsResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteVpcEndpointsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("unsuccessful") /* Unsuccessful com.amazonaws.ec2.synthetic#DeleteVpcEndpointsOutput$Unsuccessful */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_unsuccessful_item_set::de_unsuccessful_item_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_unsuccessful(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
