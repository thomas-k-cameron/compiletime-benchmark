// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_phase1_integrity_algorithms_list_value(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Phase1IntegrityAlgorithmsListValue, aws_smithy_xml::decode::XmlDecodeError>
{
    #[allow(unused_mut)]
    let mut builder = crate::types::Phase1IntegrityAlgorithmsListValue::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("value") /* Value com.amazonaws.ec2#Phase1IntegrityAlgorithmsListValue$Value */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_value(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
