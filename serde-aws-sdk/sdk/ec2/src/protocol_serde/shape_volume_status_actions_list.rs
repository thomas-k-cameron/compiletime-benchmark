// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_volume_status_actions_list(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<std::vec::Vec<crate::types::VolumeStatusAction>, aws_smithy_xml::decode::XmlDecodeError>
{
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#VolumeStatusActionsList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_volume_status_action::de_volume_status_action(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
