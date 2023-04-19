// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_network_card_info(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::NetworkCardInfo, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::NetworkCardInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("networkCardIndex") /* NetworkCardIndex com.amazonaws.ec2#NetworkCardInfo$NetworkCardIndex */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#NetworkCardIndex`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_network_card_index(var_1);
            }
            ,
            s if s.matches("networkPerformance") /* NetworkPerformance com.amazonaws.ec2#NetworkCardInfo$NetworkPerformance */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_performance(var_2);
            }
            ,
            s if s.matches("maximumNetworkInterfaces") /* MaximumNetworkInterfaces com.amazonaws.ec2#NetworkCardInfo$MaximumNetworkInterfaces */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#MaxNetworkInterfaces`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_maximum_network_interfaces(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
