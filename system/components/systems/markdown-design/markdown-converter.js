/**
 * Markdown to HTML Converter
 * 
 * This script converts markdown files to elegant HTML pages using a customizable template
 * and styling system. It provides clean, readable, and visually appealing documentation.
 */

// Import required modules (commented out as this is for conceptual design)
// const fs = require('fs');
// const path = require('path');
// const marked = require('marked'); // For markdown parsing
// const highlightjs = require('highlight.js'); // For code syntax highlighting
// const yaml = require('js-yaml'); // For frontmatter parsing

/**
 * Configuration options for the markdown converter
 */
const config = {
  // Default template file path
  templatePath: 'template.html',
  
  // Default CSS styles
  stylesPath: 'markdown-styles.css',
  
  // Output directory for HTML files
  outputDir: '../MARKDOWN_DESIGNS',
  
  // Theme options: 'light', 'dark', 'sepia', 'minimal', 'ocean'
  defaultTheme: 'light',
  
  // Whether to generate a table of contents
  generateToc: true,
  
  // Whether to process front matter in markdown files
  processFrontMatter: true,
  
  // Whether to use syntax highlighting for code blocks
  syntaxHighlighting: true,
  
  // Whether to add anchor links to headings
  anchorHeadings: true,
  
  // Whether to process special markdown extensions (admonitions, etc.)
  useExtensions: true
};

/**
 * Parse frontmatter from markdown content
 * @param {string} content - The markdown content
 * @returns {Object} An object containing the frontmatter and the remaining content
 */
function parseFrontMatter(content) {
  // Check if content starts with frontmatter delimiters
  if (!content.startsWith('---')) {
    return { frontMatter: {}, content };
  }
  
  // Extract frontmatter
  const frontMatterEnd = content.indexOf('---', 3);
  if (frontMatterEnd === -1) {
    return { frontMatter: {}, content };
  }
  
  const frontMatterRaw = content.substring(3, frontMatterEnd).trim();
  const remainingContent = content.substring(frontMatterEnd + 3).trim();
  
  try {
    // Parse YAML frontmatter
    const frontMatter = {}; // Replace with yaml.load(frontMatterRaw);
    return { frontMatter, content: remainingContent };
  } catch (error) {
    console.error('Error parsing frontmatter:', error);
    return { frontMatter: {}, content };
  }
}

/**
 * Generate a table of contents from markdown content
 * @param {string} html - The HTML content
 * @returns {string} HTML for the table of contents
 */
function generateTableOfContents(html) {
  // Extract headings from HTML
  const headings = [];
  const headingRegex = /<h([2-4]).*?id="([^"]+)".*?>(.+?)<\/h\1>/g;
  let match;
  
  while ((match = headingRegex.exec(html)) !== null) {
    headings.push({
      level: parseInt(match[1], 10),
      id: match[2],
      text: match[3].replace(/<[^>]+>/g, '') // Remove HTML tags from heading text
    });
  }
  
  if (headings.length === 0) {
    return '';
  }
  
  // Generate nested TOC
  let toc = '<div class="toc">\n<div class="toc-title">Table of Contents</div>\n<ul>\n';
  
  let currentLevel = 2;
  let levelStack = [currentLevel];
  
  headings.forEach((heading) => {
    if (heading.level > currentLevel) {
      // Deeper level, add a new list
      for (let i = currentLevel; i < heading.level; i++) {
        toc += '<ul>\n';
        levelStack.push(heading.level);
      }
    } else if (heading.level < currentLevel) {
      // Higher level, close lists
      for (let i = currentLevel; i > heading.level; i--) {
        toc += '</ul>\n';
        levelStack.pop();
      }
    }
    
    currentLevel = heading.level;
    toc += `<li><a href="#${heading.id}">${heading.text}</a></li>\n`;
  });
  
  // Close remaining lists
  for (let i = 1; i < levelStack.length; i++) {
    toc += '</ul>\n';
  }
  
  toc += '</ul>\n</div>\n';
  return toc;
}

/**
 * Add anchor links to headings
 * @param {string} html - The HTML content
 * @returns {string} HTML with anchor links added to headings
 */
function addAnchorLinksToHeadings(html) {
  return html.replace(/<h([2-6]).*?id="([^"]+)".*?>(.+?)<\/h\1>/g, (match, level, id, content) => {
    return `<h${level} id="${id}">${content} <a class="anchor-link" href="#${id}" aria-hidden="true">#</a></h${level}>`;
  });
}

/**
 * Process markdown extensions like admonitions, callouts, etc.
 * @param {string} content - The markdown content
 * @returns {string} Processed markdown content
 */
function processMarkdownExtensions(content) {
  // Process admonitions (note, warning, tip)
  content = content.replace(/:::(\w+)(.*?)\n([\s\S]*?):::/gm, (match, type, title, body) => {
    title = title.trim() || type.charAt(0).toUpperCase() + type.slice(1);
    return `<div class="admonition ${type}">\n<div class="admonition-title">${title}</div>\n${body}</div>`;
  });
  
  // Process callouts
  content = content.replace(/>\s*\[!(\w+)\](.*?)\n([\s\S]*?)(?=\n\n|\n>|\n\[|$)/gm, (match, type, title, body) => {
    title = title.trim() || type.charAt(0).toUpperCase() + type.slice(1);
    return `<div class="callout ${type.toLowerCase()}">\n<div class="callout-icon">💡</div>\n<div class="callout-content">\n<h4>${title}</h4>\n${body}</div>\n</div>`;
  });
  
  return content;
}

/**
 * Convert a markdown file to HTML
 * @param {string} markdownPath - Path to the markdown file
 * @param {Object} options - Custom options for this conversion
 * @returns {Promise<string>} Path to the generated HTML file
 */
async function convertMarkdownToHtml(markdownPath, options = {}) {
  try {
    // Merge default config with custom options
    const mergedOptions = { ...config, ...options };
    
    // Read the markdown file content
    // const markdownContent = await fs.promises.readFile(markdownPath, 'utf8');
    const markdownContent = "# Example Markdown\n\nThis is placeholder content.";
    
    // Parse frontmatter if enabled
    let content = markdownContent;
    let frontMatter = {};
    
    if (mergedOptions.processFrontMatter) {
      const parsed = parseFrontMatter(markdownContent);
      content = parsed.content;
      frontMatter = parsed.frontMatter;
    }
    
    // Process markdown extensions if enabled
    if (mergedOptions.useExtensions) {
      content = processMarkdownExtensions(content);
    }
    
    // Convert markdown to HTML
    // Configure marked with options
    /*
    marked.setOptions({
      highlight: mergedOptions.syntaxHighlighting ? (code, lang) => {
        if (lang && highlightjs.getLanguage(lang)) {
          return highlightjs.highlight(code, { language: lang }).value;
        }
        return highlightjs.highlightAuto(code).value;
      } : null,
      headerIds: true,
      gfm: true
    });
    */
    
    // Parse markdown to HTML
    // const htmlContent = marked(content);
    const htmlContent = "<h1>Example HTML</h1><p>This is placeholder content.</p>";
    
    // Add table of contents if enabled
    let finalHtml = htmlContent;
    if (mergedOptions.generateToc) {
      const toc = generateTableOfContents(htmlContent);
      if (toc) {
        finalHtml = toc + finalHtml;
      }
    }
    
    // Add anchor links to headings if enabled
    if (mergedOptions.anchorHeadings) {
      finalHtml = addAnchorLinksToHeadings(finalHtml);
    }
    
    // Read the HTML template
    // const templateContent = await fs.promises.readFile(mergedOptions.templatePath, 'utf8');
    const templateContent = "<!DOCTYPE html>\n<html>\n<head>\n<title>{{TITLE}}</title>\n</head>\n<body>\n{{CONTENT}}\n</body>\n</html>";
    
    // Extract the base name of the markdown file for the title
    // const fileName = path.basename(markdownPath, path.extname(markdownPath));
    const fileName = "Example";
    
    // Prepare template variables
    const title = frontMatter.title || fileName;
    const date = frontMatter.date || new Date().toISOString().split('T')[0];
    const theme = frontMatter.theme || mergedOptions.defaultTheme;
    const author = frontMatter.author || '';
    const description = frontMatter.description || '';
    const customStyles = frontMatter.styles || '';
    
    // Generate navigation links if provided in frontmatter
    let navigationHtml = '';
    if (frontMatter.navigation && Array.isArray(frontMatter.navigation)) {
      navigationHtml = frontMatter.navigation
        .map(nav => `<a href="${nav.url}">${nav.title}</a>`)
        .join('\n');
    }
    
    // Replace template placeholders
    let finalOutput = templateContent
      .replace(/{{TITLE}}/g, title)
      .replace(/{{CONTENT}}/g, finalHtml)
      .replace(/{{AUTHOR}}/g, author)
      .replace(/{{DESCRIPTION}}/g, description)
      .replace(/{{THEME}}/g, theme)
      .replace(/{{DATE}}/g, date)
      .replace(/{{TIMESTAMP}}/g, new Date().toLocaleString())
      .replace(/{{NAVIGATION}}/g, navigationHtml)
      .replace(/{{CUSTOM_STYLES}}/g, customStyles);
    
    // Determine the output file path
    // const outputFileName = fileName + '.html';
    // const outputPath = path.join(mergedOptions.outputDir, outputFileName);
    const outputPath = "output.html";
    
    // Write the HTML file
    // await fs.promises.writeFile(outputPath, finalOutput, 'utf8');
    
    console.log(`Successfully converted ${markdownPath} to ${outputPath}`);
    return outputPath;
  } catch (error) {
    console.error('Error converting markdown to HTML:', error);
    throw error;
  }
}

/**
 * Process all markdown files in a directory
 * @param {string} sourceDir - Directory containing markdown files
 * @param {Object} options - Custom options for conversion
 * @returns {Promise<string[]>} Paths to the generated HTML files
 */
async function processDirectory(sourceDir, options = {}) {
  try {
    // Get all markdown files in the directory
    // const files = await fs.promises.readdir(sourceDir);
    // const markdownFiles = files.filter(file => file.endsWith('.md'));
    const markdownFiles = ["example.md", "another-example.md"];
    
    // Convert each markdown file to HTML
    const conversionPromises = markdownFiles.map(file => {
      // const filePath = path.join(sourceDir, file);
      // return convertMarkdownToHtml(filePath, options);
      return convertMarkdownToHtml(file, options);
    });
    
    // Wait for all conversions to complete
    const outputPaths = await Promise.all(conversionPromises);
    
    console.log(`Successfully processed ${outputPaths.length} markdown files`);
    return outputPaths;
  } catch (error) {
    console.error('Error processing directory:', error);
    throw error;
  }
}

/**
 * Main function to convert a markdown file to HTML
 * @param {string} inputPath - Path to the markdown file or directory
 * @param {Object} options - Custom options for conversion
 * @returns {Promise<string|string[]>} Path(s) to the generated HTML file(s)
 */
async function markdown2html(inputPath, options = {}) {
  try {
    // Check if input path is a file or directory
    // const stats = await fs.promises.stat(inputPath);
    
    // if (stats.isDirectory()) {
    //   return processDirectory(inputPath, options);
    // } else if (stats.isFile() && inputPath.endsWith('.md')) {
    //   return convertMarkdownToHtml(inputPath, options);
    // } else {
    //   throw new Error('Input path must be a markdown file or a directory containing markdown files');
    // }
    
    // Placeholder for demo purposes
    return convertMarkdownToHtml(inputPath, options);
  } catch (error) {
    console.error('Error in markdown2html:', error);
    throw error;
  }
}

// Export the functions (for Node.js module system)
// module.exports = { markdown2html, convertMarkdownToHtml, processDirectory };

// Function for direct use in the browser
function convertMarkdown(markdownText, options = {}) {
  // Implementation for browser use
}

// Command-line interface (if run directly)
// if (require.main === module) {
//   const args = process.argv.slice(2);
//   if (args.length === 0) {
//     console.error('Please provide a markdown file or directory path');
//     process.exit(1);
//   }
//   
//   const inputPath = args[0];
//   markdown2html(inputPath)
//     .then(result => {
//       console.log('Conversion completed successfully');
//     })
//     .catch(error => {
//       console.error('Conversion failed:', error);
//       process.exit(1);
//     });
// }