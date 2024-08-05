#!/bin/bash

concurrency_levels=(10 25 50 100)
url_file="urls.txt"
duration="1M"
timestamp=$(date +"%Y-%m-%d_%H-%M-%S")
results_dir="results"
results_file="${results_dir}/siege_results_${timestamp}.csv"

mkdir -p $results_dir

echo "Date,Concurrency,Resource Availability,Avg Response Time,Throughput" > $results_file

for c in "${concurrency_levels[@]}"
do
    echo "Running siege with concurrency level: $c"
    result=$(siege -v -c$c -t$duration -f $url_file 2>&1 | tee /dev/tty)

    # Extract metrics from the siege output
    availability=$(echo "$result" | grep "Availability" | awk '{print $2}')
    response_time=$(echo "$result" | grep "Response time" | awk '{print $3}')
    throughput=$(echo "$result" | grep "Transaction rate" | awk '{print $3}')

    echo "$timestamp,$c,$availability,$response_time,$throughput" >> $results_file
done

echo "Results saved to $results_file"
