// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vpc_cidr_block_state(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::VpcCidrBlockState, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VpcCidrBlockState::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("state") /* State com.amazonaws.ec2#VpcCidrBlockState$State */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::VpcCidrBlockStateCode, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VpcCidrBlockStateCode::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_1);
            }
            ,
            s if s.matches("statusMessage") /* StatusMessage com.amazonaws.ec2#VpcCidrBlockState$StatusMessage */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_message(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}