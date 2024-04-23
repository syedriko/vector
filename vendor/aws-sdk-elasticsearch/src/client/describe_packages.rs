// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribePackages`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(DescribePackagesFilter)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::filters) / [`set_filters(Option<Vec::<DescribePackagesFilter>>)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::set_filters):<br>required: **false**<br><p>Only returns packages that match the <code>DescribePackagesFilterList</code> values.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::set_max_results):<br>required: **false**<br><p>Limits results to a maximum number of packages.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::set_next_token):<br>required: **false**<br><p>Used for pagination. Only necessary if a previous API call includes a non-null NextToken value. If provided, returns results for the next page.</p><br>
    /// - On success, responds with [`DescribePackagesOutput`](crate::operation::describe_packages::DescribePackagesOutput) with field(s):
    ///   - [`package_details_list(Option<Vec::<PackageDetails>>)`](crate::operation::describe_packages::DescribePackagesOutput::package_details_list): <p>List of <code>PackageDetails</code> objects.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_packages::DescribePackagesOutput::next_token): (undocumented)
    /// - On failure, responds with [`SdkError<DescribePackagesError>`](crate::operation::describe_packages::DescribePackagesError)
    pub fn describe_packages(&self) -> crate::operation::describe_packages::builders::DescribePackagesFluentBuilder {
        crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::new(self.handle.clone())
    }
}
