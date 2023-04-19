// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_replication_configuration_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::types::ReplicationConfiguration>,
    crate::operation::get_bucket_replication::GetBucketReplicationError,
> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_get_bucket_replication_output::de_replication_configuration(body).map_err(crate::operation::get_bucket_replication::GetBucketReplicationError::unhandled)
    }).transpose()
}

pub fn de_replication_configuration(
    inp: &[u8],
) -> Result<crate::types::ReplicationConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("ReplicationConfiguration")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ReplicationConfiguration got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_replication_configuration::de_replication_configuration(
        &mut decoder,
    )
}
