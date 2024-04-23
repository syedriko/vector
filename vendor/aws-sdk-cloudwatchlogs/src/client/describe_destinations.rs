// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDestinations`](crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`destination_name_prefix(impl Into<String>)`](crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder::destination_name_prefix) / [`set_destination_name_prefix(Option<String>)`](crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder::set_destination_name_prefix):<br>required: **false**<br><p>The prefix to match. If you don't specify a value, no prefix filter is applied.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of items to return. (You received this token from a previous call.)</p><br>
    ///   - [`limit(i32)`](crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of items returned. If you don't specify a value, the default maximum value of 50 items is used.</p><br>
    /// - On success, responds with [`DescribeDestinationsOutput`](crate::operation::describe_destinations::DescribeDestinationsOutput) with field(s):
    ///   - [`destinations(Option<Vec::<Destination>>)`](crate::operation::describe_destinations::DescribeDestinationsOutput::destinations): <p>The destinations.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_destinations::DescribeDestinationsOutput::next_token): <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    /// - On failure, responds with [`SdkError<DescribeDestinationsError>`](crate::operation::describe_destinations::DescribeDestinationsError)
    pub fn describe_destinations(&self) -> crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder {
        crate::operation::describe_destinations::builders::DescribeDestinationsFluentBuilder::new(self.handle.clone())
    }
}
