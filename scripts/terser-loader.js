const fs = require('fs');
const Terser = require('terser');

// Example function to minify JavaScript using Terser
function terserLoader(inputFile, outputFile) {
    try {
        const code = fs.readFileSync(inputFile, 'utf8');

        const result = Terser.minify(code);

        if (result.error) {
            throw result.error;
        }

        fs.writeFileSync(outputFile, result.code);

        console.log(`File minified successfully: ${outputFile}`);
    } catch (error) {
        console.error('Error minifying JavaScript:', error);
    }
}

// Example usage
const inputFilePath = './src/main.js'; // Replace with your input JavaScript file path
const outputFilePath = './dist/main.min.js'; // Replace with your output minified JavaScript file path

terserLoader(inputFilePath, outputFilePath);
