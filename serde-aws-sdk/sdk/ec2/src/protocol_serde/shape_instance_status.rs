// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_instance_status(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InstanceStatus, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InstanceStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#InstanceStatus$AvailabilityZone */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_1);
            }
            ,
            s if s.matches("outpostArn") /* OutpostArn com.amazonaws.ec2#InstanceStatus$OutpostArn */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_outpost_arn(var_2);
            }
            ,
            s if s.matches("eventsSet") /* Events com.amazonaws.ec2#InstanceStatus$Events */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_instance_status_event_list::de_instance_status_event_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_events(var_3);
            }
            ,
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2#InstanceStatus$InstanceId */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_id(var_4);
            }
            ,
            s if s.matches("instanceState") /* InstanceState com.amazonaws.ec2#InstanceStatus$InstanceState */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_instance_state::de_instance_state(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_state(var_5);
            }
            ,
            s if s.matches("instanceStatus") /* InstanceStatus com.amazonaws.ec2#InstanceStatus$InstanceStatus */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_instance_status_summary::de_instance_status_summary(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_status(var_6);
            }
            ,
            s if s.matches("systemStatus") /* SystemStatus com.amazonaws.ec2#InstanceStatus$SystemStatus */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_instance_status_summary::de_instance_status_summary(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_system_status(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
