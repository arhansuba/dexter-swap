const fs = require('fs');

// Example function to generate a sitemap XML file
function generateSitemap(pages) {
    try {
        let xml = '<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n';

        // Loop through each page and generate sitemap entries
        pages.forEach(page => {
            xml += `<url>\n`;
            xml += `  <loc>${page.url}</loc>\n`; // URL of the page
            xml += `  <lastmod>${page.lastModified}</lastmod>\n`; // Last modified date
            xml += `</url>\n`;
        });

        xml += `</urlset>`;

        // Write XML to file
        fs.writeFileSync('./public/sitemap.xml', xml);

        console.log('Sitemap generated successfully.');
    } catch (error) {
        console.error('Error generating sitemap:', error);
    }
}

// Example usage
const pages = [
    { url: 'https://example.com/page1', lastModified: '2023-01-01' },
    { url: 'https://example.com/page2', lastModified: '2023-01-02' },
    // Add more pages as needed
];

generateSitemap(pages);
