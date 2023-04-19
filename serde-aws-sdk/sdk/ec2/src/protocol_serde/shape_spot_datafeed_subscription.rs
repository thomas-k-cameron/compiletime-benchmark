// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_spot_datafeed_subscription(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::SpotDatafeedSubscription, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SpotDatafeedSubscription::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("bucket") /* Bucket com.amazonaws.ec2#SpotDatafeedSubscription$Bucket */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket(var_1);
            }
            ,
            s if s.matches("fault") /* Fault com.amazonaws.ec2#SpotDatafeedSubscription$Fault */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_spot_instance_state_fault::de_spot_instance_state_fault(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_fault(var_2);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#SpotDatafeedSubscription$OwnerId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_3);
            }
            ,
            s if s.matches("prefix") /* Prefix com.amazonaws.ec2#SpotDatafeedSubscription$Prefix */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_4);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#SpotDatafeedSubscription$State */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::DatafeedSubscriptionState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DatafeedSubscriptionState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
