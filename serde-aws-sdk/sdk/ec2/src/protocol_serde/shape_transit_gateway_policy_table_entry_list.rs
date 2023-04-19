// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_transit_gateway_policy_table_entry_list(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    std::vec::Vec<crate::types::TransitGatewayPolicyTableEntry>,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#TransitGatewayPolicyTableEntryList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_transit_gateway_policy_table_entry::de_transit_gateway_policy_table_entry(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
