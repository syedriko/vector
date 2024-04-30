// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetMetricDataInput {
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 500 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, a Metrics Insights query, or a math expression to perform on retrieved data. </p>
    pub metric_data_queries: ::std::option::Option<::std::vec::Vec<crate::types::MetricDataQuery>>,
    /// <p>The time stamp indicating the earliest data to be returned.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp. </p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li> <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p> </li>
    /// <li> <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p> </li>
    /// <li> <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p> </li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15. </p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>StartTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>StartTime</code>.</p>
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time stamp indicating the latest data to be returned.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>EndTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>EndTime</code>.</p>
    pub end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Include this value, if it was returned by the previous <code>GetMetricData</code> operation, to get the next set of data points.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    pub scan_by: ::std::option::Option<crate::types::ScanBy>,
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub max_datapoints: ::std::option::Option<i32>,
    /// <p>This structure includes the <code>Timezone</code> parameter, which you can use to specify your time zone so that the labels of returned data display the correct time for your time zone. </p>
    pub label_options: ::std::option::Option<crate::types::LabelOptions>,
}
impl GetMetricDataInput {
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 500 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, a Metrics Insights query, or a math expression to perform on retrieved data. </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.metric_data_queries.is_none()`.
    pub fn metric_data_queries(&self) -> &[crate::types::MetricDataQuery] {
        self.metric_data_queries.as_deref().unwrap_or_default()
    }
    /// <p>The time stamp indicating the earliest data to be returned.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp. </p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li> <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p> </li>
    /// <li> <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p> </li>
    /// <li> <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p> </li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15. </p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>StartTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>StartTime</code>.</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The time stamp indicating the latest data to be returned.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>EndTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>EndTime</code>.</p>
    pub fn end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
    /// <p>Include this value, if it was returned by the previous <code>GetMetricData</code> operation, to get the next set of data points.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    pub fn scan_by(&self) -> ::std::option::Option<&crate::types::ScanBy> {
        self.scan_by.as_ref()
    }
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub fn max_datapoints(&self) -> ::std::option::Option<i32> {
        self.max_datapoints
    }
    /// <p>This structure includes the <code>Timezone</code> parameter, which you can use to specify your time zone so that the labels of returned data display the correct time for your time zone. </p>
    pub fn label_options(&self) -> ::std::option::Option<&crate::types::LabelOptions> {
        self.label_options.as_ref()
    }
}
impl GetMetricDataInput {
    /// Creates a new builder-style object to manufacture [`GetMetricDataInput`](crate::operation::get_metric_data::GetMetricDataInput).
    pub fn builder() -> crate::operation::get_metric_data::builders::GetMetricDataInputBuilder {
        crate::operation::get_metric_data::builders::GetMetricDataInputBuilder::default()
    }
}

/// A builder for [`GetMetricDataInput`](crate::operation::get_metric_data::GetMetricDataInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetMetricDataInputBuilder {
    pub(crate) metric_data_queries: ::std::option::Option<::std::vec::Vec<crate::types::MetricDataQuery>>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) scan_by: ::std::option::Option<crate::types::ScanBy>,
    pub(crate) max_datapoints: ::std::option::Option<i32>,
    pub(crate) label_options: ::std::option::Option<crate::types::LabelOptions>,
}
impl GetMetricDataInputBuilder {
    /// Appends an item to `metric_data_queries`.
    ///
    /// To override the contents of this collection use [`set_metric_data_queries`](Self::set_metric_data_queries).
    ///
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 500 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, a Metrics Insights query, or a math expression to perform on retrieved data. </p>
    pub fn metric_data_queries(mut self, input: crate::types::MetricDataQuery) -> Self {
        let mut v = self.metric_data_queries.unwrap_or_default();
        v.push(input);
        self.metric_data_queries = ::std::option::Option::Some(v);
        self
    }
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 500 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, a Metrics Insights query, or a math expression to perform on retrieved data. </p>
    pub fn set_metric_data_queries(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MetricDataQuery>>) -> Self {
        self.metric_data_queries = input;
        self
    }
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 500 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, a Metrics Insights query, or a math expression to perform on retrieved data. </p>
    pub fn get_metric_data_queries(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MetricDataQuery>> {
        &self.metric_data_queries
    }
    /// <p>The time stamp indicating the earliest data to be returned.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp. </p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li> <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p> </li>
    /// <li> <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p> </li>
    /// <li> <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p> </li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15. </p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>StartTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>StartTime</code>.</p>
    /// This field is required.
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time stamp indicating the earliest data to be returned.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp. </p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li> <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p> </li>
    /// <li> <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p> </li>
    /// <li> <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p> </li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15. </p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>StartTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>StartTime</code>.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The time stamp indicating the earliest data to be returned.</p>
    /// <p>The value specified is inclusive; results include data points with the specified time stamp. </p>
    /// <p>CloudWatch rounds the specified time stamp as follows:</p>
    /// <ul>
    /// <li> <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p> </li>
    /// <li> <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p> </li>
    /// <li> <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p> </li>
    /// </ul>
    /// <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15. </p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>StartTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>StartTime</code>.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.start_time
    }
    /// <p>The time stamp indicating the latest data to be returned.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>EndTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>EndTime</code>.</p>
    /// This field is required.
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time stamp indicating the latest data to be returned.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>EndTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>EndTime</code>.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.end_time = input;
        self
    }
    /// <p>The time stamp indicating the latest data to be returned.</p>
    /// <p>The value specified is exclusive; results include data points up to the specified time stamp.</p>
    /// <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>EndTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>EndTime</code>.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.end_time
    }
    /// <p>Include this value, if it was returned by the previous <code>GetMetricData</code> operation, to get the next set of data points.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Include this value, if it was returned by the previous <code>GetMetricData</code> operation, to get the next set of data points.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Include this value, if it was returned by the previous <code>GetMetricData</code> operation, to get the next set of data points.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    pub fn scan_by(mut self, input: crate::types::ScanBy) -> Self {
        self.scan_by = ::std::option::Option::Some(input);
        self
    }
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    pub fn set_scan_by(mut self, input: ::std::option::Option<crate::types::ScanBy>) -> Self {
        self.scan_by = input;
        self
    }
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    pub fn get_scan_by(&self) -> &::std::option::Option<crate::types::ScanBy> {
        &self.scan_by
    }
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub fn max_datapoints(mut self, input: i32) -> Self {
        self.max_datapoints = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub fn set_max_datapoints(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_datapoints = input;
        self
    }
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub fn get_max_datapoints(&self) -> &::std::option::Option<i32> {
        &self.max_datapoints
    }
    /// <p>This structure includes the <code>Timezone</code> parameter, which you can use to specify your time zone so that the labels of returned data display the correct time for your time zone. </p>
    pub fn label_options(mut self, input: crate::types::LabelOptions) -> Self {
        self.label_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>This structure includes the <code>Timezone</code> parameter, which you can use to specify your time zone so that the labels of returned data display the correct time for your time zone. </p>
    pub fn set_label_options(mut self, input: ::std::option::Option<crate::types::LabelOptions>) -> Self {
        self.label_options = input;
        self
    }
    /// <p>This structure includes the <code>Timezone</code> parameter, which you can use to specify your time zone so that the labels of returned data display the correct time for your time zone. </p>
    pub fn get_label_options(&self) -> &::std::option::Option<crate::types::LabelOptions> {
        &self.label_options
    }
    /// Consumes the builder and constructs a [`GetMetricDataInput`](crate::operation::get_metric_data::GetMetricDataInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_metric_data::GetMetricDataInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_metric_data::GetMetricDataInput {
            metric_data_queries: self.metric_data_queries,
            start_time: self.start_time,
            end_time: self.end_time,
            next_token: self.next_token,
            scan_by: self.scan_by,
            max_datapoints: self.max_datapoints,
            label_options: self.label_options,
        })
    }
}
