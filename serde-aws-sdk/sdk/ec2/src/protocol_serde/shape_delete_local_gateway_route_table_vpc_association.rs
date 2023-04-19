// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_local_gateway_route_table_vpc_association_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::delete_local_gateway_route_table_vpc_association::DeleteLocalGatewayRouteTableVpcAssociationOutput, crate::operation::delete_local_gateway_route_table_vpc_association::DeleteLocalGatewayRouteTableVpcAssociationError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_local_gateway_route_table_vpc_association::DeleteLocalGatewayRouteTableVpcAssociationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::delete_local_gateway_route_table_vpc_association::DeleteLocalGatewayRouteTableVpcAssociationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_local_gateway_route_table_vpc_association_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::delete_local_gateway_route_table_vpc_association::DeleteLocalGatewayRouteTableVpcAssociationOutput, crate::operation::delete_local_gateway_route_table_vpc_association::DeleteLocalGatewayRouteTableVpcAssociationError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_local_gateway_route_table_vpc_association::builders::DeleteLocalGatewayRouteTableVpcAssociationOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_local_gateway_route_table_vpc_association::de_delete_local_gateway_route_table_vpc_association(response.body().as_ref(), output).map_err(crate::operation::delete_local_gateway_route_table_vpc_association::DeleteLocalGatewayRouteTableVpcAssociationError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_local_gateway_route_table_vpc_association(inp: &[u8], mut builder: crate::operation::delete_local_gateway_route_table_vpc_association::builders::DeleteLocalGatewayRouteTableVpcAssociationOutputBuilder) -> Result<crate::operation::delete_local_gateway_route_table_vpc_association::builders::DeleteLocalGatewayRouteTableVpcAssociationOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteLocalGatewayRouteTableVpcAssociationResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteLocalGatewayRouteTableVpcAssociationResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("localGatewayRouteTableVpcAssociation") /* LocalGatewayRouteTableVpcAssociation com.amazonaws.ec2.synthetic#DeleteLocalGatewayRouteTableVpcAssociationOutput$LocalGatewayRouteTableVpcAssociation */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_local_gateway_route_table_vpc_association::de_local_gateway_route_table_vpc_association(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_local_gateway_route_table_vpc_association(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
