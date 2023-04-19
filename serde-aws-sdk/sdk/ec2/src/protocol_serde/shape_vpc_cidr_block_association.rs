// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vpc_cidr_block_association(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::VpcCidrBlockAssociation, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VpcCidrBlockAssociation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("associationId") /* AssociationId com.amazonaws.ec2#VpcCidrBlockAssociation$AssociationId */ =>  {
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
            s if s.matches("cidrBlock") /* CidrBlock com.amazonaws.ec2#VpcCidrBlockAssociation$CidrBlock */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cidr_block(var_2);
            }
            ,
            s if s.matches("cidrBlockState") /* CidrBlockState com.amazonaws.ec2#VpcCidrBlockAssociation$CidrBlockState */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_vpc_cidr_block_state::de_vpc_cidr_block_state(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cidr_block_state(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
