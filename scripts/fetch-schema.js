const axios = require('axios');
const fs = require('fs');
const path = require('path');

// Function to fetch schema from a remote URL
async function fetchSchema(url) {
    try {
        console.log(`Fetching schema from ${url}...`);
        
        // Make GET request to fetch schema
        const response = await axios.get(url);

        // Assuming the response data contains the schema
        const schema = response.data;

        console.log('Schema fetched successfully.');

        return schema;
    } catch (error) {
        console.error('Error fetching schema:', error.message);
        throw error;
    }
}

// Function to save schema to a local file
async function saveSchemaToFile(schema, filename) {
    const filePath = path.join(__dirname, filename);

    try {
        console.log(`Saving schema to ${filePath}...`);

        // Write schema JSON to file
        fs.writeFileSync(filePath, JSON.stringify(schema, null, 2));

        console.log('Schema saved successfully.');

        return filePath;
    } catch (error) {
        console.error('Error saving schema:', error.message);
        throw error;
    }
}

// Main function to fetch schema and save to file
async function main() {
    const schemaUrl = 'https://example.com/schema.json'; // Replace with your actual schema URL
    const schemaFileName = 'schema.json'; // Name of the file to save the schema

    try {
        // Fetch schema from URL
        const schema = await fetchSchema(schemaUrl);

        // Save schema to file
        const filePath = await saveSchemaToFile(schema, schemaFileName);

        console.log(`Schema fetched and saved to ${filePath}`);
    } catch (error) {
        console.error('An error occurred:', error);
    }
}

// Run the main function
main();
