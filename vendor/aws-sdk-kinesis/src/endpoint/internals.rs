// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(
    clippy::collapsible_if,
    clippy::bool_comparison,
    clippy::nonminimal_bool,
    clippy::comparison_to_empty,
    clippy::redundant_pattern_matching
)]
pub(super) fn resolve_endpoint(
    _params: &crate::endpoint::Params,
    _diagnostic_collector: &mut crate::endpoint_lib::diagnostic::DiagnosticCollector,
    partition_resolver: &crate::endpoint_lib::partition::PartitionResolver,
) -> aws_smithy_http::endpoint::Result {
    #[allow(unused_variables)]
    let region = &_params.region;
    #[allow(unused_variables)]
    let use_dual_stack = &_params.use_dual_stack;
    #[allow(unused_variables)]
    let use_fips = &_params.use_fips;
    #[allow(unused_variables)]
    let endpoint = &_params.endpoint;
    #[allow(unused_variables)]
    let stream_arn = &_params.stream_arn;
    #[allow(unused_variables)]
    let operation_type = &_params.operation_type;
    #[allow(unused_variables)]
    let consumer_arn = &_params.consumer_arn;
    #[allow(unused_variables)]
    if let Some(partition_result) =
        partition_resolver.resolve_partition(region, _diagnostic_collector)
    {
        #[allow(unused_variables)]
        if let Some(stream_arn) = stream_arn {
            if !(endpoint.is_some()) {
                if !((partition_result.name()) == ("aws-iso")) {
                    if !((partition_result.name()) == ("aws-iso-b")) {
                        #[allow(unused_variables)]
                        if let Some(arn) =
                            crate::endpoint_lib::arn::parse_arn(stream_arn, _diagnostic_collector)
                        {
                            if crate::endpoint_lib::host::is_valid_host_label(
                                arn.account_id(),
                                false,
                                _diagnostic_collector,
                            ) {
                                if crate::endpoint_lib::host::is_valid_host_label(
                                    arn.region(),
                                    false,
                                    _diagnostic_collector,
                                ) {
                                    if (arn.service()) == ("kinesis") {
                                        #[allow(unused_variables)]
                                        if let Some(arn_type) = arn.resource_id().get(0).cloned() {
                                            if !((arn_type) == ("")) {
                                                if (arn_type) == ("stream") {
                                                    if (partition_result.name())
                                                        == (arn.partition())
                                                    {
                                                        #[allow(unused_variables)]
                                                        if let Some(operation_type) = operation_type
                                                        {
                                                            if (*use_fips) == (true) {
                                                                if (*use_dual_stack) == (true) {
                                                                    if (partition_result
                                                                        .supports_fips())
                                                                        == (true)
                                                                    {
                                                                        if (partition_result
                                                                            .supports_dual_stack())
                                                                            == (true)
                                                                        {
                                                                            return Ok(aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
out.push_str("https://");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.account_id());
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&operation_type);
out.push_str("-kinesis-fips.");
#[allow(clippy::needless_borrow)]
out.push_str(&region);
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.dual_stack_dns_suffix());
out })
.build());
                                                                        }
                                                                        return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("DualStack is enabled, but this partition does not support DualStack."
.to_string()));
                                                                    }
                                                                    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS is enabled, but this partition does not support FIPS."
.to_string()));
                                                                }
                                                            }
                                                            if (*use_fips) == (true) {
                                                                if (partition_result
                                                                    .supports_fips())
                                                                    == (true)
                                                                {
                                                                    return Ok(aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
out.push_str("https://");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.account_id());
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&operation_type);
out.push_str("-kinesis-fips.");
#[allow(clippy::needless_borrow)]
out.push_str(&region);
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.dns_suffix());
out })
.build());
                                                                }
                                                                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS is enabled but this partition does not support FIPS"
.to_string()));
                                                            }
                                                            if (*use_dual_stack) == (true) {
                                                                if (partition_result
                                                                    .supports_dual_stack())
                                                                    == (true)
                                                                {
                                                                    return Ok(aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
out.push_str("https://");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.account_id());
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&operation_type);
out.push_str("-kinesis.");
#[allow(clippy::needless_borrow)]
out.push_str(&region);
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.dual_stack_dns_suffix());
out })
.build());
                                                                }
                                                                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("DualStack is enabled but this partition does not support DualStack"
.to_string()));
                                                            }
                                                            return Ok(aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
out.push_str("https://");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.account_id());
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&operation_type);
out.push_str("-kinesis.");
#[allow(clippy::needless_borrow)]
out.push_str(&region);
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.dns_suffix());
out })
.build());
                                                        }
                                                        return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("Operation Type is not set. Please contact service team for resolution."
.to_string()));
                                                    }
                                                    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message({ let mut out = String::new();
out.push_str("Partition: ");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.partition());
out.push_str(" from ARN doesn't match with partition name: ");
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.name());
out.push('.');
out }));
                                                }
                                                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message({ let mut out = String::new();
out.push_str("Invalid ARN: Kinesis ARNs don't support `");
#[allow(clippy::needless_borrow)]
out.push_str(&arn_type);
out.push_str("` arn types.");
out }));
                                            }
                                        }
                                        return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid ARN: No ARN type specified"
.to_string()));
                                    }
                                    return Err(
                                        aws_smithy_http::endpoint::ResolveEndpointError::message({
                                            let mut out = String::new();
                                            out.push_str("Invalid ARN: The ARN was not for the Kinesis service, found: ");
                                            #[allow(clippy::needless_borrow)]
                                            out.push_str(&arn.service());
                                            out.push('.');
                                            out
                                        }),
                                    );
                                }
                                return Err(
                                    aws_smithy_http::endpoint::ResolveEndpointError::message(
                                        "Invalid ARN: Invalid region.".to_string(),
                                    ),
                                );
                            }
                            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                                "Invalid ARN: Invalid account id.".to_string(),
                            ));
                        }
                        return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                            "Invalid ARN: Failed to parse ARN.".to_string(),
                        ));
                    }
                }
            }
        }
        #[allow(unused_variables)]
        if let Some(consumer_arn) = consumer_arn {
            if !(endpoint.is_some()) {
                if !((partition_result.name()) == ("aws-iso")) {
                    if !((partition_result.name()) == ("aws-iso-b")) {
                        #[allow(unused_variables)]
                        if let Some(arn) =
                            crate::endpoint_lib::arn::parse_arn(consumer_arn, _diagnostic_collector)
                        {
                            if crate::endpoint_lib::host::is_valid_host_label(
                                arn.account_id(),
                                false,
                                _diagnostic_collector,
                            ) {
                                if crate::endpoint_lib::host::is_valid_host_label(
                                    arn.region(),
                                    false,
                                    _diagnostic_collector,
                                ) {
                                    if (arn.service()) == ("kinesis") {
                                        #[allow(unused_variables)]
                                        if let Some(arn_type) = arn.resource_id().get(0).cloned() {
                                            if !((arn_type) == ("")) {
                                                if (arn_type) == ("stream") {
                                                    if (partition_result.name())
                                                        == (arn.partition())
                                                    {
                                                        #[allow(unused_variables)]
                                                        if let Some(operation_type) = operation_type
                                                        {
                                                            if (*use_fips) == (true) {
                                                                if (*use_dual_stack) == (true) {
                                                                    if (partition_result
                                                                        .supports_fips())
                                                                        == (true)
                                                                    {
                                                                        if (partition_result
                                                                            .supports_dual_stack())
                                                                            == (true)
                                                                        {
                                                                            return Ok(aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
out.push_str("https://");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.account_id());
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&operation_type);
out.push_str("-kinesis-fips.");
#[allow(clippy::needless_borrow)]
out.push_str(&region);
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.dual_stack_dns_suffix());
out })
.build());
                                                                        }
                                                                        return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("DualStack is enabled, but this partition does not support DualStack."
.to_string()));
                                                                    }
                                                                    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS is enabled, but this partition does not support FIPS."
.to_string()));
                                                                }
                                                            }
                                                            if (*use_fips) == (true) {
                                                                if (partition_result
                                                                    .supports_fips())
                                                                    == (true)
                                                                {
                                                                    return Ok(aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
out.push_str("https://");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.account_id());
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&operation_type);
out.push_str("-kinesis-fips.");
#[allow(clippy::needless_borrow)]
out.push_str(&region);
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.dns_suffix());
out })
.build());
                                                                }
                                                                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS is enabled but this partition does not support FIPS"
.to_string()));
                                                            }
                                                            if (*use_dual_stack) == (true) {
                                                                if (partition_result
                                                                    .supports_dual_stack())
                                                                    == (true)
                                                                {
                                                                    return Ok(aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
out.push_str("https://");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.account_id());
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&operation_type);
out.push_str("-kinesis.");
#[allow(clippy::needless_borrow)]
out.push_str(&region);
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.dual_stack_dns_suffix());
out })
.build());
                                                                }
                                                                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("DualStack is enabled but this partition does not support DualStack"
.to_string()));
                                                            }
                                                            return Ok(aws_smithy_types::endpoint::Endpoint::builder().url({ let mut out = String::new();
out.push_str("https://");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.account_id());
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&operation_type);
out.push_str("-kinesis.");
#[allow(clippy::needless_borrow)]
out.push_str(&region);
out.push('.');
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.dns_suffix());
out })
.build());
                                                        }
                                                        return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("Operation Type is not set. Please contact service team for resolution."
.to_string()));
                                                    }
                                                    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message({ let mut out = String::new();
out.push_str("Partition: ");
#[allow(clippy::needless_borrow)]
out.push_str(&arn.partition());
out.push_str(" from ARN doesn't match with partition name: ");
#[allow(clippy::needless_borrow)]
out.push_str(&partition_result.name());
out.push('.');
out }));
                                                }
                                                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message({ let mut out = String::new();
out.push_str("Invalid ARN: Kinesis ARNs don't support `");
#[allow(clippy::needless_borrow)]
out.push_str(&arn_type);
out.push_str("` arn types.");
out }));
                                            }
                                        }
                                        return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("Invalid ARN: No ARN type specified"
.to_string()));
                                    }
                                    return Err(
                                        aws_smithy_http::endpoint::ResolveEndpointError::message({
                                            let mut out = String::new();
                                            out.push_str("Invalid ARN: The ARN was not for the Kinesis service, found: ");
                                            #[allow(clippy::needless_borrow)]
                                            out.push_str(&arn.service());
                                            out.push('.');
                                            out
                                        }),
                                    );
                                }
                                return Err(
                                    aws_smithy_http::endpoint::ResolveEndpointError::message(
                                        "Invalid ARN: Invalid region.".to_string(),
                                    ),
                                );
                            }
                            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                                "Invalid ARN: Invalid account id.".to_string(),
                            ));
                        }
                        return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                            "Invalid ARN: Failed to parse ARN.".to_string(),
                        ));
                    }
                }
            }
        }
        #[allow(unused_variables)]
        if let Some(endpoint) = endpoint {
            if (*use_fips) == (true) {
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                    "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
                ));
            }
            if (*use_dual_stack) == (true) {
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                    "Invalid Configuration: Dualstack and custom endpoint are not supported"
                        .to_string(),
                ));
            }
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url(endpoint.to_owned())
                .build());
        }
        if (*use_fips) == (true) {
            if (*use_dual_stack) == (true) {
                if (true) == (partition_result.supports_fips()) {
                    if (true) == (partition_result.supports_dual_stack()) {
                        return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                            .url({
                                let mut out = String::new();
                                out.push_str("https://kinesis-fips.");
                                #[allow(clippy::needless_borrow)]
                                out.push_str(&region);
                                out.push('.');
                                #[allow(clippy::needless_borrow)]
                                out.push_str(&partition_result.dual_stack_dns_suffix());
                                out
                            })
                            .build());
                    }
                }
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS and DualStack are enabled, but this partition does not support one or both"
.to_string()));
            }
        }
        if (*use_fips) == (true) {
            if (true) == (partition_result.supports_fips()) {
                if ("aws-us-gov") == (partition_result.name()) {
                    return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                        .url({
                            let mut out = String::new();
                            out.push_str("https://kinesis.");
                            #[allow(clippy::needless_borrow)]
                            out.push_str(&region);
                            out.push_str(".amazonaws.com");
                            out
                        })
                        .build());
                }
                return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                    .url({
                        let mut out = String::new();
                        out.push_str("https://kinesis-fips.");
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&region);
                        out.push('.');
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&partition_result.dns_suffix());
                        out
                    })
                    .build());
            }
            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                "FIPS is enabled but this partition does not support FIPS".to_string(),
            ));
        }
        if (*use_dual_stack) == (true) {
            if (true) == (partition_result.supports_dual_stack()) {
                return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                    .url({
                        let mut out = String::new();
                        out.push_str("https://kinesis.");
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&region);
                        out.push('.');
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&partition_result.dual_stack_dns_suffix());
                        out
                    })
                    .build());
            }
            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                "DualStack is enabled but this partition does not support DualStack".to_string(),
            ));
        }
        if (region) == ("us-gov-east-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://kinesis.us-gov-east-1.amazonaws.com".to_string())
                .build());
        }
        if (region) == ("us-gov-west-1") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://kinesis.us-gov-west-1.amazonaws.com".to_string())
                .build());
        }
        return Ok(aws_smithy_types::endpoint::Endpoint::builder()
            .url({
                let mut out = String::new();
                out.push_str("https://kinesis.");
                #[allow(clippy::needless_borrow)]
                out.push_str(&region);
                out.push('.');
                #[allow(clippy::needless_borrow)]
                out.push_str(&partition_result.dns_suffix());
                out
            })
            .build());
    }
    #[allow(unreachable_code)]
    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
        format!(
            "No rules matched these parameters. This is a bug. {:?}",
            _params
        ),
    ));
}
