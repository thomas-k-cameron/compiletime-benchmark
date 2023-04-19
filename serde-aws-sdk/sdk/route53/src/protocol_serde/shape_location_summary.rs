// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_location_summary(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LocationSummary, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LocationSummary::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LocationName") /* LocationName com.amazonaws.route53#LocationSummary$LocationName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_location_name(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}