package metadata

remap: functions: get_enrichment_table_record: {
	category: "Enrichment"
	description: """
		Searches an enrichment table for a row that matches the given
		condition. A single row must be matched. If the no row, or more
		than one row is found an error is returned.

		The condition is specified as an object of field to value. The
		given fields are searched with the enrichment table to find the
		row that matches the given values. All fields must match.

		There are currently two forms of search criteria:

		1.  An exact match search. The given field must match the value
		exactly (case sensitivity can be specified with a separate parameter
		to the function.

		2. Date range search. The given field must be greater than or
		equal to the `from` date and less than or equal to the `to` date.
		"""

	arguments: [
		{
			name:        "table"
			description: "The enrichment table to search."
			required:    true
			type: ["string"]
		},
		{
			name:        "condition"
			description: "The condition to search on."
			required:    true
			type: ["object"]
		},
		{
			name: "select"
			description: '''
				A subset of fields from the enrichment table to return. If not
				specified all fields are returned.
		  '''
			required: false
			type: ["array"]
		},
		{
			name:        "case_sensitive"
			description: "Should text fields match case exactly."
			required:    false
			type: ["boolean"]
			default: true
		},
	]
	internal_failure_reasons: ["The row is not found.",
		"Multiple rows are found that match the condition",
	]
	return: types: ["object"]

	examples: [
		{
			title: "Exact match"
			source: #"""
				get_enrichment_table_record("csvfile",
																		{ "surname": "smith",
																			"firstname": "John" },
																		case_sensitive: false)
				"""#
			return: true
		},
		{
			title: "Date range search"
			source: #"""
				get_enrichment_table_record("csvfile",
																		{ "surname": "Smith",
																			"date_of_birth": { "from": t'1985-01-01',
																												 "to": t'1985-31-12'} })
				"""#
			return: true
		},
	]
}
