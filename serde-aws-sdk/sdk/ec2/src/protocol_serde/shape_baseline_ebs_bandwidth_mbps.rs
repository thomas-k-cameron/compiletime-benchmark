// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_baseline_ebs_bandwidth_mbps(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::BaselineEbsBandwidthMbps,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Min");
    if let Some(var_2) = &input.min {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Max");
    if let Some(var_4) = &input.max {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}

pub fn de_baseline_ebs_bandwidth_mbps(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::BaselineEbsBandwidthMbps, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::BaselineEbsBandwidthMbps::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("min") /* Min com.amazonaws.ec2#BaselineEbsBandwidthMbps$Min */ =>  {
                let var_5 =
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
                builder = builder.set_min(var_5);
            }
            ,
            s if s.matches("max") /* Max com.amazonaws.ec2#BaselineEbsBandwidthMbps$Max */ =>  {
                let var_6 =
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
                builder = builder.set_max(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
