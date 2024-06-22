const fs = require('fs');
const path = require('path');
const Ajv = require('ajv');

// Directory where your JSON schema files are located
const schemaDir = path.join(__dirname, 'schemas');

// Directory where compiled validators will be saved
const outputDir = path.join(__dirname, 'compiledValidators');

// Create output directory if it doesn't exist
if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir);
}

// Initialize AJV instance
const ajv = new Ajv({ allErrors: true });

// Function to compile a JSON schema file
function compileSchema(schemaFile) {
    try {
        const schemaPath = path.join(schemaDir, schemaFile);
        const schema = require(schemaPath);
        const validator = ajv.compile(schema);

        // Write compiled validator to a new JavaScript file
        const validatorFileName = `${path.parse(schemaFile).name}Validator.js`;
        const validatorFilePath = path.join(outputDir, validatorFileName);
        const validatorCode = `module.exports = ${validator};`;

        fs.writeFileSync(validatorFilePath, validatorCode);

        console.log(`Compiled '${schemaFile}' to '${validatorFileName}'`);
    } catch (error) {
        console.error(`Error compiling '${schemaFile}':`, error);
    }
}

// Function to compile all JSON schema files in the directory
function compileAllSchemas() {
    fs.readdir(schemaDir, (err, files) => {
        if (err) {
            console.error('Error reading schema directory:', err);
            return;
        }

        files.forEach(file => {
            if (file.endsWith('.json')) {
                compileSchema(file);
            }
        });
    });
}

// Compile all schemas when script is run
compileAllSchemas();
