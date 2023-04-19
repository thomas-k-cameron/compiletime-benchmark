// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_modify_vpn_tunnel_options_specification(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::ModifyVpnTunnelOptionsSpecification,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TunnelInsideCidr");
    if let Some(var_2) = &input.tunnel_inside_cidr {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TunnelInsideIpv6Cidr");
    if let Some(var_4) = &input.tunnel_inside_ipv6_cidr {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PreSharedKey");
    if let Some(var_6) = &input.pre_shared_key {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Phase1LifetimeSeconds");
    if let Some(var_8) = &input.phase1_lifetime_seconds {
        scope_7.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Phase2LifetimeSeconds");
    if let Some(var_10) = &input.phase2_lifetime_seconds {
        scope_9.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_10).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("RekeyMarginTimeSeconds");
    if let Some(var_12) = &input.rekey_margin_time_seconds {
        scope_11.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("RekeyFuzzPercentage");
    if let Some(var_14) = &input.rekey_fuzz_percentage {
        scope_13.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("ReplayWindowSize");
    if let Some(var_16) = &input.replay_window_size {
        scope_15.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("DPDTimeoutSeconds");
    if let Some(var_18) = &input.dpd_timeout_seconds {
        scope_17.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_18).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("DPDTimeoutAction");
    if let Some(var_20) = &input.dpd_timeout_action {
        scope_19.string(var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Phase1EncryptionAlgorithm");
    if let Some(var_22) = &input.phase1_encryption_algorithms {
        let mut list_24 = scope_21.start_list(true, Some("item"));
        for item_23 in var_22 {
            #[allow(unused_mut)]
            let mut entry_25 = list_24.entry();
            crate::protocol_serde::shape_phase1_encryption_algorithms_request_list_value::ser_phase1_encryption_algorithms_request_list_value(entry_25, item_23)?;
        }
        list_24.finish();
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("Phase2EncryptionAlgorithm");
    if let Some(var_27) = &input.phase2_encryption_algorithms {
        let mut list_29 = scope_26.start_list(true, Some("item"));
        for item_28 in var_27 {
            #[allow(unused_mut)]
            let mut entry_30 = list_29.entry();
            crate::protocol_serde::shape_phase2_encryption_algorithms_request_list_value::ser_phase2_encryption_algorithms_request_list_value(entry_30, item_28)?;
        }
        list_29.finish();
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("Phase1IntegrityAlgorithm");
    if let Some(var_32) = &input.phase1_integrity_algorithms {
        let mut list_34 = scope_31.start_list(true, Some("item"));
        for item_33 in var_32 {
            #[allow(unused_mut)]
            let mut entry_35 = list_34.entry();
            crate::protocol_serde::shape_phase1_integrity_algorithms_request_list_value::ser_phase1_integrity_algorithms_request_list_value(entry_35, item_33)?;
        }
        list_34.finish();
    }
    #[allow(unused_mut)]
    let mut scope_36 = writer.prefix("Phase2IntegrityAlgorithm");
    if let Some(var_37) = &input.phase2_integrity_algorithms {
        let mut list_39 = scope_36.start_list(true, Some("item"));
        for item_38 in var_37 {
            #[allow(unused_mut)]
            let mut entry_40 = list_39.entry();
            crate::protocol_serde::shape_phase2_integrity_algorithms_request_list_value::ser_phase2_integrity_algorithms_request_list_value(entry_40, item_38)?;
        }
        list_39.finish();
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("Phase1DHGroupNumber");
    if let Some(var_42) = &input.phase1_dh_group_numbers {
        let mut list_44 = scope_41.start_list(true, Some("item"));
        for item_43 in var_42 {
            #[allow(unused_mut)]
            let mut entry_45 = list_44.entry();
            crate::protocol_serde::shape_phase1_dh_group_numbers_request_list_value::ser_phase1_dh_group_numbers_request_list_value(entry_45, item_43)?;
        }
        list_44.finish();
    }
    #[allow(unused_mut)]
    let mut scope_46 = writer.prefix("Phase2DHGroupNumber");
    if let Some(var_47) = &input.phase2_dh_group_numbers {
        let mut list_49 = scope_46.start_list(true, Some("item"));
        for item_48 in var_47 {
            #[allow(unused_mut)]
            let mut entry_50 = list_49.entry();
            crate::protocol_serde::shape_phase2_dh_group_numbers_request_list_value::ser_phase2_dh_group_numbers_request_list_value(entry_50, item_48)?;
        }
        list_49.finish();
    }
    #[allow(unused_mut)]
    let mut scope_51 = writer.prefix("IKEVersion");
    if let Some(var_52) = &input.ike_versions {
        let mut list_54 = scope_51.start_list(true, Some("item"));
        for item_53 in var_52 {
            #[allow(unused_mut)]
            let mut entry_55 = list_54.entry();
            crate::protocol_serde::shape_ike_versions_request_list_value::ser_ike_versions_request_list_value(entry_55, item_53)?;
        }
        list_54.finish();
    }
    #[allow(unused_mut)]
    let mut scope_56 = writer.prefix("StartupAction");
    if let Some(var_57) = &input.startup_action {
        scope_56.string(var_57);
    }
    #[allow(unused_mut)]
    let mut scope_58 = writer.prefix("LogOptions");
    if let Some(var_59) = &input.log_options {
        crate::protocol_serde::shape_vpn_tunnel_log_options_specification::ser_vpn_tunnel_log_options_specification(scope_58, var_59)?;
    }
    Ok(())
}
