const fs = require('fs');

// Example function to fix empty i18n values
function fixEmptyI18nValues(filePath) {
    try {
        // Read the i18n file
        const data = fs.readFileSync(filePath, 'utf8');

        // Parse JSON data
        const i18nConfig = JSON.parse(data);

        // Traverse the i18n configuration and fix empty values
        traverseAndFix(i18nConfig);

        // Write back the updated configuration
        fs.writeFileSync(filePath, JSON.stringify(i18nConfig, null, 2));
        
        console.log(`Fixed empty values in ${filePath}`);
    } catch (error) {
        console.error('Error fixing empty i18n values:', error);
    }
}

// Example recursive function to traverse and fix empty values
function traverseAndFix(obj) {
    for (const key in obj) {
        if (typeof obj[key] === 'object') {
            traverseAndFix(obj[key]);
        } else if (obj[key] === '' || obj[key] === undefined) {
            obj[key] = 'default value'; // Replace with your default value or logic
        }
    }
}

// Usage example
const i18nFilePath = './locales/en.json'; // Replace with your i18n file path
fixEmptyI18nValues(i18nFilePath);
