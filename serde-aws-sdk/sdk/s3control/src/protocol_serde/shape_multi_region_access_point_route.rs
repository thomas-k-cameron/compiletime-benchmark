// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_multi_region_access_point_route(
    input: &crate::types::MultiRegionAccessPointRoute,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.bucket {
        let mut inner_writer = scope.start_el("Bucket").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.region {
        let mut inner_writer = scope.start_el("Region").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.traffic_dial_percentage {
        let mut inner_writer = scope.start_el("TrafficDialPercentage").finish();
        inner_writer.data(aws_smithy_types::primitive::Encoder::from(*var_3).encode());
    }
    scope.finish();
    Ok(())
}

pub fn de_multi_region_access_point_route(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::MultiRegionAccessPointRoute, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::MultiRegionAccessPointRoute::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Bucket") /* Bucket com.amazonaws.s3control#MultiRegionAccessPointRoute$Bucket */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket(var_4);
            }
            ,
            s if s.matches("Region") /* Region com.amazonaws.s3control#MultiRegionAccessPointRoute$Region */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_region(var_5);
            }
            ,
            s if s.matches("TrafficDialPercentage") /* TrafficDialPercentage com.amazonaws.s3control#MultiRegionAccessPointRoute$TrafficDialPercentage */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3control#TrafficDialPercentage`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_traffic_dial_percentage(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
