// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_verified_access_trust_provider_condensed_list(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    std::vec::Vec<crate::types::VerifiedAccessTrustProviderCondensed>,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#VerifiedAccessTrustProviderCondensedList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_verified_access_trust_provider_condensed::de_verified_access_trust_provider_condensed(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
