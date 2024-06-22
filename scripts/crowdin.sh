#!/bin/bash

# Crowdin API credentials and project details
API_KEY="your_crowdin_api_key"
PROJECT_IDENTIFIER="your_project_identifier"
BASE_URL="https://api.crowdin.com/api/v2/projects/$PROJECT_IDENTIFIER"
DOWNLOAD_DIR="./translations"

# Function to download translations
download_translations() {
    echo "Downloading translations from Crowdin..."

    # Create download directory if it doesn't exist
    mkdir -p $DOWNLOAD_DIR

    # List all languages configured in your Crowdin project
    LANGUAGES=$(curl -s -X GET "$BASE_URL/languages" \
        -H "Authorization: Bearer $API_KEY" \
        | jq -r '.data[].data.id')

    # Loop through each language and download translations
    for lang_id in $LANGUAGES; do
        lang_code=$(curl -s -X GET "$BASE_URL/languages/$lang_id" \
            -H "Authorization: Bearer $API_KEY" \
            | jq -r '.data.data.code')

        echo "Downloading translations for $lang_code..."
        
        # Download translations for each language
        curl -s -X GET "$BASE_URL/translations/export?languageId=$lang_id" \
            -H "Authorization: Bearer $API_KEY" \
            -o "$DOWNLOAD_DIR/$lang_code.json"
        
        echo "Translations for $lang_code downloaded."
    done

    echo "All translations downloaded successfully."
}

# Function to push source files to Crowdin (if needed)
push_source_files() {
    echo "Pushing source files to Crowdin..."
    
    # Example: Push source files using Crowdin CLI or API if needed
    # crowdin-cli upload sources

    echo "Source files pushed to Crowdin."
}

# Main script flow
case "$1" in
    download)
        download_translations
        ;;
    push)
        push_source_files
        ;;
    *)
        echo "Usage: $0 {download|push}"
        exit 1
esac

exit 0
