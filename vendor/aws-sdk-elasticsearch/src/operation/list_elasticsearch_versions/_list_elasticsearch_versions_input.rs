// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Container for the parameters to the <code> <code>ListElasticsearchVersions</code> </code> operation. </p>
/// <p> Use <code> <code>MaxResults</code> </code> to control the maximum number of results to retrieve in a single call. </p>
/// <p> Use <code> <code>NextToken</code> </code> in response to retrieve more results. If the received response does not contain a NextToken, then there are no more results to retrieve. </p>
/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListElasticsearchVersionsInput {
    /// <p> Set this value to limit the number of results returned. Value provided must be greater than 10 else it wont be honored. </p>
    pub max_results: ::std::option::Option<i32>,
    /// <p> Paginated APIs accepts NextToken input to returns next page results and provides a NextToken output in the response which can be used by the client to retrieve more results. </p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListElasticsearchVersionsInput {
    /// <p> Set this value to limit the number of results returned. Value provided must be greater than 10 else it wont be honored. </p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p> Paginated APIs accepts NextToken input to returns next page results and provides a NextToken output in the response which can be used by the client to retrieve more results. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListElasticsearchVersionsInput {
    /// Creates a new builder-style object to manufacture [`ListElasticsearchVersionsInput`](crate::operation::list_elasticsearch_versions::ListElasticsearchVersionsInput).
    pub fn builder() -> crate::operation::list_elasticsearch_versions::builders::ListElasticsearchVersionsInputBuilder {
        crate::operation::list_elasticsearch_versions::builders::ListElasticsearchVersionsInputBuilder::default()
    }
}

/// A builder for [`ListElasticsearchVersionsInput`](crate::operation::list_elasticsearch_versions::ListElasticsearchVersionsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListElasticsearchVersionsInputBuilder {
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListElasticsearchVersionsInputBuilder {
    /// <p> Set this value to limit the number of results returned. Value provided must be greater than 10 else it wont be honored. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p> Set this value to limit the number of results returned. Value provided must be greater than 10 else it wont be honored. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p> Set this value to limit the number of results returned. Value provided must be greater than 10 else it wont be honored. </p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p> Paginated APIs accepts NextToken input to returns next page results and provides a NextToken output in the response which can be used by the client to retrieve more results. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Paginated APIs accepts NextToken input to returns next page results and provides a NextToken output in the response which can be used by the client to retrieve more results. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p> Paginated APIs accepts NextToken input to returns next page results and provides a NextToken output in the response which can be used by the client to retrieve more results. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`ListElasticsearchVersionsInput`](crate::operation::list_elasticsearch_versions::ListElasticsearchVersionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_elasticsearch_versions::ListElasticsearchVersionsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_elasticsearch_versions::ListElasticsearchVersionsInput {
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
