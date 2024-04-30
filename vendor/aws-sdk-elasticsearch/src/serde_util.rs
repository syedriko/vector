// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn authorize_vpc_endpoint_access_output_output_correct_errors(
    mut builder: crate::operation::authorize_vpc_endpoint_access::builders::AuthorizeVpcEndpointAccessOutputBuilder,
) -> crate::operation::authorize_vpc_endpoint_access::builders::AuthorizeVpcEndpointAccessOutputBuilder {
    if builder.authorized_principal.is_none() {
        builder.authorized_principal = {
            let builder = crate::types::builders::AuthorizedPrincipalBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn create_vpc_endpoint_output_output_correct_errors(
    mut builder: crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointOutputBuilder,
) -> crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointOutputBuilder {
    if builder.vpc_endpoint.is_none() {
        builder.vpc_endpoint = {
            let builder = crate::types::builders::VpcEndpointBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn delete_vpc_endpoint_output_output_correct_errors(
    mut builder: crate::operation::delete_vpc_endpoint::builders::DeleteVpcEndpointOutputBuilder,
) -> crate::operation::delete_vpc_endpoint::builders::DeleteVpcEndpointOutputBuilder {
    if builder.vpc_endpoint_summary.is_none() {
        builder.vpc_endpoint_summary = {
            let builder = crate::types::builders::VpcEndpointSummaryBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn describe_elasticsearch_domain_output_output_correct_errors(
    mut builder: crate::operation::describe_elasticsearch_domain::builders::DescribeElasticsearchDomainOutputBuilder,
) -> crate::operation::describe_elasticsearch_domain::builders::DescribeElasticsearchDomainOutputBuilder {
    if builder.domain_status.is_none() {
        builder.domain_status = {
            let builder = crate::types::builders::ElasticsearchDomainStatusBuilder::default();
            crate::serde_util::elasticsearch_domain_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn describe_elasticsearch_domain_config_output_output_correct_errors(
    mut builder: crate::operation::describe_elasticsearch_domain_config::builders::DescribeElasticsearchDomainConfigOutputBuilder,
) -> crate::operation::describe_elasticsearch_domain_config::builders::DescribeElasticsearchDomainConfigOutputBuilder {
    if builder.domain_config.is_none() {
        builder.domain_config = {
            let builder = crate::types::builders::ElasticsearchDomainConfigBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn describe_elasticsearch_domains_output_output_correct_errors(
    mut builder: crate::operation::describe_elasticsearch_domains::builders::DescribeElasticsearchDomainsOutputBuilder,
) -> crate::operation::describe_elasticsearch_domains::builders::DescribeElasticsearchDomainsOutputBuilder {
    if builder.domain_status_list.is_none() {
        builder.domain_status_list = Some(Default::default())
    }
    builder
}

pub(crate) fn describe_vpc_endpoints_output_output_correct_errors(
    mut builder: crate::operation::describe_vpc_endpoints::builders::DescribeVpcEndpointsOutputBuilder,
) -> crate::operation::describe_vpc_endpoints::builders::DescribeVpcEndpointsOutputBuilder {
    if builder.vpc_endpoints.is_none() {
        builder.vpc_endpoints = Some(Default::default())
    }
    if builder.vpc_endpoint_errors.is_none() {
        builder.vpc_endpoint_errors = Some(Default::default())
    }
    builder
}

pub(crate) fn list_vpc_endpoint_access_output_output_correct_errors(
    mut builder: crate::operation::list_vpc_endpoint_access::builders::ListVpcEndpointAccessOutputBuilder,
) -> crate::operation::list_vpc_endpoint_access::builders::ListVpcEndpointAccessOutputBuilder {
    if builder.authorized_principal_list.is_none() {
        builder.authorized_principal_list = Some(Default::default())
    }
    if builder.next_token.is_none() {
        builder.next_token = Some(Default::default())
    }
    builder
}

pub(crate) fn list_vpc_endpoints_output_output_correct_errors(
    mut builder: crate::operation::list_vpc_endpoints::builders::ListVpcEndpointsOutputBuilder,
) -> crate::operation::list_vpc_endpoints::builders::ListVpcEndpointsOutputBuilder {
    if builder.vpc_endpoint_summary_list.is_none() {
        builder.vpc_endpoint_summary_list = Some(Default::default())
    }
    if builder.next_token.is_none() {
        builder.next_token = Some(Default::default())
    }
    builder
}

pub(crate) fn list_vpc_endpoints_for_domain_output_output_correct_errors(
    mut builder: crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainOutputBuilder,
) -> crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainOutputBuilder {
    if builder.vpc_endpoint_summary_list.is_none() {
        builder.vpc_endpoint_summary_list = Some(Default::default())
    }
    if builder.next_token.is_none() {
        builder.next_token = Some(Default::default())
    }
    builder
}

pub(crate) fn update_elasticsearch_domain_config_output_output_correct_errors(
    mut builder: crate::operation::update_elasticsearch_domain_config::builders::UpdateElasticsearchDomainConfigOutputBuilder,
) -> crate::operation::update_elasticsearch_domain_config::builders::UpdateElasticsearchDomainConfigOutputBuilder {
    if builder.domain_config.is_none() {
        builder.domain_config = {
            let builder = crate::types::builders::ElasticsearchDomainConfigBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn update_vpc_endpoint_output_output_correct_errors(
    mut builder: crate::operation::update_vpc_endpoint::builders::UpdateVpcEndpointOutputBuilder,
) -> crate::operation::update_vpc_endpoint::builders::UpdateVpcEndpointOutputBuilder {
    if builder.vpc_endpoint.is_none() {
        builder.vpc_endpoint = {
            let builder = crate::types::builders::VpcEndpointBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn elasticsearch_domain_status_correct_errors(
    mut builder: crate::types::builders::ElasticsearchDomainStatusBuilder,
) -> crate::types::builders::ElasticsearchDomainStatusBuilder {
    if builder.domain_id.is_none() {
        builder.domain_id = Some(Default::default())
    }
    if builder.domain_name.is_none() {
        builder.domain_name = Some(Default::default())
    }
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.elasticsearch_cluster_config.is_none() {
        builder.elasticsearch_cluster_config = {
            let builder = crate::types::builders::ElasticsearchClusterConfigBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn domain_information_correct_errors(
    mut builder: crate::types::builders::DomainInformationBuilder,
) -> crate::types::builders::DomainInformationBuilder {
    if builder.domain_name.is_none() {
        builder.domain_name = Some(Default::default())
    }
    builder
}

pub(crate) fn access_policies_status_correct_errors(
    mut builder: crate::types::builders::AccessPoliciesStatusBuilder,
) -> crate::types::builders::AccessPoliciesStatusBuilder {
    if builder.options.is_none() {
        builder.options = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn advanced_options_status_correct_errors(
    mut builder: crate::types::builders::AdvancedOptionsStatusBuilder,
) -> crate::types::builders::AdvancedOptionsStatusBuilder {
    if builder.options.is_none() {
        builder.options = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn advanced_security_options_status_correct_errors(
    mut builder: crate::types::builders::AdvancedSecurityOptionsStatusBuilder,
) -> crate::types::builders::AdvancedSecurityOptionsStatusBuilder {
    if builder.options.is_none() {
        builder.options = {
            let builder = crate::types::builders::AdvancedSecurityOptionsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn cognito_options_status_correct_errors(
    mut builder: crate::types::builders::CognitoOptionsStatusBuilder,
) -> crate::types::builders::CognitoOptionsStatusBuilder {
    if builder.options.is_none() {
        builder.options = {
            let builder = crate::types::builders::CognitoOptionsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn domain_endpoint_options_status_correct_errors(
    mut builder: crate::types::builders::DomainEndpointOptionsStatusBuilder,
) -> crate::types::builders::DomainEndpointOptionsStatusBuilder {
    if builder.options.is_none() {
        builder.options = {
            let builder = crate::types::builders::DomainEndpointOptionsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn ebs_options_status_correct_errors(
    mut builder: crate::types::builders::EbsOptionsStatusBuilder,
) -> crate::types::builders::EbsOptionsStatusBuilder {
    if builder.options.is_none() {
        builder.options = {
            let builder = crate::types::builders::EbsOptionsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn elasticsearch_cluster_config_status_correct_errors(
    mut builder: crate::types::builders::ElasticsearchClusterConfigStatusBuilder,
) -> crate::types::builders::ElasticsearchClusterConfigStatusBuilder {
    if builder.options.is_none() {
        builder.options = {
            let builder = crate::types::builders::ElasticsearchClusterConfigBuilder::default();
            Some(builder.build())
        }
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn elasticsearch_version_status_correct_errors(
    mut builder: crate::types::builders::ElasticsearchVersionStatusBuilder,
) -> crate::types::builders::ElasticsearchVersionStatusBuilder {
    if builder.options.is_none() {
        builder.options = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn encryption_at_rest_options_status_correct_errors(
    mut builder: crate::types::builders::EncryptionAtRestOptionsStatusBuilder,
) -> crate::types::builders::EncryptionAtRestOptionsStatusBuilder {
    if builder.options.is_none() {
        builder.options = {
            let builder = crate::types::builders::EncryptionAtRestOptionsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn node_to_node_encryption_options_status_correct_errors(
    mut builder: crate::types::builders::NodeToNodeEncryptionOptionsStatusBuilder,
) -> crate::types::builders::NodeToNodeEncryptionOptionsStatusBuilder {
    if builder.options.is_none() {
        builder.options = {
            let builder = crate::types::builders::NodeToNodeEncryptionOptionsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn snapshot_options_status_correct_errors(
    mut builder: crate::types::builders::SnapshotOptionsStatusBuilder,
) -> crate::types::builders::SnapshotOptionsStatusBuilder {
    if builder.options.is_none() {
        builder.options = {
            let builder = crate::types::builders::SnapshotOptionsBuilder::default();
            Some(builder.build())
        }
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn vpc_derived_info_status_correct_errors(
    mut builder: crate::types::builders::VpcDerivedInfoStatusBuilder,
) -> crate::types::builders::VpcDerivedInfoStatusBuilder {
    if builder.options.is_none() {
        builder.options = {
            let builder = crate::types::builders::VpcDerivedInfoBuilder::default();
            Some(builder.build())
        }
    }
    if builder.status.is_none() {
        builder.status = {
            let builder = crate::types::builders::OptionStatusBuilder::default();
            crate::serde_util::option_status_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn auto_tune_status_correct_errors(
    mut builder: crate::types::builders::AutoTuneStatusBuilder,
) -> crate::types::builders::AutoTuneStatusBuilder {
    if builder.creation_date.is_none() {
        builder.creation_date = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.update_date.is_none() {
        builder.update_date = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::AutoTuneState>().ok()
    }
    builder
}

pub(crate) fn cold_storage_options_correct_errors(
    mut builder: crate::types::builders::ColdStorageOptionsBuilder,
) -> crate::types::builders::ColdStorageOptionsBuilder {
    if builder.enabled.is_none() {
        builder.enabled = Some(Default::default())
    }
    builder
}

pub(crate) fn option_status_correct_errors(mut builder: crate::types::builders::OptionStatusBuilder) -> crate::types::builders::OptionStatusBuilder {
    if builder.creation_date.is_none() {
        builder.creation_date = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.update_date.is_none() {
        builder.update_date = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::OptionState>().ok()
    }
    builder
}

pub(crate) fn saml_idp_correct_errors(mut builder: crate::types::builders::SamlIdpBuilder) -> crate::types::builders::SamlIdpBuilder {
    if builder.metadata_content.is_none() {
        builder.metadata_content = Some(Default::default())
    }
    if builder.entity_id.is_none() {
        builder.entity_id = Some(Default::default())
    }
    builder
}
