#!/bin/bash

# Generate schema
cargo run --bin schema

# Clear previous message_types ts files
rm -rf message_types

# Ensure output directory exists
mkdir -p message_types

# Function to convert snake_case to PascalCase
to_pascal_case() {
    # Split the string by underscore and store in array
    IFS='_' read -ra words <<< "$1"
    
    # Initialize an empty result string
    result=""

    # Loop through each word, capitalize the first letter, and append
    for word in "${words[@]}"; do
        # Use `awk` to capitalize the first letter if `${word^}` doesn't work
        result+=$(echo "${word}" | awk '{print toupper(substr($0,1,1)) tolower(substr($0,2))}')
    done
    
    echo "$result"
}

# Process each matching JSON file in the schema directory
for file in schema/*_msg.json; do
  # Extract the base filename without path or extension
  base_name=$(basename "$file" .json)
  
  # Convert to PascalCase and add "Types.ts" suffix
  pascal_case_name=$(to_pascal_case "$base_name")Types.ts
  
  # Run json2ts command
  npx json2ts -i "$file" -o "message_types/$pascal_case_name"
  
  # Check for success
  if [ $? -eq 0 ]; then
    echo "Successfully processed $file -> message_types/$pascal_case_name"
  else
    echo "Error processing $file"
  fi
done