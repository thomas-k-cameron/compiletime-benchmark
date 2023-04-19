// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vpc_ipv6_cidr_block_association(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::VpcIpv6CidrBlockAssociation, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VpcIpv6CidrBlockAssociation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("associationId") /* AssociationId com.amazonaws.ec2#VpcIpv6CidrBlockAssociation$AssociationId */ =>  {
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
            s if s.matches("ipv6CidrBlock") /* Ipv6CidrBlock com.amazonaws.ec2#VpcIpv6CidrBlockAssociation$Ipv6CidrBlock */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipv6_cidr_block(var_2);
            }
            ,
            s if s.matches("ipv6CidrBlockState") /* Ipv6CidrBlockState com.amazonaws.ec2#VpcIpv6CidrBlockAssociation$Ipv6CidrBlockState */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_vpc_cidr_block_state::de_vpc_cidr_block_state(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv6_cidr_block_state(var_3);
            }
            ,
            s if s.matches("networkBorderGroup") /* NetworkBorderGroup com.amazonaws.ec2#VpcIpv6CidrBlockAssociation$NetworkBorderGroup */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_border_group(var_4);
            }
            ,
            s if s.matches("ipv6Pool") /* Ipv6Pool com.amazonaws.ec2#VpcIpv6CidrBlockAssociation$Ipv6Pool */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipv6_pool(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
