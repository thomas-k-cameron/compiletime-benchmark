// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_phase2_encryption_algorithms_list(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    std::vec::Vec<crate::types::Phase2EncryptionAlgorithmsListValue>,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#Phase2EncryptionAlgorithmsList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_phase2_encryption_algorithms_list_value::de_phase2_encryption_algorithms_list_value(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
