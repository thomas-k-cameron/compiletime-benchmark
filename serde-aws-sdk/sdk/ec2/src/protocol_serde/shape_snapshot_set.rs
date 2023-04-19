// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_snapshot_set(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<std::vec::Vec<crate::types::SnapshotInfo>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#SnapshotSet$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_snapshot_info::de_snapshot_info(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
