// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `DescribeReservedElasticsearchInstanceOfferings`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeReservedElasticsearchInstanceOfferings;
impl DescribeReservedElasticsearchInstanceOfferings {
    /// Creates a new `DescribeReservedElasticsearchInstanceOfferings`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsInput,
    ) -> ::std::result::Result<
        crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                                err.downcast::<crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsError>().expect("correct error type")
                            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(output.downcast::<crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsOutput>().expect("correct output type"))
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
            "elasticsearchservice",
            "DescribeReservedElasticsearchInstanceOfferings",
            input,
            runtime_plugins,
            stop_point,
        )
        .await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());
        runtime_plugins = runtime_plugins.with_client_plugin(crate::auth_plugin::DefaultAuthOptionsPlugin::new(vec![
            ::aws_runtime::auth::sigv4::SCHEME_ID,
        ]));
        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DescribeReservedElasticsearchInstanceOfferings {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DescribeReservedElasticsearchInstanceOfferings");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            DescribeReservedElasticsearchInstanceOfferingsRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            DescribeReservedElasticsearchInstanceOfferingsResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_http::operation::Metadata::new(
            "DescribeReservedElasticsearchInstanceOfferings",
            "elasticsearchservice",
        ));
        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
            signing_options,
            ..::std::default::Default::default()
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(
        &self,
        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        #[allow(unused_mut)]
        let mut rcb =
            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DescribeReservedElasticsearchInstanceOfferings")
                .with_interceptor(
                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::new(
                        ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptorKind::ResponseBody,
                    ),
                )
                .with_interceptor(DescribeReservedElasticsearchInstanceOfferingsEndpointParamsInterceptor)
                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                    crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsError,
                >::new())
                .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                    crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsError,
                >::new())
                .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                    crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsError,
                >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct DescribeReservedElasticsearchInstanceOfferingsResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for DescribeReservedElasticsearchInstanceOfferingsResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 || force_error {
            crate::protocol_serde::shape_describe_reserved_elasticsearch_instance_offerings::de_describe_reserved_elasticsearch_instance_offerings_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_describe_reserved_elasticsearch_instance_offerings::de_describe_reserved_elasticsearch_instance_offerings_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct DescribeReservedElasticsearchInstanceOfferingsRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for DescribeReservedElasticsearchInstanceOfferingsRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/2015-01-01/es/reservedInstanceOfferings").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            fn uri_query(
                _input: &crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsInput,
                mut output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                let mut query = ::aws_smithy_http::query::Writer::new(output);
                if let ::std::option::Option::Some(inner_1) = &_input.reserved_elasticsearch_instance_offering_id {
                    {
                        query.push_kv("offeringId", &::aws_smithy_http::query::fmt_string(&inner_1));
                    }
                }
                if let ::std::option::Option::Some(inner_2) = &_input.max_results {
                    if *inner_2 != 0 {
                        query.push_kv("maxResults", ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode());
                    }
                }
                if let ::std::option::Option::Some(inner_3) = &_input.next_token {
                    {
                        query.push_kv("nextToken", &::aws_smithy_http::query::fmt_string(&inner_3));
                    }
                }
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from("");

        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct DescribeReservedElasticsearchInstanceOfferingsEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for DescribeReservedElasticsearchInstanceOfferingsEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "DescribeReservedElasticsearchInstanceOfferingsEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<DescribeReservedElasticsearchInstanceOfferingsInput>()
            .ok_or("failed to downcast to DescribeReservedElasticsearchInstanceOfferingsInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

/// Error type for the `DescribeReservedElasticsearchInstanceOfferingsError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum DescribeReservedElasticsearchInstanceOfferingsError {
    /// <p>An error occured because the client wanted to access a not supported operation. Gives http status code of 409.</p>
    DisabledOperationException(crate::types::error::DisabledOperationException),
    /// <p>The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500.</p>
    InternalException(crate::types::error::InternalException),
    /// <p>An exception for accessing or deleting a resource that does not exist. Gives http status code of 400.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>An exception for missing / invalid input fields. Gives http status code of 400.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-DescribeReservedElasticsearchInstanceOfferingsError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl DescribeReservedElasticsearchInstanceOfferingsError {
    /// Creates the `DescribeReservedElasticsearchInstanceOfferingsError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `DescribeReservedElasticsearchInstanceOfferingsError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::DisabledOperationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InternalException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ValidationException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `DescribeReservedElasticsearchInstanceOfferingsError::DisabledOperationException`.
    pub fn is_disabled_operation_exception(&self) -> bool {
        matches!(self, Self::DisabledOperationException(_))
    }
    /// Returns `true` if the error kind is `DescribeReservedElasticsearchInstanceOfferingsError::InternalException`.
    pub fn is_internal_exception(&self) -> bool {
        matches!(self, Self::InternalException(_))
    }
    /// Returns `true` if the error kind is `DescribeReservedElasticsearchInstanceOfferingsError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `DescribeReservedElasticsearchInstanceOfferingsError::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(self, Self::ValidationException(_))
    }
}
impl ::std::error::Error for DescribeReservedElasticsearchInstanceOfferingsError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::DisabledOperationException(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::ValidationException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for DescribeReservedElasticsearchInstanceOfferingsError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::DisabledOperationException(_inner) => _inner.fmt(f),
            Self::InternalException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::ValidationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for DescribeReservedElasticsearchInstanceOfferingsError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DescribeReservedElasticsearchInstanceOfferingsError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::DisabledOperationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InternalException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ValidationException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for DescribeReservedElasticsearchInstanceOfferingsError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}
impl ::aws_types::request_id::RequestId
    for crate::operation::describe_reserved_elasticsearch_instance_offerings::DescribeReservedElasticsearchInstanceOfferingsError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::describe_reserved_elasticsearch_instance_offerings::_describe_reserved_elasticsearch_instance_offerings_output::DescribeReservedElasticsearchInstanceOfferingsOutput;

pub use crate::operation::describe_reserved_elasticsearch_instance_offerings::_describe_reserved_elasticsearch_instance_offerings_input::DescribeReservedElasticsearchInstanceOfferingsInput;

mod _describe_reserved_elasticsearch_instance_offerings_input;

mod _describe_reserved_elasticsearch_instance_offerings_output;

/// Builders
pub mod builders;

/// Paginator for this operation
pub mod paginator;
