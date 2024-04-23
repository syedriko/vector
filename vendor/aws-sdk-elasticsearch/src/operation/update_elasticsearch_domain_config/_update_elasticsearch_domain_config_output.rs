// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The result of an <code>UpdateElasticsearchDomain</code> request. Contains the status of the Elasticsearch domain being updated.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateElasticsearchDomainConfigOutput {
    /// <p>The status of the updated Elasticsearch domain. </p>
    pub domain_config: ::std::option::Option<crate::types::ElasticsearchDomainConfig>,
    /// <p>Contains result of DryRun. </p>
    pub dry_run_results: ::std::option::Option<crate::types::DryRunResults>,
    _request_id: Option<String>,
}
impl UpdateElasticsearchDomainConfigOutput {
    /// <p>The status of the updated Elasticsearch domain. </p>
    pub fn domain_config(&self) -> ::std::option::Option<&crate::types::ElasticsearchDomainConfig> {
        self.domain_config.as_ref()
    }
    /// <p>Contains result of DryRun. </p>
    pub fn dry_run_results(&self) -> ::std::option::Option<&crate::types::DryRunResults> {
        self.dry_run_results.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for UpdateElasticsearchDomainConfigOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateElasticsearchDomainConfigOutput {
    /// Creates a new builder-style object to manufacture [`UpdateElasticsearchDomainConfigOutput`](crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigOutput).
    pub fn builder() -> crate::operation::update_elasticsearch_domain_config::builders::UpdateElasticsearchDomainConfigOutputBuilder {
        crate::operation::update_elasticsearch_domain_config::builders::UpdateElasticsearchDomainConfigOutputBuilder::default()
    }
}

/// A builder for [`UpdateElasticsearchDomainConfigOutput`](crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateElasticsearchDomainConfigOutputBuilder {
    pub(crate) domain_config: ::std::option::Option<crate::types::ElasticsearchDomainConfig>,
    pub(crate) dry_run_results: ::std::option::Option<crate::types::DryRunResults>,
    _request_id: Option<String>,
}
impl UpdateElasticsearchDomainConfigOutputBuilder {
    /// <p>The status of the updated Elasticsearch domain. </p>
    /// This field is required.
    pub fn domain_config(mut self, input: crate::types::ElasticsearchDomainConfig) -> Self {
        self.domain_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the updated Elasticsearch domain. </p>
    pub fn set_domain_config(mut self, input: ::std::option::Option<crate::types::ElasticsearchDomainConfig>) -> Self {
        self.domain_config = input;
        self
    }
    /// <p>The status of the updated Elasticsearch domain. </p>
    pub fn get_domain_config(&self) -> &::std::option::Option<crate::types::ElasticsearchDomainConfig> {
        &self.domain_config
    }
    /// <p>Contains result of DryRun. </p>
    pub fn dry_run_results(mut self, input: crate::types::DryRunResults) -> Self {
        self.dry_run_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains result of DryRun. </p>
    pub fn set_dry_run_results(mut self, input: ::std::option::Option<crate::types::DryRunResults>) -> Self {
        self.dry_run_results = input;
        self
    }
    /// <p>Contains result of DryRun. </p>
    pub fn get_dry_run_results(&self) -> &::std::option::Option<crate::types::DryRunResults> {
        &self.dry_run_results
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateElasticsearchDomainConfigOutput`](crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigOutput).
    pub fn build(self) -> crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigOutput {
        crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigOutput {
            domain_config: self.domain_config,
            dry_run_results: self.dry_run_results,
            _request_id: self._request_id,
        }
    }
}
