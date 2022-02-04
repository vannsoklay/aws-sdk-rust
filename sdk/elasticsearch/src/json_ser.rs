// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddTagsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("ARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tag_list {
        let mut array_3 = object.key("TagList").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_cancel_elasticsearch_service_software_update_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelElasticsearchServiceSoftwareUpdateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.domain_name {
        object.key("DomainName").string(var_6.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_elasticsearch_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateElasticsearchDomainInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.access_policies {
        object.key("AccessPolicies").string(var_7.as_str());
    }
    if let Some(var_8) = &input.advanced_options {
        let mut object_9 = object.key("AdvancedOptions").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    if let Some(var_12) = &input.advanced_security_options {
        let mut object_13 = object.key("AdvancedSecurityOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_advanced_security_options_input(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.auto_tune_options {
        let mut object_15 = object.key("AutoTuneOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_auto_tune_options_input(
            &mut object_15,
            var_14,
        )?;
        object_15.finish();
    }
    if let Some(var_16) = &input.cognito_options {
        let mut object_17 = object.key("CognitoOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_cognito_options(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.domain_endpoint_options {
        let mut object_19 = object.key("DomainEndpointOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_domain_endpoint_options(
            &mut object_19,
            var_18,
        )?;
        object_19.finish();
    }
    if let Some(var_20) = &input.domain_name {
        object.key("DomainName").string(var_20.as_str());
    }
    if let Some(var_21) = &input.ebs_options {
        let mut object_22 = object.key("EBSOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_ebs_options(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.elasticsearch_cluster_config {
        let mut object_24 = object.key("ElasticsearchClusterConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_elasticsearch_cluster_config(
            &mut object_24,
            var_23,
        )?;
        object_24.finish();
    }
    if let Some(var_25) = &input.elasticsearch_version {
        object.key("ElasticsearchVersion").string(var_25.as_str());
    }
    if let Some(var_26) = &input.encryption_at_rest_options {
        let mut object_27 = object.key("EncryptionAtRestOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_at_rest_options(
            &mut object_27,
            var_26,
        )?;
        object_27.finish();
    }
    if let Some(var_28) = &input.log_publishing_options {
        let mut object_29 = object.key("LogPublishingOptions").start_object();
        for (key_30, value_31) in var_28 {
            {
                let mut object_32 = object_29.key(key_30.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_log_publishing_option(
                    &mut object_32,
                    value_31,
                )?;
                object_32.finish();
            }
        }
        object_29.finish();
    }
    if let Some(var_33) = &input.node_to_node_encryption_options {
        let mut object_34 = object.key("NodeToNodeEncryptionOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_node_to_node_encryption_options(
            &mut object_34,
            var_33,
        )?;
        object_34.finish();
    }
    if let Some(var_35) = &input.snapshot_options {
        let mut object_36 = object.key("SnapshotOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_snapshot_options(&mut object_36, var_35)?;
        object_36.finish();
    }
    if let Some(var_37) = &input.tag_list {
        let mut array_38 = object.key("TagList").start_array();
        for item_39 in var_37 {
            {
                let mut object_40 = array_38.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_40, item_39)?;
                object_40.finish();
            }
        }
        array_38.finish();
    }
    if let Some(var_41) = &input.vpc_options {
        let mut object_42 = object.key("VPCOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_options(&mut object_42, var_41)?;
        object_42.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_outbound_cross_cluster_search_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateOutboundCrossClusterSearchConnectionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.connection_alias {
        object.key("ConnectionAlias").string(var_43.as_str());
    }
    if let Some(var_44) = &input.destination_domain_info {
        let mut object_45 = object.key("DestinationDomainInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_domain_information(
            &mut object_45,
            var_44,
        )?;
        object_45.finish();
    }
    if let Some(var_46) = &input.source_domain_info {
        let mut object_47 = object.key("SourceDomainInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_domain_information(
            &mut object_47,
            var_46,
        )?;
        object_47.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_package_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePackageInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.package_description {
        object.key("PackageDescription").string(var_48.as_str());
    }
    if let Some(var_49) = &input.package_name {
        object.key("PackageName").string(var_49.as_str());
    }
    if let Some(var_50) = &input.package_source {
        let mut object_51 = object.key("PackageSource").start_object();
        crate::json_ser::serialize_structure_crate_model_package_source(&mut object_51, var_50)?;
        object_51.finish();
    }
    if let Some(var_52) = &input.package_type {
        object.key("PackageType").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_domain_auto_tunes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDomainAutoTunesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_53) = &input.next_token {
        object.key("NextToken").string(var_53.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_elasticsearch_domains_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeElasticsearchDomainsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.domain_names {
        let mut array_55 = object.key("DomainNames").start_array();
        for item_56 in var_54 {
            {
                array_55.value().string(item_56.as_str());
            }
        }
        array_55.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_inbound_cross_cluster_search_connections_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeInboundCrossClusterSearchConnectionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_57) = &input.filters {
        let mut array_58 = object.key("Filters").start_array();
        for item_59 in var_57 {
            {
                let mut object_60 = array_58.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_60, item_59)?;
                object_60.finish();
            }
        }
        array_58.finish();
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_61) = &input.next_token {
        object.key("NextToken").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_outbound_cross_cluster_search_connections_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeOutboundCrossClusterSearchConnectionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.filters {
        let mut array_63 = object.key("Filters").start_array();
        for item_64 in var_62 {
            {
                let mut object_65 = array_63.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_65, item_64)?;
                object_65.finish();
            }
        }
        array_63.finish();
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_66) = &input.next_token {
        object.key("NextToken").string(var_66.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_packages_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribePackagesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.filters {
        let mut array_68 = object.key("Filters").start_array();
        for item_69 in var_67 {
            {
                let mut object_70 = array_68.value().start_object();
                crate::json_ser::serialize_structure_crate_model_describe_packages_filter(
                    &mut object_70,
                    item_69,
                )?;
                object_70.finish();
            }
        }
        array_68.finish();
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_71) = &input.next_token {
        object.key("NextToken").string(var_71.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_purchase_reserved_elasticsearch_instance_offering_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PurchaseReservedElasticsearchInstanceOfferingInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.instance_count != 0 {
        object.key("InstanceCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.instance_count).into()),
        );
    }
    if let Some(var_72) = &input.reservation_name {
        object.key("ReservationName").string(var_72.as_str());
    }
    if let Some(var_73) = &input.reserved_elasticsearch_instance_offering_id {
        object
            .key("ReservedElasticsearchInstanceOfferingId")
            .string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveTagsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_74) = &input.arn {
        object.key("ARN").string(var_74.as_str());
    }
    if let Some(var_75) = &input.tag_keys {
        let mut array_76 = object.key("TagKeys").start_array();
        for item_77 in var_75 {
            {
                array_76.value().string(item_77.as_str());
            }
        }
        array_76.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_elasticsearch_service_software_update_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartElasticsearchServiceSoftwareUpdateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.domain_name {
        object.key("DomainName").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_elasticsearch_domain_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateElasticsearchDomainConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.access_policies {
        object.key("AccessPolicies").string(var_79.as_str());
    }
    if let Some(var_80) = &input.advanced_options {
        let mut object_81 = object.key("AdvancedOptions").start_object();
        for (key_82, value_83) in var_80 {
            {
                object_81.key(key_82).string(value_83.as_str());
            }
        }
        object_81.finish();
    }
    if let Some(var_84) = &input.advanced_security_options {
        let mut object_85 = object.key("AdvancedSecurityOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_advanced_security_options_input(
            &mut object_85,
            var_84,
        )?;
        object_85.finish();
    }
    if let Some(var_86) = &input.auto_tune_options {
        let mut object_87 = object.key("AutoTuneOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_auto_tune_options(&mut object_87, var_86)?;
        object_87.finish();
    }
    if let Some(var_88) = &input.cognito_options {
        let mut object_89 = object.key("CognitoOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_cognito_options(&mut object_89, var_88)?;
        object_89.finish();
    }
    if let Some(var_90) = &input.domain_endpoint_options {
        let mut object_91 = object.key("DomainEndpointOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_domain_endpoint_options(
            &mut object_91,
            var_90,
        )?;
        object_91.finish();
    }
    if let Some(var_92) = &input.dry_run {
        object.key("DryRun").boolean(*var_92);
    }
    if let Some(var_93) = &input.ebs_options {
        let mut object_94 = object.key("EBSOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_ebs_options(&mut object_94, var_93)?;
        object_94.finish();
    }
    if let Some(var_95) = &input.elasticsearch_cluster_config {
        let mut object_96 = object.key("ElasticsearchClusterConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_elasticsearch_cluster_config(
            &mut object_96,
            var_95,
        )?;
        object_96.finish();
    }
    if let Some(var_97) = &input.encryption_at_rest_options {
        let mut object_98 = object.key("EncryptionAtRestOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_at_rest_options(
            &mut object_98,
            var_97,
        )?;
        object_98.finish();
    }
    if let Some(var_99) = &input.log_publishing_options {
        let mut object_100 = object.key("LogPublishingOptions").start_object();
        for (key_101, value_102) in var_99 {
            {
                let mut object_103 = object_100.key(key_101.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_log_publishing_option(
                    &mut object_103,
                    value_102,
                )?;
                object_103.finish();
            }
        }
        object_100.finish();
    }
    if let Some(var_104) = &input.node_to_node_encryption_options {
        let mut object_105 = object.key("NodeToNodeEncryptionOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_node_to_node_encryption_options(
            &mut object_105,
            var_104,
        )?;
        object_105.finish();
    }
    if let Some(var_106) = &input.snapshot_options {
        let mut object_107 = object.key("SnapshotOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_snapshot_options(
            &mut object_107,
            var_106,
        )?;
        object_107.finish();
    }
    if let Some(var_108) = &input.vpc_options {
        let mut object_109 = object.key("VPCOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_options(&mut object_109, var_108)?;
        object_109.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_package_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePackageInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_110) = &input.commit_message {
        object.key("CommitMessage").string(var_110.as_str());
    }
    if let Some(var_111) = &input.package_description {
        object.key("PackageDescription").string(var_111.as_str());
    }
    if let Some(var_112) = &input.package_id {
        object.key("PackageID").string(var_112.as_str());
    }
    if let Some(var_113) = &input.package_source {
        let mut object_114 = object.key("PackageSource").start_object();
        crate::json_ser::serialize_structure_crate_model_package_source(&mut object_114, var_113)?;
        object_114.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_upgrade_elasticsearch_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpgradeElasticsearchDomainInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_115) = &input.domain_name {
        object.key("DomainName").string(var_115.as_str());
    }
    if let Some(var_116) = &input.perform_check_only {
        object.key("PerformCheckOnly").boolean(*var_116);
    }
    if let Some(var_117) = &input.target_version {
        object.key("TargetVersion").string(var_117.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_118) = &input.key {
        object.key("Key").string(var_118.as_str());
    }
    if let Some(var_119) = &input.value {
        object.key("Value").string(var_119.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_advanced_security_options_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdvancedSecurityOptionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_120) = &input.enabled {
        object.key("Enabled").boolean(*var_120);
    }
    if let Some(var_121) = &input.internal_user_database_enabled {
        object.key("InternalUserDatabaseEnabled").boolean(*var_121);
    }
    if let Some(var_122) = &input.master_user_options {
        let mut object_123 = object.key("MasterUserOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_master_user_options(
            &mut object_123,
            var_122,
        )?;
        object_123.finish();
    }
    if let Some(var_124) = &input.saml_options {
        let mut object_125 = object.key("SAMLOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_saml_options_input(
            &mut object_125,
            var_124,
        )?;
        object_125.finish();
    }
    if let Some(var_126) = &input.anonymous_auth_enabled {
        object.key("AnonymousAuthEnabled").boolean(*var_126);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_auto_tune_options_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoTuneOptionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.desired_state {
        object.key("DesiredState").string(var_127.as_str());
    }
    if let Some(var_128) = &input.maintenance_schedules {
        let mut array_129 = object.key("MaintenanceSchedules").start_array();
        for item_130 in var_128 {
            {
                let mut object_131 = array_129.value().start_object();
                crate::json_ser::serialize_structure_crate_model_auto_tune_maintenance_schedule(
                    &mut object_131,
                    item_130,
                )?;
                object_131.finish();
            }
        }
        array_129.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cognito_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CognitoOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_132) = &input.enabled {
        object.key("Enabled").boolean(*var_132);
    }
    if let Some(var_133) = &input.user_pool_id {
        object.key("UserPoolId").string(var_133.as_str());
    }
    if let Some(var_134) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_134.as_str());
    }
    if let Some(var_135) = &input.role_arn {
        object.key("RoleArn").string(var_135.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_domain_endpoint_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DomainEndpointOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_136) = &input.enforce_https {
        object.key("EnforceHTTPS").boolean(*var_136);
    }
    if let Some(var_137) = &input.tls_security_policy {
        object.key("TLSSecurityPolicy").string(var_137.as_str());
    }
    if let Some(var_138) = &input.custom_endpoint_enabled {
        object.key("CustomEndpointEnabled").boolean(*var_138);
    }
    if let Some(var_139) = &input.custom_endpoint {
        object.key("CustomEndpoint").string(var_139.as_str());
    }
    if let Some(var_140) = &input.custom_endpoint_certificate_arn {
        object
            .key("CustomEndpointCertificateArn")
            .string(var_140.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ebs_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EbsOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_141) = &input.ebs_enabled {
        object.key("EBSEnabled").boolean(*var_141);
    }
    if let Some(var_142) = &input.volume_type {
        object.key("VolumeType").string(var_142.as_str());
    }
    if let Some(var_143) = &input.volume_size {
        object.key("VolumeSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_143).into()),
        );
    }
    if let Some(var_144) = &input.iops {
        object.key("Iops").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_144).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_elasticsearch_cluster_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ElasticsearchClusterConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_145) = &input.instance_type {
        object.key("InstanceType").string(var_145.as_str());
    }
    if let Some(var_146) = &input.instance_count {
        object.key("InstanceCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_146).into()),
        );
    }
    if let Some(var_147) = &input.dedicated_master_enabled {
        object.key("DedicatedMasterEnabled").boolean(*var_147);
    }
    if let Some(var_148) = &input.zone_awareness_enabled {
        object.key("ZoneAwarenessEnabled").boolean(*var_148);
    }
    if let Some(var_149) = &input.zone_awareness_config {
        let mut object_150 = object.key("ZoneAwarenessConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_zone_awareness_config(
            &mut object_150,
            var_149,
        )?;
        object_150.finish();
    }
    if let Some(var_151) = &input.dedicated_master_type {
        object.key("DedicatedMasterType").string(var_151.as_str());
    }
    if let Some(var_152) = &input.dedicated_master_count {
        object.key("DedicatedMasterCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_152).into()),
        );
    }
    if let Some(var_153) = &input.warm_enabled {
        object.key("WarmEnabled").boolean(*var_153);
    }
    if let Some(var_154) = &input.warm_type {
        object.key("WarmType").string(var_154.as_str());
    }
    if let Some(var_155) = &input.warm_count {
        object.key("WarmCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_155).into()),
        );
    }
    if let Some(var_156) = &input.cold_storage_options {
        let mut object_157 = object.key("ColdStorageOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_cold_storage_options(
            &mut object_157,
            var_156,
        )?;
        object_157.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_at_rest_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionAtRestOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_158) = &input.enabled {
        object.key("Enabled").boolean(*var_158);
    }
    if let Some(var_159) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_159.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_log_publishing_option(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LogPublishingOption,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_160) = &input.cloud_watch_logs_log_group_arn {
        object
            .key("CloudWatchLogsLogGroupArn")
            .string(var_160.as_str());
    }
    if let Some(var_161) = &input.enabled {
        object.key("Enabled").boolean(*var_161);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_node_to_node_encryption_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NodeToNodeEncryptionOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_162) = &input.enabled {
        object.key("Enabled").boolean(*var_162);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_snapshot_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SnapshotOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_163) = &input.automated_snapshot_start_hour {
        object.key("AutomatedSnapshotStartHour").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_163).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_vpc_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_164) = &input.subnet_ids {
        let mut array_165 = object.key("SubnetIds").start_array();
        for item_166 in var_164 {
            {
                array_165.value().string(item_166.as_str());
            }
        }
        array_165.finish();
    }
    if let Some(var_167) = &input.security_group_ids {
        let mut array_168 = object.key("SecurityGroupIds").start_array();
        for item_169 in var_167 {
            {
                array_168.value().string(item_169.as_str());
            }
        }
        array_168.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_domain_information(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DomainInformation,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_170) = &input.owner_id {
        object.key("OwnerId").string(var_170.as_str());
    }
    if let Some(var_171) = &input.domain_name {
        object.key("DomainName").string(var_171.as_str());
    }
    if let Some(var_172) = &input.region {
        object.key("Region").string(var_172.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_package_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PackageSource,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_173) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_173.as_str());
    }
    if let Some(var_174) = &input.s3_key {
        object.key("S3Key").string(var_174.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_175) = &input.name {
        object.key("Name").string(var_175.as_str());
    }
    if let Some(var_176) = &input.values {
        let mut array_177 = object.key("Values").start_array();
        for item_178 in var_176 {
            {
                array_177.value().string(item_178.as_str());
            }
        }
        array_177.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_describe_packages_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DescribePackagesFilter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_179) = &input.name {
        object.key("Name").string(var_179.as_str());
    }
    if let Some(var_180) = &input.value {
        let mut array_181 = object.key("Value").start_array();
        for item_182 in var_180 {
            {
                array_181.value().string(item_182.as_str());
            }
        }
        array_181.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_auto_tune_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoTuneOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_183) = &input.desired_state {
        object.key("DesiredState").string(var_183.as_str());
    }
    if let Some(var_184) = &input.rollback_on_disable {
        object.key("RollbackOnDisable").string(var_184.as_str());
    }
    if let Some(var_185) = &input.maintenance_schedules {
        let mut array_186 = object.key("MaintenanceSchedules").start_array();
        for item_187 in var_185 {
            {
                let mut object_188 = array_186.value().start_object();
                crate::json_ser::serialize_structure_crate_model_auto_tune_maintenance_schedule(
                    &mut object_188,
                    item_187,
                )?;
                object_188.finish();
            }
        }
        array_186.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_master_user_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MasterUserOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_189) = &input.master_user_arn {
        object.key("MasterUserARN").string(var_189.as_str());
    }
    if let Some(var_190) = &input.master_user_name {
        object.key("MasterUserName").string(var_190.as_str());
    }
    if let Some(var_191) = &input.master_user_password {
        object.key("MasterUserPassword").string(var_191.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_saml_options_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SamlOptionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_192) = &input.enabled {
        object.key("Enabled").boolean(*var_192);
    }
    if let Some(var_193) = &input.idp {
        let mut object_194 = object.key("Idp").start_object();
        crate::json_ser::serialize_structure_crate_model_saml_idp(&mut object_194, var_193)?;
        object_194.finish();
    }
    if let Some(var_195) = &input.master_user_name {
        object.key("MasterUserName").string(var_195.as_str());
    }
    if let Some(var_196) = &input.master_backend_role {
        object.key("MasterBackendRole").string(var_196.as_str());
    }
    if let Some(var_197) = &input.subject_key {
        object.key("SubjectKey").string(var_197.as_str());
    }
    if let Some(var_198) = &input.roles_key {
        object.key("RolesKey").string(var_198.as_str());
    }
    if let Some(var_199) = &input.session_timeout_minutes {
        object.key("SessionTimeoutMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_199).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_auto_tune_maintenance_schedule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoTuneMaintenanceSchedule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_200) = &input.start_at {
        object
            .key("StartAt")
            .date_time(var_200, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_201) = &input.duration {
        let mut object_202 = object.key("Duration").start_object();
        crate::json_ser::serialize_structure_crate_model_duration(&mut object_202, var_201)?;
        object_202.finish();
    }
    if let Some(var_203) = &input.cron_expression_for_recurrence {
        object
            .key("CronExpressionForRecurrence")
            .string(var_203.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_zone_awareness_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ZoneAwarenessConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_204) = &input.availability_zone_count {
        object.key("AvailabilityZoneCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_204).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cold_storage_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ColdStorageOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_205) = &input.enabled {
        object.key("Enabled").boolean(*var_205);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_saml_idp(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SamlIdp,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_206) = &input.metadata_content {
        object.key("MetadataContent").string(var_206.as_str());
    }
    if let Some(var_207) = &input.entity_id {
        object.key("EntityId").string(var_207.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_duration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Duration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.value != 0 {
        object.key("Value").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.value).into()),
        );
    }
    if let Some(var_208) = &input.unit {
        object.key("Unit").string(var_208.as_str());
    }
    Ok(())
}
