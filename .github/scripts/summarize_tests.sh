#!/bin/bash

# Set output file locations
unit_test_file="./unit-test-output/unit_tests_output.txt"
integration_test_file="./integration-test-output/integration_tests_output.txt"
coverage_report_file="./coverage-output/coverage_report.json"

# Initialize counters for unit tests
unit_total_passed=0
unit_total_failed=0
unit_total_ignored=0
unit_total_runtime=0.0

# Initialize counters for integration tests
integration_total_passed=0
integration_total_failed=0
integration_total_ignored=0
integration_total_runtime=0.0

# Flag for overall status
overall_status="success"

# Function to process test files (unit or integration)
process_test_file() {
  local file_type="$1" # "unit" or "integration"
  local file_path="$2"
  
  if [ -f "$file_path" ]; then
    while IFS= read -r line; do
      if [[ "$line" == *"test result:"* ]]; then
        if [[ "$line" == *"failed; "* && "$overall_status" != "failed" ]]; then
          overall_status="failed"
        fi

        passed=$(echo "$line" | grep -oP '(?<=passed; )\d+')
        failed=$(echo "$line" | grep -oP '(?<=failed; )\d+')
        ignored=$(echo "$line" | grep -oP '(?<=ignored; )\d+')
        runtime=$(echo "$line" | grep -oP '(?<=finished in )\d+\.\d+')

        # Update counters based on file type
        if [[ "$file_type" == "unit" ]]; then
          unit_total_passed=$((unit_total_passed + passed))
          unit_total_failed=$((unit_total_failed + failed))
          unit_total_ignored=$((unit_total_ignored + ignored))
          unit_total_runtime=$(echo "$unit_total_runtime + $runtime" | bc)
        elif [[ "$file_type" == "integration" ]]; then
          integration_total_passed=$((integration_total_passed + passed))
          integration_total_failed=$((integration_total_failed + failed))
          integration_total_ignored=$((integration_total_ignored + ignored))
          integration_total_runtime=$(echo "$integration_total_runtime + $runtime" | bc)
        fi
      fi
    done < "$file_path"
  else
    echo "$file_type test output not found."
  fi
}

# Process unit and integration test files
process_test_file "unit" "$unit_test_file"
process_test_file "integration" "$integration_test_file"

# Extract and summarize overall test coverage data
if [ -f "$coverage_report_file" ]; then
    coverage_totals=$(cat "$coverage_report_file" | jq '.data[0].totals')
    
    functions_count=$(echo "$coverage_totals" | jq '.functions.count')
    functions_percent=$(echo "$coverage_totals" | jq '.functions.percent')
    lines_count=$(echo "$coverage_totals" | jq '.lines.count')
    lines_percent=$(echo "$coverage_totals" | jq '.lines.percent')
    regions_count=$(echo "$coverage_totals" | jq '.regions.count')
    regions_percent=$(echo "$coverage_totals" | jq '.regions.percent')
    instantiations_count=$(echo "$coverage_totals" | jq '.instantiations.count')
    instantiations_percent=$(echo "$coverage_totals" | jq '.instantiations.percent')

    coverage_summary="\n*Test Coverage Summary*\n"
    coverage_summary+="Functions: $functions_count covered, $functions_percent% coverage\n"
    coverage_summary+="Lines: $lines_count covered, $lines_percent% coverage\n"
    coverage_summary+="Regions: $regions_count covered, $regions_percent% coverage\n"
    coverage_summary+="Instantiations: $instantiations_count covered, $instantiations_percent% coverage"
else
    coverage_summary="\nCoverage data not found."
fi

# Format the run times to 2 decimal places
unit_total_runtime=$(printf "%.2f" $unit_total_runtime)
integration_total_runtime=$(printf "%.2f" $integration_total_runtime)

# Prepare the summary
summary+="*Unit Tests*\nTotal passed: $unit_total_passed\nTotal failed: $unit_total_failed\nTotal ignored: $unit_total_ignored\nRuntime: ${unit_total_runtime}s\n\n"
summary+="*Integration Tests*\nTotal passed: $integration_total_passed\nTotal failed: $integration_total_failed\nTotal ignored: $integration_total_ignored\nRuntime: ${integration_total_runtime}s"
summary+="$coverage_summary"

# Output the summary
echo -e "$summary"
