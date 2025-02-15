#!/bin/bash

# Arrays of possible values
levels=("DEBUG" "INFO " "TRACE" "ERROR" "WARN ")
systems=("database" "frontend" "network" "auth" "cache" "api" "scheduler" "queue")

# Array of possible messages
messages=(
    "Operation completed successfully"
    "Connection timed out"
    "Cache miss for key"
    "Request processed in"
    "Authentication failed for user"
    "Data sync completed"
    "Resource utilization at"
    "Background job started"
    "Configuration reloaded"
    "Rate limit exceeded"
)

# Additional words to make messages more dynamic
adjectives=("unexpected" "slow" "quick" "partial" "complete" "delayed")
nouns=("response" "process" "connection" "request" "operation")

while true; do
    # Get random values from arrays
    level=${levels[$RANDOM % ${#levels[@]}]}
    system=${systems[$RANDOM % ${#systems[@]}]}
    base_message=${messages[$RANDOM % ${#messages[@]}]}
    adj=${adjectives[$RANDOM % ${#adjectives[@]}]}
    noun=${nouns[$RANDOM % ${#nouns[@]}]}
    
    # Generate a random number for variety
    rand_num=$((RANDOM % 1000))
    
    # Construct the message with some randomization
    message="$base_message $adj $noun (id: $rand_num)"
    
    # Print the log entry
    echo "[$level] [$system] $message"
    
    # Sleep for a random duration between 0.1 and 0.3 seconds
    sleep "0.$(( RANDOM % 30 + 0 ))"
done
