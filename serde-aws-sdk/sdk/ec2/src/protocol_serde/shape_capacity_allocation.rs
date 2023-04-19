// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_capacity_allocation(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::CapacityAllocation, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::CapacityAllocation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("allocationType") /* AllocationType com.amazonaws.ec2#CapacityAllocation$AllocationType */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::AllocationType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::AllocationType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_allocation_type(var_1);
            }
            ,
            s if s.matches("count") /* Count com.amazonaws.ec2#CapacityAllocation$Count */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_count(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
