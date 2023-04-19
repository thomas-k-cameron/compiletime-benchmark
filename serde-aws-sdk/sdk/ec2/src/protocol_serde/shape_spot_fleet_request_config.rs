// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_spot_fleet_request_config(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::SpotFleetRequestConfig, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SpotFleetRequestConfig::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("activityStatus") /* ActivityStatus com.amazonaws.ec2#SpotFleetRequestConfig$ActivityStatus */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::ActivityStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ActivityStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_activity_status(var_1);
            }
            ,
            s if s.matches("createTime") /* CreateTime com.amazonaws.ec2#SpotFleetRequestConfig$CreateTime */ =>  {
                let var_2 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_create_time(var_2);
            }
            ,
            s if s.matches("spotFleetRequestConfig") /* SpotFleetRequestConfig com.amazonaws.ec2#SpotFleetRequestConfig$SpotFleetRequestConfig */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_spot_fleet_request_config_data::de_spot_fleet_request_config_data(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_spot_fleet_request_config(var_3);
            }
            ,
            s if s.matches("spotFleetRequestId") /* SpotFleetRequestId com.amazonaws.ec2#SpotFleetRequestConfig$SpotFleetRequestId */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_spot_fleet_request_id(var_4);
            }
            ,
            s if s.matches("spotFleetRequestState") /* SpotFleetRequestState com.amazonaws.ec2#SpotFleetRequestConfig$SpotFleetRequestState */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::BatchState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::BatchState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_spot_fleet_request_state(var_5);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#SpotFleetRequestConfig$Tags */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}