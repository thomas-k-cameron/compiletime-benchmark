// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_scheduled_instance_set(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<std::vec::Vec<crate::types::ScheduledInstance>, aws_smithy_xml::decode::XmlDecodeError>
{
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#ScheduledInstanceSet$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_scheduled_instance::de_scheduled_instance(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
