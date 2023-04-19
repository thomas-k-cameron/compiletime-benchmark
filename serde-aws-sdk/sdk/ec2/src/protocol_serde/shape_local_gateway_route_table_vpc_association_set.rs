// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_local_gateway_route_table_vpc_association_set(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    std::vec::Vec<crate::types::LocalGatewayRouteTableVpcAssociation>,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#LocalGatewayRouteTableVpcAssociationSet$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_local_gateway_route_table_vpc_association::de_local_gateway_route_table_vpc_association(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
