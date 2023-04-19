// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_health_check_observation(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::HealthCheckObservation, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::HealthCheckObservation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Region") /* Region com.amazonaws.route53#HealthCheckObservation$Region */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::HealthCheckRegion, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::HealthCheckRegion::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_region(var_1);
            }
            ,
            s if s.matches("IPAddress") /* IPAddress com.amazonaws.route53#HealthCheckObservation$IPAddress */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ip_address(var_2);
            }
            ,
            s if s.matches("StatusReport") /* StatusReport com.amazonaws.route53#HealthCheckObservation$StatusReport */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_status_report::de_status_report(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status_report(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}