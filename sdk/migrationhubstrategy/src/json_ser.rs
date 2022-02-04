// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_list_application_components_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListApplicationComponentsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.application_component_criteria {
        object
            .key("applicationComponentCriteria")
            .string(var_1.as_str());
    }
    if let Some(var_2) = &input.filter_value {
        object.key("filterValue").string(var_2.as_str());
    }
    if let Some(var_3) = &input.group_id_filter {
        let mut array_4 = object.key("groupIdFilter").start_array();
        for item_5 in var_3 {
            {
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_crate_model_group(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.next_token {
        object.key("nextToken").string(var_8.as_str());
    }
    if let Some(var_9) = &input.sort {
        object.key("sort").string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_servers_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServersInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.filter_value {
        object.key("filterValue").string(var_10.as_str());
    }
    if let Some(var_11) = &input.group_id_filter {
        let mut array_12 = object.key("groupIdFilter").start_array();
        for item_13 in var_11 {
            {
                let mut object_14 = array_12.value().start_object();
                crate::json_ser::serialize_structure_crate_model_group(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_15).into()),
        );
    }
    if let Some(var_16) = &input.next_token {
        object.key("nextToken").string(var_16.as_str());
    }
    if let Some(var_17) = &input.server_criteria {
        object.key("serverCriteria").string(var_17.as_str());
    }
    if let Some(var_18) = &input.sort {
        object.key("sort").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_portfolio_preferences_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutPortfolioPreferencesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.application_preferences {
        let mut object_20 = object.key("applicationPreferences").start_object();
        crate::json_ser::serialize_structure_crate_model_application_preferences(
            &mut object_20,
            var_19,
        )?;
        object_20.finish();
    }
    if let Some(var_21) = &input.database_preferences {
        let mut object_22 = object.key("databasePreferences").start_object();
        crate::json_ser::serialize_structure_crate_model_database_preferences(
            &mut object_22,
            var_21,
        )?;
        object_22.finish();
    }
    if let Some(var_23) = &input.prioritize_business_goals {
        let mut object_24 = object.key("prioritizeBusinessGoals").start_object();
        crate::json_ser::serialize_structure_crate_model_prioritize_business_goals(
            &mut object_24,
            var_23,
        )?;
        object_24.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_assessment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartAssessmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.s3bucket_for_analysis_data {
        object
            .key("s3bucketForAnalysisData")
            .string(var_25.as_str());
    }
    if let Some(var_26) = &input.s3bucket_for_report_data {
        object.key("s3bucketForReportData").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_import_file_task_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartImportFileTaskInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.s3_bucket {
        object.key("S3Bucket").string(var_27.as_str());
    }
    if let Some(var_28) = &input.data_source_type {
        object.key("dataSourceType").string(var_28.as_str());
    }
    if let Some(var_29) = &input.group_id {
        let mut array_30 = object.key("groupId").start_array();
        for item_31 in var_29 {
            {
                let mut object_32 = array_30.value().start_object();
                crate::json_ser::serialize_structure_crate_model_group(&mut object_32, item_31)?;
                object_32.finish();
            }
        }
        array_30.finish();
    }
    if let Some(var_33) = &input.name {
        object.key("name").string(var_33.as_str());
    }
    if let Some(var_34) = &input.s3bucket_for_report_data {
        object.key("s3bucketForReportData").string(var_34.as_str());
    }
    if let Some(var_35) = &input.s3key {
        object.key("s3key").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_recommendation_report_generation_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartRecommendationReportGenerationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.group_id_filter {
        let mut array_37 = object.key("groupIdFilter").start_array();
        for item_38 in var_36 {
            {
                let mut object_39 = array_37.value().start_object();
                crate::json_ser::serialize_structure_crate_model_group(&mut object_39, item_38)?;
                object_39.finish();
            }
        }
        array_37.finish();
    }
    if let Some(var_40) = &input.output_format {
        object.key("outputFormat").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_assessment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopAssessmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.assessment_id {
        object.key("assessmentId").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_application_component_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApplicationComponentConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.application_component_id {
        object.key("applicationComponentId").string(var_42.as_str());
    }
    if let Some(var_43) = &input.inclusion_status {
        object.key("inclusionStatus").string(var_43.as_str());
    }
    if let Some(var_44) = &input.secrets_manager_key {
        object.key("secretsManagerKey").string(var_44.as_str());
    }
    if let Some(var_45) = &input.source_code_list {
        let mut array_46 = object.key("sourceCodeList").start_array();
        for item_47 in var_45 {
            {
                let mut object_48 = array_46.value().start_object();
                crate::json_ser::serialize_structure_crate_model_source_code(
                    &mut object_48,
                    item_47,
                )?;
                object_48.finish();
            }
        }
        array_46.finish();
    }
    if let Some(var_49) = &input.strategy_option {
        let mut object_50 = object.key("strategyOption").start_object();
        crate::json_ser::serialize_structure_crate_model_strategy_option(&mut object_50, var_49)?;
        object_50.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_server_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateServerConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.server_id {
        object.key("serverId").string(var_51.as_str());
    }
    if let Some(var_52) = &input.strategy_option {
        let mut object_53 = object.key("strategyOption").start_object();
        crate::json_ser::serialize_structure_crate_model_strategy_option(&mut object_53, var_52)?;
        object_53.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_group(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Group,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.name {
        object.key("name").string(var_54.as_str());
    }
    if let Some(var_55) = &input.value {
        object.key("value").string(var_55.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_application_preferences(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ApplicationPreferences,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.management_preference {
        let mut object_57 = object.key("managementPreference").start_object();
        crate::json_ser::serialize_union_crate_model_management_preference(&mut object_57, var_56)?;
        object_57.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_database_preferences(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatabasePreferences,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.database_management_preference {
        object
            .key("databaseManagementPreference")
            .string(var_58.as_str());
    }
    if let Some(var_59) = &input.database_migration_preference {
        let mut object_60 = object.key("databaseMigrationPreference").start_object();
        crate::json_ser::serialize_union_crate_model_database_migration_preference(
            &mut object_60,
            var_59,
        )?;
        object_60.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_prioritize_business_goals(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PrioritizeBusinessGoals,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.business_goals {
        let mut object_62 = object.key("businessGoals").start_object();
        crate::json_ser::serialize_structure_crate_model_business_goals(&mut object_62, var_61)?;
        object_62.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_source_code(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SourceCode,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.version_control {
        object.key("versionControl").string(var_63.as_str());
    }
    if let Some(var_64) = &input.source_version {
        object.key("sourceVersion").string(var_64.as_str());
    }
    if let Some(var_65) = &input.location {
        object.key("location").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_strategy_option(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StrategyOption,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.strategy {
        object.key("strategy").string(var_66.as_str());
    }
    if let Some(var_67) = &input.tool_name {
        object.key("toolName").string(var_67.as_str());
    }
    if let Some(var_68) = &input.target_destination {
        object.key("targetDestination").string(var_68.as_str());
    }
    if let Some(var_69) = &input.is_preferred {
        object.key("isPreferred").boolean(*var_69);
    }
    Ok(())
}

pub fn serialize_union_crate_model_management_preference(
    object_57: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ManagementPreference,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::ManagementPreference::AwsManagedResources(inner) => {
            let mut object_70 = object_57.key("awsManagedResources").start_object();
            crate::json_ser::serialize_structure_crate_model_aws_managed_resources(
                &mut object_70,
                inner,
            )?;
            object_70.finish();
        }
        crate::model::ManagementPreference::SelfManageResources(inner) => {
            let mut object_71 = object_57.key("selfManageResources").start_object();
            crate::json_ser::serialize_structure_crate_model_self_manage_resources(
                &mut object_71,
                inner,
            )?;
            object_71.finish();
        }
        crate::model::ManagementPreference::NoPreference(inner) => {
            let mut object_72 = object_57.key("noPreference").start_object();
            crate::json_ser::serialize_structure_crate_model_no_management_preference(
                &mut object_72,
                inner,
            )?;
            object_72.finish();
        }
        crate::model::ManagementPreference::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "ManagementPreference",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_database_migration_preference(
    object_60: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatabaseMigrationPreference,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::DatabaseMigrationPreference::Heterogeneous(inner) => {
            let mut object_73 = object_60.key("heterogeneous").start_object();
            crate::json_ser::serialize_structure_crate_model_heterogeneous(&mut object_73, inner)?;
            object_73.finish();
        }
        crate::model::DatabaseMigrationPreference::Homogeneous(inner) => {
            let mut object_74 = object_60.key("homogeneous").start_object();
            crate::json_ser::serialize_structure_crate_model_homogeneous(&mut object_74, inner)?;
            object_74.finish();
        }
        crate::model::DatabaseMigrationPreference::NoPreference(inner) => {
            let mut object_75 = object_60.key("noPreference").start_object();
            crate::json_ser::serialize_structure_crate_model_no_database_migration_preference(
                &mut object_75,
                inner,
            )?;
            object_75.finish();
        }
        crate::model::DatabaseMigrationPreference::Unknown => {
            return Err(
                aws_smithy_http::operation::SerializationError::unknown_variant(
                    "DatabaseMigrationPreference",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_business_goals(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BusinessGoals,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.speed_of_migration {
        object.key("speedOfMigration").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_76).into()),
        );
    }
    if let Some(var_77) = &input.reduce_operational_overhead_with_managed_services {
        object
            .key("reduceOperationalOverheadWithManagedServices")
            .number(
                #[allow(clippy::useless_conversion)]
                aws_smithy_types::Number::NegInt((*var_77).into()),
            );
    }
    if let Some(var_78) = &input.modernize_infrastructure_with_cloud_native_technologies {
        object
            .key("modernizeInfrastructureWithCloudNativeTechnologies")
            .number(
                #[allow(clippy::useless_conversion)]
                aws_smithy_types::Number::NegInt((*var_78).into()),
            );
    }
    if let Some(var_79) = &input.license_cost_reduction {
        object.key("licenseCostReduction").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_79).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_aws_managed_resources(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AwsManagedResources,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_80) = &input.target_destination {
        let mut array_81 = object.key("targetDestination").start_array();
        for item_82 in var_80 {
            {
                array_81.value().string(item_82.as_str());
            }
        }
        array_81.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_self_manage_resources(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SelfManageResources,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_83) = &input.target_destination {
        let mut array_84 = object.key("targetDestination").start_array();
        for item_85 in var_83 {
            {
                array_84.value().string(item_85.as_str());
            }
        }
        array_84.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_no_management_preference(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NoManagementPreference,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.target_destination {
        let mut array_87 = object.key("targetDestination").start_array();
        for item_88 in var_86 {
            {
                array_87.value().string(item_88.as_str());
            }
        }
        array_87.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_heterogeneous(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Heterogeneous,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.target_database_engine {
        let mut array_90 = object.key("targetDatabaseEngine").start_array();
        for item_91 in var_89 {
            {
                array_90.value().string(item_91.as_str());
            }
        }
        array_90.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_homogeneous(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Homogeneous,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_92) = &input.target_database_engine {
        let mut array_93 = object.key("targetDatabaseEngine").start_array();
        for item_94 in var_92 {
            {
                array_93.value().string(item_94.as_str());
            }
        }
        array_93.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_no_database_migration_preference(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NoDatabaseMigrationPreference,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.target_database_engine {
        let mut array_96 = object.key("targetDatabaseEngine").start_array();
        for item_97 in var_95 {
            {
                array_96.value().string(item_97.as_str());
            }
        }
        array_96.finish();
    }
    Ok(())
}
