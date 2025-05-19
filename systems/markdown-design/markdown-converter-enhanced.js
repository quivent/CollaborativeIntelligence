/**
 * Enhanced Markdown-to-HTML Converter
 * Designer Agent - Advanced Markdown System
 * 
 * This script provides advanced markdown conversion with additional features:
 * - YAML frontmatter processing
 * - Extended markdown syntax
 * - Component rendering
 * - Table of contents generation
 * - Code highlighting
 * - Responsive image handling
 * - Theme integration
 * - Layout options
 * - Social media preview integration
 * - Embedded SVG favicons
 * - Mobile app metadata
 * 
 * DESIGN FREEDOM PRIORITY:
 * ------------------------
 * This script provides a foundation, but the Designer's creative judgment 
 * always takes precedence over these default implementations. The Designer 
 * is expected to override, customize, or completely replace any aspect of 
 * this system when creatively appropriate. Templates and scripts exist to 
 * serve the Designer's vision, never to constrain it.
 */

// Import dependencies (conceptual, would need to be installed)
// const marked = require('marked');
// const yaml = require('js-yaml');
// const hljs = require('highlight.js');
// const fs = require('fs');
// const path = require('path');
// const designLanguageParser = require('./design-language-parser');
const socialPreviewGenerator = require('./social-preview-generator');

/**
 * Main conversion function
 * @param {string} inputPath - Path to markdown file
 * @param {Object} options - Conversion options
 * @returns {string} Generated HTML content
 */
function convertMarkdownToHtml(inputPath, options = {}) {
  // Default options
  const defaultOptions = {
    theme: 'light',
    layout: 'centered',
    showToc: true,
    siteName: 'Markdown Documentation',
    siteUrl: '/',
    author: 'Designer Agent',
    outputDir: './output',
    assetPath: './assets',
    processComponents: true,
    highlightSyntax: true,
    includeJavaScript: true,
    optimizeMobile: true,
    applyAnimations: true,
    generateSocialPreviews: true, // Social preview options
    favicon: 'auto' // Auto-generate favicon from design
  };
  
  // Merge options
  const settings = { ...defaultOptions, ...options };
  
  try {
    // Read markdown file
    // const markdownContent = fs.readFileSync(inputPath, 'utf8');
    const markdownContent = "# Sample content for conceptual implementation";
    
    // Extract frontmatter, content, and design styles
    const { frontmatter, content, designStyles } = extractFrontmatter(markdownContent);
    
    // Override settings with frontmatter values
    const documentSettings = { ...settings, ...frontmatter };
    
    // Apply design styles if available
    if (designStyles) {
      console.log('Applying design styles from natural language description:', frontmatter.design);
      
      // In a real implementation, we would apply the design styles here
      if (designStyles.palette) {
        console.log(`Using ${designStyles.palette} color palette`);
        documentSettings.customStyles = (documentSettings.customStyles || '') + 
          `\n/* Natural language palette: ${designStyles.palette} */\n:root {\n  ${designStyles.cssString}\n}`;
      }
      
      if (designStyles.typography) {
        console.log(`Using ${designStyles.typography} typography`);
      }
      
      if (designStyles.layout) {
        console.log(`Using ${designStyles.layout} layout`);
      }
      
      if (designStyles.formFactor) {
        console.log(`Using ${designStyles.formFactor} form factor`);
      }
    }
    
    // Configure markdown parser
    configureMarkdownParser();
    
    // Convert markdown to HTML
    const htmlContent = parseMarkdown(content, documentSettings);
    
    // Generate table of contents
    const toc = documentSettings.showToc ? generateTableOfContents(htmlContent) : '';
    
    // Get template
    const template = getTemplate(documentSettings.template || 'default');
    
    // Generate social preview metadata if enabled
    let socialPreviewMetadata = '';
    let faviconUrl = documentSettings.favicon;
    
    if (documentSettings.generateSocialPreviews) {
      try {
        // Use the actual socialPreviewGenerator module
        const metadata = socialPreviewGenerator.generateSocialPreviewMetadata(documentSettings, designStyles || {});
        socialPreviewMetadata = socialPreviewGenerator.generateMetadataHTML(metadata);
        
        // Auto-generate favicon if set to 'auto'
        if (documentSettings.favicon === 'auto') {
          faviconUrl = socialPreviewGenerator.generateFavicon(documentSettings, designStyles || {});
        }
      } catch (error) {
        console.warn('Error generating social preview metadata:', error);
        // Fallback to placeholder implementation
        socialPreviewMetadata = generateSocialPreviewMetadataPlaceholder(documentSettings, designStyles);
        
        if (documentSettings.favicon === 'auto') {
          faviconUrl = generatePlaceholderFavicon(documentSettings);
        }
      }
    }
    
    // Process template with variables
    const finalHtml = processTemplate(template, {
      TITLE: documentSettings.title || 'Untitled Document',
      DESCRIPTION: documentSettings.description || '',
      AUTHOR: documentSettings.author,
      THEME: documentSettings.theme,
      LAYOUT: documentSettings.layout,
      SITE_TITLE: documentSettings.siteName,
      HOME_LINK: documentSettings.siteUrl,
      CONTENT: htmlContent,
      TOC_CONTENT: toc,
      SHOW_TOC: documentSettings.showToc && documentSettings.layout !== 'toc',
      TOC_SIDEBAR: documentSettings.layout === 'toc',
      SIDEBAR: documentSettings.layout === 'sidebar',
      SIDEBAR_LINKS: generateSidebarLinks(documentSettings),
      SIDEBAR_CONTENT: documentSettings.sidebarContent || '',
      NAVIGATION_LINKS: generateNavigationLinks(documentSettings),
      PREV_PAGE: !!documentSettings.prevPage,
      PREV_PAGE_URL: documentSettings.prevPage?.url || '',
      PREV_PAGE_TITLE: documentSettings.prevPage?.title || 'Previous',
      NEXT_PAGE: !!documentSettings.nextPage,
      NEXT_PAGE_URL: documentSettings.nextPage?.url || '',
      NEXT_PAGE_TITLE: documentSettings.nextPage?.title || 'Next',
      CUSTOM_STYLES: documentSettings.customStyles || '',
      ADDITIONAL_SCRIPTS: documentSettings.additionalScripts || '',
      FOOTER_TEXT: documentSettings.footerText || '© ' + new Date().getFullYear(),
      GENERATION_DATE: formatDate(new Date()),
      FAVICON: faviconUrl || 'data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22%3E%3Ctext y=%22.9em%22 font-size=%2290%22%3E📄%3C/text%3E%3C/svg%3E',
      SOCIAL_IMAGE: documentSettings.socialImage || '',
      INCLUDE_JAVASCRIPT: documentSettings.includeJavaScript !== false,
      APPLY_ANIMATIONS: documentSettings.applyAnimations !== false,
      AVAILABLE_THEMES: getAvailableThemes(documentSettings),
      SOCIAL_PREVIEW_METADATA: socialPreviewMetadata
    });
    
    // Write output file
    const outputPath = getOutputPath(inputPath, documentSettings);
    // fs.writeFileSync(outputPath, finalHtml);
    
    // Copy assets if specified
    if (documentSettings.assets) {
      copyAssets(documentSettings.assets, path.dirname(outputPath));
    }
    
    return finalHtml;
  } catch (error) {
    console.error('Error converting markdown to HTML:', error);
    throw error;
  }
}

/**
 * Extract frontmatter from markdown content
 * @param {string} content - Markdown content
 * @returns {Object} Object containing frontmatter, content, and design styles
 */
function extractFrontmatter(content) {
  // Check if content has frontmatter
  if (!content.startsWith('---')) {
    return { frontmatter: {}, content, designStyles: null };
  }
  
  // Find the end of frontmatter
  const endIndex = content.indexOf('---', 3);
  if (endIndex === -1) {
    return { frontmatter: {}, content, designStyles: null };
  }
  
  // Extract frontmatter content
  const frontmatterRaw = content.substring(3, endIndex).trim();
  let frontmatter = {};
  let designStyles = null;
  
  try {
    // In a real implementation, this would use the yaml parser
    // frontmatter = yaml.load(frontmatterRaw);
    
    // For this conceptual implementation, we'll manually parse
    const lines = frontmatterRaw.split('\n');
    lines.forEach(line => {
      if (line.includes(':')) {
        const parts = line.split(':');
        const key = parts[0].trim();
        const value = parts.slice(1).join(':').trim();
        
        // Remove quotes if they exist
        const cleanValue = value.replace(/^["'](.*)["']$/, '$1');
        frontmatter[key] = cleanValue;
      }
    });
    
    // Process design language if specified
    if (frontmatter.design) {
      // This would use the designLanguageParser in a real implementation
      // designStyles = designLanguageParser.parseDesignLanguage(frontmatter.design);
      designStyles = processDesignLanguage(frontmatter.design);
    }
    
  } catch (error) {
    console.warn('Error parsing frontmatter:', error);
  }
  
  // Extract content after frontmatter
  const contentWithoutFrontmatter = content.substring(endIndex + 3).trim();
  
  return { frontmatter, content: contentWithoutFrontmatter, designStyles };
}

/**
 * Process design language from frontmatter
 * @param {string} designDescription - Natural language description of design
 * @returns {Object} CSS variables and design properties
 */
function processDesignLanguage(designDescription) {
  // In a real implementation, this would directly use the imported module
  // return designLanguageParser.parseDesignLanguage(designDescription);
  
  // For this conceptual implementation, we'll return a simulated result
  // with some basic parsing of common terms
  
  const design = {
    cssVariables: {},
    palette: null,
    typography: null,
    layout: null,
    formFactor: null
  };
  
  // Detect color schemes
  if (designDescription.includes('earth') || designDescription.includes('earthy')) {
    design.cssVariables['--primary-color'] = '#5E3C2F';
    design.cssVariables['--secondary-color'] = '#A67E5B';
    design.cssVariables['--accent-color'] = '#E9C46A';
    design.cssVariables['--background-color'] = '#F5F1E9';
    design.palette = 'earth';
  } else if (designDescription.includes('ocean') || designDescription.includes('sea')) {
    design.cssVariables['--primary-color'] = '#0077B6';
    design.cssVariables['--secondary-color'] = '#48CAE4';
    design.cssVariables['--accent-color'] = '#FF9E00';
    design.cssVariables['--background-color'] = '#F0F7F9';
    design.palette = 'ocean';
  } else if (designDescription.includes('forest') || designDescription.includes('green')) {
    design.cssVariables['--primary-color'] = '#2D4F33';
    design.cssVariables['--secondary-color'] = '#7D9D7F';
    design.cssVariables['--accent-color'] = '#E3B23C';
    design.cssVariables['--background-color'] = '#F4F7F4';
    design.palette = 'forest';
  } else if (designDescription.includes('pastel')) {
    design.cssVariables['--primary-color'] = '#9CB4CC';
    design.cssVariables['--secondary-color'] = '#BAD7E9';
    design.cssVariables['--accent-color'] = '#FFBD9B';
    design.cssVariables['--background-color'] = '#FFF6F6';
    design.palette = 'pastel';
  } else if (designDescription.includes('vibrant') || designDescription.includes('bright')) {
    design.cssVariables['--primary-color'] = '#6930C3';
    design.cssVariables['--secondary-color'] = '#48BFE3';
    design.cssVariables['--accent-color'] = '#FF5D8F';
    design.cssVariables['--background-color'] = '#FFFFFF';
    design.palette = 'vibrant';
  } else if (designDescription.includes('vintage') || designDescription.includes('retro')) {
    design.cssVariables['--primary-color'] = '#7D5A50';
    design.cssVariables['--secondary-color'] = '#B4846C';
    design.cssVariables['--accent-color'] = '#E5B299';
    design.cssVariables['--background-color'] = '#FDF6EC';
    design.palette = 'vintage';
  } else if (designDescription.includes('minimal') || designDescription.includes('clean')) {
    design.cssVariables['--primary-color'] = '#333333';
    design.cssVariables['--secondary-color'] = '#555555';
    design.cssVariables['--accent-color'] = '#000000';
    design.cssVariables['--background-color'] = '#FFFFFF';
    design.palette = 'minimalist';
  } else if (designDescription.includes('calm') || designDescription.includes('peaceful')) {
    design.cssVariables['--primary-color'] = '#5b9aa0';
    design.cssVariables['--secondary-color'] = '#78c2ad';
    design.cssVariables['--accent-color'] = '#f0ca62';
    design.cssVariables['--background-color'] = '#f0f9fa';
    design.palette = 'calm';
  }
  
  // Detect typography preferences
  if (designDescription.includes('modern') || designDescription.includes('clean')) {
    design.cssVariables['--font-heading'] = "'Montserrat', sans-serif";
    design.cssVariables['--font-body'] = "'Open Sans', sans-serif";
    design.typography = 'modern';
  } else if (designDescription.includes('classic') || designDescription.includes('elegant')) {
    design.cssVariables['--font-heading'] = "'Playfair Display', serif";
    design.cssVariables['--font-body'] = "'Source Serif Pro', serif";
    design.typography = 'classic';
  } else if (designDescription.includes('minimal') || designDescription.includes('simple')) {
    design.cssVariables['--font-heading'] = "'Inter', sans-serif";
    design.cssVariables['--font-body'] = "'Inter', sans-serif";
    design.typography = 'minimal';
  } else if (designDescription.includes('playful') || designDescription.includes('friendly')) {
    design.cssVariables['--font-heading'] = "'Nunito', sans-serif";
    design.cssVariables['--font-body'] = "'Quicksand', sans-serif";
    design.typography = 'playful';
  } else if (designDescription.includes('technical') || designDescription.includes('precise')) {
    design.cssVariables['--font-heading'] = "'IBM Plex Sans', sans-serif";
    design.cssVariables['--font-body'] = "'IBM Plex Sans', sans-serif";
    design.typography = 'technical';
  }
  
  // Detect layout preferences
  if (designDescription.includes('spacious') || designDescription.includes('open')) {
    design.cssVariables['--spacing-multiplier'] = '1.2';
    design.cssVariables['--content-width'] = '920px';
    design.layout = 'spacious';
  } else if (designDescription.includes('compact') || designDescription.includes('dense')) {
    design.cssVariables['--spacing-multiplier'] = '0.8';
    design.cssVariables['--content-width'] = '800px';
    design.layout = 'compact';
  } else if (designDescription.includes('balanced') || designDescription.includes('symmetrical')) {
    design.cssVariables['--spacing-multiplier'] = '1.0';
    design.cssVariables['--content-width'] = '850px';
    design.layout = 'balanced';
  } else if (designDescription.includes('fluid') || designDescription.includes('flexible')) {
    design.cssVariables['--spacing-multiplier'] = '1.0';
    design.cssVariables['--content-width'] = '90%';
    design.cssVariables['--max-content-width'] = '1200px';
    design.layout = 'fluid';
  }
  
  // Detect form factor preferences
  if (designDescription.includes('rounded') || designDescription.includes('soft corners')) {
    design.cssVariables['--border-radius-sm'] = '8px';
    design.cssVariables['--border-radius-md'] = '16px';
    design.cssVariables['--border-radius-lg'] = '24px';
    design.formFactor = 'rounded';
  } else if (designDescription.includes('sharp') || designDescription.includes('angular')) {
    design.cssVariables['--border-radius-sm'] = '0px';
    design.cssVariables['--border-radius-md'] = '0px';
    design.cssVariables['--border-radius-lg'] = '0px';
    design.formFactor = 'sharp';
  } else if (designDescription.includes('geometric') || designDescription.includes('structured')) {
    design.cssVariables['--border-radius-sm'] = '4px';
    design.cssVariables['--border-radius-md'] = '8px';
    design.cssVariables['--border-radius-lg'] = '12px';
    design.formFactor = 'geometric';
  } else if (designDescription.includes('organic') || designDescription.includes('natural')) {
    design.cssVariables['--border-radius-sm'] = 'var(--organic-radius, 8px 4px 12px 6px)';
    design.cssVariables['--border-radius-md'] = 'var(--organic-radius, 12px 8px 16px 12px)';
    design.cssVariables['--border-radius-lg'] = 'var(--organic-radius, 16px 12px 24px 18px)';
    design.formFactor = 'organic';
  }
  
  // Generate CSS variable string
  design.cssString = Object.entries(design.cssVariables)
    .map(([key, value]) => `${key}: ${value};`)
    .join('\n  ');
  
  return design;
}

/**
 * Generate placeholder social preview metadata for demonstration
 * @param {Object} documentSettings - Document settings
 * @param {Object} designStyles - Design styles
 * @returns {string} HTML metadata tags
 */
function generateSocialPreviewMetadataPlaceholder(documentSettings, designStyles) {
  // Create SVG image template for social sharing
  const primaryColor = (designStyles?.cssVariables?.['--primary-color']) || '#5b9aa0';
  const bgColor = (designStyles?.cssVariables?.['--background-color']) || '#ffffff';
  
  // Create a simple SVG image
  const svgImage = `<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="630" viewBox="0 0 1200 630">
    <rect width="1200" height="630" fill="${primaryColor}"/>
    <circle cx="600" cy="315" r="150" fill="${bgColor}"/>
    <path d="M600,200 C550,200 510,240 510,290 C510,329 535,362 570,375 L570,430 L630,430 L630,375 C665,362 690,329 690,290 C690,240 650,200 600,200 z" fill="${primaryColor}"/>
    <text x="600" y="520" font-family="Arial" font-size="48" text-anchor="middle" fill="${bgColor}">${documentSettings.title?.substring(0, 40) || 'Document'}</text>
  </svg>`;
  
  // Encode the SVG for use in meta tags
  const encodedSvg = `data:image/svg+xml,${encodeURIComponent(svgImage)}`;
  
  // Generate metadata tags
  let metadata = '';
  
  // Open Graph / Facebook
  metadata += `<!-- Enhanced Metadata for Social Media/Messaging Previews -->\n`;
  metadata += `<!-- Open Graph / Facebook -->\n`;
  metadata += `<meta property="og:type" content="website">\n`;
  metadata += `<meta property="og:url" content="${documentSettings.url || 'https://example.com/document'}">\n`;
  metadata += `<meta property="og:title" content="${documentSettings.title || 'Document'}">\n`;
  metadata += `<meta property="og:description" content="${documentSettings.description || ''}">\n`;
  metadata += `<meta property="og:image" content="${encodedSvg}">\n`;
  metadata += `<meta property="og:image:width" content="1200">\n`;
  metadata += `<meta property="og:image:height" content="630">\n`;
  metadata += `<meta property="og:site_name" content="${documentSettings.siteName || 'Documentation'}">\n`;
  
  // Twitter / X
  metadata += `\n<!-- Twitter / X -->\n`;
  metadata += `<meta name="twitter:card" content="summary_large_image">\n`;
  metadata += `<meta name="twitter:title" content="${documentSettings.title || 'Document'}">\n`;
  metadata += `<meta name="twitter:description" content="${documentSettings.description || ''}">\n`;
  metadata += `<meta name="twitter:image" content="${encodedSvg}">\n`;
  
  // WhatsApp/Telegram specific
  metadata += `\n<!-- WhatsApp/Telegram specific -->\n`;
  metadata += `<meta property="og:image:alt" content="${documentSettings.title || 'Document'} Documentation">\n`;
  metadata += `<meta property="og:locale" content="${documentSettings.locale || 'en_US'}">\n`;
  
  // iOS/Mobile specific
  metadata += `\n<!-- iOS/Mobile specific -->\n`;
  metadata += `<meta name="apple-mobile-web-app-capable" content="yes">\n`;
  metadata += `<meta name="apple-mobile-web-app-status-bar-style" content="default">\n`;
  metadata += `<meta name="apple-mobile-web-app-title" content="${documentSettings.title?.split(':')[0] || documentSettings.title || 'Document'}">\n`;
  metadata += `<meta name="theme-color" content="${primaryColor}">\n`;
  
  return metadata;
}

/**
 * Generate placeholder favicon for demonstration
 * @param {Object} documentSettings - Document settings
 * @returns {string} SVG data URL
 */
function generatePlaceholderFavicon(documentSettings) {
  // Create a simple SVG favicon
  const svgFavicon = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
    <rect width="100" height="100" rx="50" fill="#5b9aa0"/>
    <path d="M50,25 C36.7,25 26,35.7 26,49 C26,62.3 36.7,73 50,73 C63.3,73 74,62.3 74,49 C74,35.7 63.3,25 50,25 z M50,69 C39,69 30,60 30,49 C30,38 39,29 50,29 C61,29 70,38 70,49 C70,60 61,69 50,69 z" fill="#f0f9fa"/>
    <path d="M50,33 C45,33 41,37 41,42 C41,45.9 43.5,49.1 47,50.3 L47,66 L53,66 L53,50.3 C56.5,49.1 59,45.9 59,42 C59,37 55,33 50,33 z" fill="#f0f9fa"/>
  </svg>`;
  
  // Return as data URL
  return `data:image/svg+xml,${encodeURIComponent(svgFavicon)}`;
}

/**
 * Configure markdown parser with extensions and options
 */
function configureMarkdownParser() {
  // Configure marked options
  /*
  marked.setOptions({
    renderer: new marked.Renderer(),
    highlight: function(code, lang) {
      if (lang && hljs.getLanguage(lang)) {
        return hljs.highlight(lang, code).value;
      }
      return hljs.highlightAuto(code).value;
    },
    pedantic: false,
    gfm: true,
    breaks: false,
    sanitize: false,
    smartLists: true,
    smartypants: true,
    xhtml: false
  });
  */
  
  // Add custom extensions
  extendMarkdownParser();
}

/**
 * Extend markdown parser with custom renderers and rules
 */
function extendMarkdownParser() {
  // Custom renderer for various elements
  /*
  const renderer = new marked.Renderer();
  
  // Custom link renderer
  renderer.link = function(href, title, text) {
    const isExternal = href.startsWith('http') || href.startsWith('//');
    const attributes = [];
    
    if (isExternal) {
      attributes.push('target="_blank" rel="noopener noreferrer"');
    }
    
    if (title) {
      attributes.push(`title="${title}"`);
    }
    
    return `<a href="${href}" ${attributes.join(' ')}>${text}</a>`;
  };
  
  // Custom image renderer
  renderer.image = function(href, title, text) {
    const imgClass = title && title.startsWith(':') ? title.substring(1) : '';
    const imgTitle = title && !title.startsWith(':') ? `title="${title}"` : '';
    return `<figure>
      <img src="${href}" alt="${text}" class="${imgClass}" ${imgTitle} loading="lazy">
      <figcaption>${text}</figcaption>
    </figure>`;
  };
  
  // Custom code block renderer
  renderer.code = function(code, language) {
    const validLanguage = hljs.getLanguage(language) ? language : 'plaintext';
    const highlightedCode = hljs.highlight(validLanguage, code).value;
    
    return `<div class="code-header">
      <span>${validLanguage}</span>
      <button class="copy-code-button" data-clipboard-text="${encodeURIComponent(code)}">Copy</button>
    </div>
    <pre><code class="language-${validLanguage}">${highlightedCode}</code></pre>`;
  };
  
  // Set the custom renderer
  marked.setOptions({ renderer });
  */
}

/**
 * Parse markdown content to HTML
 * @param {string} content - Markdown content
 * @param {Object} settings - Document settings
 * @returns {string} HTML content
 */
function parseMarkdown(content, settings) {
  // Process custom components and extensions
  content = processCustomComponents(content, settings);
  
  // Convert markdown to HTML
  // const html = marked(content);
  const html = `<h1>Sample Title</h1><p>This is a placeholder for the actual HTML content that would be generated.</p>`;
  
  // Post-process HTML
  return postProcessHtml(html, settings);
}

/**
 * Process custom components in markdown
 * @param {string} content - Markdown content
 * @param {Object} settings - Document settings
 * @returns {string} Processed markdown content
 */
function processCustomComponents(content, settings) {
  // Only process components if enabled
  if (!settings.processComponents) {
    return content;
  }

  // Process admonitions/callouts (:::note, :::warning, etc.)
  content = processAdmonitions(content);
  
  // Process tabs component
  content = processTabs(content);
  
  // Process accordions
  content = processAccordions(content);
  
  // Process equations
  content = processEquations(content);

  // Process special containers
  content = processContainers(content);
  
  // Process other custom components
  return content;
}

/**
 * Process admonition blocks in markdown
 * @param {string} content - Markdown content
 * @returns {string} Processed markdown content
 */
function processAdmonitions(content) {
  // The regex to match admonition syntax: :::type Content :::
  const regex = /:::(note|warning|danger|tip|important|info|success|caution|example)(?:\s+(.+?))?(?:\n|\r)([\s\S]*?):::/g;
  
  return content.replace(regex, (match, type, title, body) => {
    const icons = {
      note: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>',
      warning: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>',
      danger: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg>',
      tip: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><path d="M8 14s1.5 2 4 2 4-2 4-2"></path><line x1="9" y1="9" x2="9.01" y2="9"></line><line x1="15" y1="9" x2="15.01" y2="9"></line></svg>',
      important: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>',
      info: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>',
      success: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>',
      caution: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg>',
      example: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 11 12 14 22 4"></polyline><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path></svg>'
    };

    const icon = icons[type] || icons.note;
    const admonitionTitle = title ? title.trim() : type.charAt(0).toUpperCase() + type.slice(1);
    
    return `<div class="admonition ${type}">
      <div class="admonition-icon">${icon}</div>
      <div class="admonition-content">
        <div class="admonition-title">${admonitionTitle}</div>
        <div class="admonition-body">${body.trim()}</div>
      </div>
    </div>`;
  });
}

/**
 * Process tabs component in markdown
 * @param {string} content - Markdown content
 * @returns {string} Processed markdown content
 */
function processTabs(content) {
  // Process the tab group syntax: ===tabs=== Tab1:Content1 Tab2:Content2 ===end===
  const tabGroupRegex = /===tabs===\s*([\s\S]*?)===end===/g;
  let processedContent = content;
  
  processedContent = processedContent.replace(tabGroupRegex, (match, tabsContent) => {
    // Process individual tabs 
    const tabRegex = /\s*@([^@]+)@\s*([\s\S]*?)(?=\s*@[^@]+@|\s*$)/g;
    let tabMatch;
    let tabsHTML = '';
    let tabsNavHTML = '';
    let tabsBodyHTML = '';
    let isFirst = true;
    let tabId = 0;
    
    // Process each tab
    while ((tabMatch = tabRegex.exec(tabsContent))) {
      const tabName = tabMatch[1].trim();
      const tabContent = tabMatch[2].trim();
      const tabIdStr = `tab-${tabId++}`;
      const activeClass = isFirst ? 'active' : '';
      
      // Build tab navigation
      tabsNavHTML += `<button class="tab-button ${activeClass}" data-tab="${tabIdStr}">${tabName}</button>`;
      
      // Build tab content
      tabsBodyHTML += `<div class="tab-content ${activeClass}" id="${tabIdStr}">${tabContent}</div>`;
      
      isFirst = false;
    }
    
    // If no tabs were found, return original content
    if (tabsBodyHTML === '') {
      return match;
    }
    
    // Create the final tabs HTML
    tabsHTML = `<div class="tabs-container">
      <div class="tabs-nav">${tabsNavHTML}</div>
      <div class="tabs-content">${tabsBodyHTML}</div>
    </div>`;
    
    return tabsHTML;
  });
  
  return processedContent;
}

/**
 * Process accordion component in markdown
 * @param {string} content - Markdown content
 * @returns {string} Processed markdown content
 */
function processAccordions(content) {
  // Process the accordion syntax: +++title+++ Content +++end+++
  const accordionRegex = /\+\+\+([^\+]+)\+\+\+\s*([\s\S]*?)\+\+\+end\+\+\+/g;
  
  return content.replace(accordionRegex, (match, title, content) => {
    const accordionId = 'accordion-' + Math.random().toString(36).substr(2, 9);
    
    return `<div class="accordion">
      <details id="${accordionId}">
        <summary class="accordion-header">
          <span class="accordion-title">${title.trim()}</span>
          <span class="accordion-icon"></span>
        </summary>
        <div class="accordion-content">${content.trim()}</div>
      </details>
    </div>`;
  });
}

/**
 * Process math equations in markdown
 * @param {string} content - Markdown content
 * @returns {string} Processed markdown content
 */
function processEquations(content) {
  // Process inline math: $equation$
  content = content.replace(/\$([^\$\n]+)\$/g, (match, equation) => {
    return `<span class="math inline">${equation}</span>`;
  });
  
  // Process block math: $$equation$$
  content = content.replace(/\$\$([\s\S]+?)\$\$/g, (match, equation) => {
    return `<div class="math block">${equation}</div>`;
  });
  
  return content;
}

/**
 * Process special containers in markdown
 * @param {string} content - Markdown content
 * @returns {string} Processed markdown content
 */
function processContainers(content) {
  // Process special containers: :::container type Content :::
  const containerRegex = /:::container\s+([^\s]+)(?:\s+(.+?))?(?:\n|\r)([\s\S]*?):::/g;
  
  return content.replace(containerRegex, (match, type, title, body) => {
    const containerTitle = title ? `<div class="container-title">${title.trim()}</div>` : '';
    
    return `<div class="special-container ${type}">
      ${containerTitle}
      <div class="container-content">${body.trim()}</div>
    </div>`;
  });
}

/**
 * Post-process HTML content
 * @param {string} html - HTML content
 * @param {Object} settings - Document settings
 * @returns {string} Processed HTML content
 */
function postProcessHtml(html, settings) {
  // Process image enhancements
  html = enhanceImages(html, settings);
  
  // Process tables
  html = enhanceTables(html, settings);
  
  // Process code blocks
  html = enhanceCodeBlocks(html, settings);
  
  // Add anchors to headings
  html = addHeadingAnchors(html, settings);
  
  // Process footnotes
  html = processFootnotes(html, settings);
  
  return html;
}

/**
 * Enhance images in HTML content
 * @param {string} html - HTML content
 * @param {Object} settings - Document settings
 * @returns {string} Enhanced HTML content
 */
function enhanceImages(html, settings) {
  // Add lightbox functionality to images
  if (settings.enableLightbox) {
    const imgRegex = /<img src="([^"]+)"([^>]*)>/g;
    html = html.replace(imgRegex, (match, src, attrs) => {
      // Skip for SVG and other non-photo formats
      if (src.match(/\.(svg|gif|webp)(\?.*)?$/i)) {
        return match;
      }
      
      return `<a href="${src}" class="lightbox-link">${match}</a>`;
    });
  }
  
  // Responsive images
  if (settings.optimizeMobile) {
    const figureRegex = /<figure>([\s\S]*?)<\/figure>/g;
    html = html.replace(figureRegex, (match, content) => {
      // Add responsive-figure class
      return `<figure class="responsive-figure">${content}</figure>`;
    });
  }
  
  return html;
}

/**
 * Enhance tables in HTML content
 * @param {string} html - HTML content
 * @param {Object} settings - Document settings
 * @returns {string} Enhanced HTML content
 */
function enhanceTables(html, settings) {
  // Wrap tables in responsive container
  const tableRegex = /<table>([\s\S]*?)<\/table>/g;
  html = html.replace(tableRegex, (match, content) => {
    return `<div class="table-container"><table>${content}</table></div>`;
  });
  
  // Add sorting capability if enabled
  if (settings.tableSorting) {
    const tableHeadRegex = /<thead>([\s\S]*?)<\/thead>/g;
    html = html.replace(tableHeadRegex, (match, content) => {
      return `<thead class="sortable">${content}</thead>`;
    });
  }
  
  return html;
}

/**
 * Enhance code blocks in HTML content
 * @param {string} html - HTML content
 * @param {Object} settings - Document settings
 * @returns {string} Enhanced HTML content
 */
function enhanceCodeBlocks(html, settings) {
  // Add line numbers to code blocks
  if (settings.codeLineNumbers) {
    const codeBlockRegex = /<pre><code class="language-([^"]+)">([\s\S]*?)<\/code><\/pre>/g;
    html = html.replace(codeBlockRegex, (match, language, code) => {
      const lines = code.split('\n');
      let numberedCode = '';
      
      for (let i = 0; i < lines.length; i++) {
        numberedCode += `<span class="line-number">${i + 1}</span>${lines[i]}\n`;
      }
      
      return `<pre class="line-numbers"><code class="language-${language}">${numberedCode}</code></pre>`;
    });
  }
  
  // Add copy button functionality
  if (settings.codeCopyButtons) {
    const preBlockRegex = /<pre(?:\s+class="[^"]*")?>([\s\S]*?)<\/pre>/g;
    html = html.replace(preBlockRegex, (match, content) => {
      if (!match.includes('class="line-numbers"')) {
        return `<div class="code-block-container">
          <div class="code-header">
            <button class="copy-code-button">Copy</button>
          </div>
          <pre>${content}</pre>
        </div>`;
      }
      
      return `<div class="code-block-container">
        <div class="code-header">
          <button class="copy-code-button">Copy</button>
        </div>
        ${match}
      </div>`;
    });
  }
  
  return html;
}

/**
 * Add anchor links to headings
 * @param {string} html - HTML content
 * @param {Object} settings - Document settings
 * @returns {string} Enhanced HTML content
 */
function addHeadingAnchors(html, settings) {
  // Add anchor links to h2, h3, h4, h5, h6
  const headingRegex = /<h([2-6])\s+id="([^"]+)">([\s\S]*?)<\/h\1>/g;
  
  return html.replace(headingRegex, (match, level, id, content) => {
    const anchorClass = settings.headingAnchorClass || 'heading-anchor';
    const anchorIcon = `<svg class="anchor-icon" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M4 9h1v1H4c-1.5 0-3-1.69-3-3.5S2.55 3 4 3h4c1.45 0 3 1.69 3 3.5 0 1.41-.91 2.72-2 3.25V8.59c.58-.45 1-1.27 1-2.09C10 5.22 8.98 4 8 4H4c-.98 0-2 1.22-2 2.5S3 9 4 9zm9-3h-1v1h1c1 0 2 1.22 2 2.5S13.98 12 13 12H9c-.98 0-2-1.22-2-2.5 0-.83.42-1.64 1-2.09V6.25c-1.09.53-2 1.84-2 3.25C6 11.31 7.55 13 9 13h4c1.45 0 3-1.69 3-3.5S14.5 6 13 6z"></path></svg>`;
    
    return `<h${level} id="${id}">${content} <a href="#${id}" class="${anchorClass}" aria-label="anchor" tabindex="-1">${anchorIcon}</a></h${level}>`;
  });
}

/**
 * Process footnotes in HTML
 * @param {string} html - HTML content
 * @param {Object} settings - Document settings
 * @returns {string} Processed HTML content
 */
function processFootnotes(html, settings) {
  // Process footnote references and definitions
  // This is a simple implementation and might need more complex processing
  
  // First find all footnote definitions
  const definitions = {};
  const defRegex = /<p>\[\^(\d+)\]:\s*([\s\S]*?)<\/p>/g;
  
  // Collect definitions
  html = html.replace(defRegex, (match, id, content) => {
    definitions[id] = content.trim();
    return '';
  });
  
  // Now replace references with footnote links
  const refRegex = /\[\^(\d+)\]/g;
  html = html.replace(refRegex, (match, id) => {
    return `<sup class="footnote-ref"><a href="#footnote-${id}" id="footnote-ref-${id}">${id}</a></sup>`;
  });
  
  // Add the footnotes section at the end if we have any definitions
  if (Object.keys(definitions).length > 0) {
    let footnotesList = '';
    
    for (const [id, content] of Object.entries(definitions)) {
      footnotesList += `<li id="footnote-${id}">${content} <a href="#footnote-ref-${id}" class="footnote-backref">↩</a></li>`;
    }
    
    if (footnotesList) {
      html += `<section class="footnotes">
        <hr>
        <ol>
          ${footnotesList}
        </ol>
      </section>`;
    }
  }
  
  return html;
}

/**
 * Generate table of contents from HTML content
 * @param {string} html - HTML content
 * @param {Object} settings - Document settings
 * @returns {string} Table of contents HTML
 */
function generateTableOfContents(html, settings) {
  // Extract headings from HTML
  const headings = [];
  const headingRegex = /<h([2-6])\s+id="([^"]+)">(.*?)<\/h\1>/g;
  let match;
  
  while ((match = headingRegex.exec(html)) !== null) {
    headings.push({
      level: parseInt(match[1], 10),
      id: match[2],
      text: match[3].replace(/<.*?>/g, '') // Remove HTML tags from heading text
    });
  }
  
  if (headings.length === 0) {
    return '';
  }
  
  // Generate nested TOC
  let toc = '<ul class="toc-list">\n';
  
  let currentLevel = 2;
  let levelStack = [currentLevel];
  
  headings.forEach((heading) => {
    if (heading.level > currentLevel) {
      // Deeper level, add a new list
      for (let i = currentLevel; i < heading.level; i++) {
        toc += '<ul class="toc-sublist">\n';
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
    const headingClass = `toc-h${heading.level}`;
    toc += `<li class="${headingClass}"><a href="#${heading.id}">${heading.text}</a></li>\n`;
  });
  
  // Close remaining lists
  for (let i = 1; i < levelStack.length; i++) {
    toc += '</ul>\n';
  }
  
  toc += '</ul>\n';
  return toc;
}

/**
 * Get HTML template
 * @param {string} templateName - Template name
 * @returns {string} Template HTML
 */
function getTemplate(templateName) {
  // Get template file path
  // const templatePath = path.join(__dirname, 'templates', `${templateName}.html`);
  
  // Read template file
  // return fs.readFileSync(templatePath, 'utf8');
  
  // For implementation, use template-enhanced.html
  try {
    // const templatePath = path.join(__dirname, `${templateName}-enhanced.html`);
    // const templateContent = fs.readFileSync(templatePath, 'utf8');
    // return templateContent;
    
    // For now, return a robust template that supports social preview metadata
    return `<!DOCTYPE html>
<html lang="en" data-theme="{{THEME}}">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{TITLE}}</title>
  <meta name="description" content="{{DESCRIPTION}}">
  <meta name="author" content="{{AUTHOR}}">
  <meta name="generator" content="Designer Agent Markdown System">
  
  <!-- Favicons -->
  <link rel="icon" href="{{FAVICON}}" type="image/svg+xml">
  
  <!-- Social Media Preview Metadata -->
  {{SOCIAL_PREVIEW_METADATA}}
  
  <style>
    /* Base styles would go here */
    {{CUSTOM_STYLES}}
  </style>
  
  {{ADDITIONAL_SCRIPTS}}
</head>
<body class="layout-{{LAYOUT}}">
  <div class="container">
    {{#SHOW_TOC}}
    <div class="toc">
      <div class="toc-title">Table of Contents</div>
      {{TOC_CONTENT}}
    </div>
    {{/SHOW_TOC}}
    
    <main class="content">
      {{CONTENT}}
    </main>
    
    <footer>
      <p>{{FOOTER_TEXT}}</p>
      <p>Generated by <strong>Designer Agent</strong> | {{GENERATION_DATE}}</p>
    </footer>
  </div>
  
  {{#INCLUDE_JAVASCRIPT}}
  <script>
    // JavaScript would go here
    document.addEventListener('DOMContentLoaded', function() {
      // Theme switching
      const themeButtons = document.querySelectorAll('.theme-button');
      if (themeButtons.length > 0) {
        themeButtons.forEach(button => {
          button.addEventListener('click', () => {
            const theme = button.dataset.theme;
            document.documentElement.setAttribute('data-theme', theme);
            localStorage.setItem('preferred-theme', theme);
          });
        });
        
        // Set theme from local storage or default
        const savedTheme = localStorage.getItem('preferred-theme') || '{{THEME}}';
        document.documentElement.setAttribute('data-theme', savedTheme);
      }
    });
  </script>
  {{/INCLUDE_JAVASCRIPT}}
</body>
</html>`;
  } catch (error) {
    console.warn('Error loading template:', error);
    // Fall back to basic template
    return `<!DOCTYPE html>
<html lang="en" data-theme="{{THEME}}">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{TITLE}}</title>
  <meta name="description" content="{{DESCRIPTION}}">
  <meta name="author" content="{{AUTHOR}}">
  
  <!-- Social Media Preview Metadata -->
  {{SOCIAL_PREVIEW_METADATA}}
  
  <!-- Favicon -->
  <link rel="icon" href="{{FAVICON}}" type="image/svg+xml">
  
  <style>
    /* Base styles would go here */
    {{CUSTOM_STYLES}}
  </style>
</head>
<body>
  <main>
    {{CONTENT}}
  </main>
</body>
</html>`;
  }
}

/**
 * Process template with variables
 * @param {string} template - Template HTML
 * @param {Object} variables - Variables to replace in template
 * @returns {string} Processed HTML
 */
function processTemplate(template, variables) {
  // Replace variables in template
  let result = template;
  
  // Replace simple variables
  Object.entries(variables).forEach(([key, value]) => {
    const regex = new RegExp(`{{${key}}}`, 'g');
    result = result.replace(regex, value !== undefined ? value : '');
  });
  
  // Process conditional blocks
  Object.entries(variables).forEach(([key, value]) => {
    // Process positive conditions {{#KEY}}content{{/KEY}}
    const blockRegex = new RegExp(`{{#${key}}}([\\s\\S]*?){{/${key}}}`, 'g');
    if (value) {
      // Keep the content and remove the tags
      result = result.replace(blockRegex, '$1');
    } else {
      // Remove the entire block
      result = result.replace(blockRegex, '');
    }
    
    // Process negative conditions {{^KEY}}content{{/KEY}}
    const negBlockRegex = new RegExp(`{{\\^${key}}}([\\s\\S]*?){{/${key}}}`, 'g');
    if (!value) {
      // Keep the content and remove the tags for negative condition
      result = result.replace(negBlockRegex, '$1');
    } else {
      // Remove the entire block for negative condition
      result = result.replace(negBlockRegex, '');
    }
  });
  
  // Add additional head content for JavaScript capabilities
  if (variables.INCLUDE_JAVASCRIPT) {
    const headClosingTag = '</head>';
    const jsScripts = getJavaScriptContent(variables);
    result = result.replace(headClosingTag, `${jsScripts}\n${headClosingTag}`);
  }
  
  return result;
}

/**
 * Get JavaScript content for template
 * @param {Object} variables - Template variables
 * @returns {string} JavaScript content
 */
function getJavaScriptContent(variables) {
  // Base JavaScript for functionality
  let jsContent = `<script>
    document.addEventListener('DOMContentLoaded', function() {
      // Theme switching
      const themeButtons = document.querySelectorAll('.theme-button');
      themeButtons.forEach(button => {
        button.addEventListener('click', () => {
          const theme = button.dataset.theme;
          document.documentElement.setAttribute('data-theme', theme);
          localStorage.setItem('preferred-theme', theme);
          
          // Update active state
          themeButtons.forEach(btn => btn.classList.remove('active'));
          button.classList.add('active');
        });
      });
      
      // Set theme from local storage or default
      const savedTheme = localStorage.getItem('preferred-theme') || '${variables.THEME}';
      document.documentElement.setAttribute('data-theme', savedTheme);
      const activeThemeButton = document.querySelector('.theme-button[data-theme="' + savedTheme + '"]');
      if (activeThemeButton) {
        activeThemeButton.classList.add('active');
      }
      
      // Sidebar toggle for mobile
      const sidebarToggle = document.querySelector('.sidebar-toggle');
      if (sidebarToggle) {
        sidebarToggle.addEventListener('click', () => {
          document.body.classList.toggle('sidebar-open');
        });
      }
      
      // Tab system
      const tabButtons = document.querySelectorAll('.tab-button');
      tabButtons.forEach(button => {
        button.addEventListener('click', () => {
          const tabId = button.dataset.tab;
          const tabContainer = button.closest('.tabs-container');
          
          // Deactivate all tabs
          tabContainer.querySelectorAll('.tab-button').forEach(btn => btn.classList.remove('active'));
          tabContainer.querySelectorAll('.tab-content').forEach(content => content.classList.remove('active'));
          
          // Activate selected tab
          button.classList.add('active');
          tabContainer.querySelector('#' + tabId).classList.add('active');
        });
      });
      
      // Copy code buttons
      const copyButtons = document.querySelectorAll('.copy-code-button');
      copyButtons.forEach(button => {
        button.addEventListener('click', () => {
          const codeBlock = button.closest('.code-block-container').querySelector('code');
          const code = codeBlock.innerText;
          
          navigator.clipboard.writeText(code).then(() => {
            // Success feedback
            button.innerText = 'Copied!';
            setTimeout(() => {
              button.innerText = 'Copy';
            }, 2000);
          }).catch(err => {
            console.error('Failed to copy code:', err);
            button.innerText = 'Error!';
            setTimeout(() => {
              button.innerText = 'Copy';
            }, 2000);
          });
        });
      });
      
      // Reading progress bar
      const progressBar = document.querySelector('.progress-bar');
      if (progressBar) {
        window.addEventListener('scroll', () => {
          const winScroll = document.body.scrollTop || document.documentElement.scrollTop;
          const height = document.documentElement.scrollHeight - document.documentElement.clientHeight;
          const scrolled = (winScroll / height) * 100;
          progressBar.style.width = scrolled + '%';
        });
      }
      
      // Heading anchor hover effect
      const headingAnchors = document.querySelectorAll('.heading-anchor');
      headingAnchors.forEach(anchor => {
        const heading = anchor.parentElement;
        heading.addEventListener('mouseenter', () => {
          anchor.style.opacity = '1';
        });
        heading.addEventListener('mouseleave', () => {
          anchor.style.opacity = '0';
        });
      });
      
      // Lightbox functionality
      const lightboxLinks = document.querySelectorAll('.lightbox-link');
      if (lightboxLinks.length > 0) {
        // Create lightbox container if it doesn't exist
        let lightbox = document.querySelector('#lightbox-container');
        if (!lightbox) {
          lightbox = document.createElement('div');
          lightbox.id = 'lightbox-container';
          lightbox.innerHTML = '<div class="lightbox-content"><img id="lightbox-image"><div class="lightbox-close">&times;</div></div>';
          document.body.appendChild(lightbox);
          
          // Close lightbox on click
          lightbox.addEventListener('click', (e) => {
            if (e.target !== document.querySelector('#lightbox-image')) {
              lightbox.classList.remove('active');
            }
          });
        }
        
        // Add click handlers to lightbox links
        lightboxLinks.forEach(link => {
          link.addEventListener('click', (e) => {
            e.preventDefault();
            const imgSrc = link.getAttribute('href');
            document.querySelector('#lightbox-image').src = imgSrc;
            lightbox.classList.add('active');
          });
        });
      }
      
      // Table of contents highlight current section
      const tocLinks = document.querySelectorAll('.toc-list a');
      const headings = document.querySelectorAll('h2, h3, h4, h5, h6');
      
      if (tocLinks.length > 0 && headings.length > 0) {
        const observerOptions = {
          rootMargin: '-100px 0px -70% 0px',
          threshold: 0
        };
        
        const headingObserver = new IntersectionObserver((entries) => {
          entries.forEach(entry => {
            if (entry.isIntersecting) {
              // Remove active from all links
              tocLinks.forEach(link => link.classList.remove('active'));
              
              // Add active to current link
              const activeLink = document.querySelector('.toc-list a[href="#' + entry.target.id + '"]');
              if (activeLink) {
                activeLink.classList.add('active');
              }
            }
          });
        }, observerOptions);
        
        headings.forEach(heading => {
          headingObserver.observe(heading);
        });
      }
    });
  </script>`;
  
  return jsContent;
}

/**
 * Generate sidebar links
 * @param {Object} settings - Document settings
 * @returns {string} Sidebar links HTML
 */
function generateSidebarLinks(settings) {
  if (!settings.sidebarLinks || !Array.isArray(settings.sidebarLinks)) {
    return '';
  }
  
  return settings.sidebarLinks.map(link => {
    const isActive = link.active ? 'active' : '';
    return `<li><a href="${link.url}" class="${isActive}">${link.title}</a></li>`;
  }).join('');
}

/**
 * Generate navigation links
 * @param {Object} settings - Document settings
 * @returns {string} Navigation links HTML
 */
function generateNavigationLinks(settings) {
  if (!settings.navigationLinks || !Array.isArray(settings.navigationLinks)) {
    return '';
  }
  
  return settings.navigationLinks.map(link => {
    return `<a href="${link.url}">${link.title}</a>`;
  }).join('');
}

/**
 * Get available themes for theme switcher
 * @param {Object} settings - Document settings
 * @returns {string} HTML for theme switcher buttons
 */
function getAvailableThemes(settings) {
  // Default themes
  const defaultThemes = [
    { name: 'light', label: 'Light' },
    { name: 'dark', label: 'Dark' },
    { name: 'sepia', label: 'Sepia' },
    { name: 'minimal', label: 'Minimal' },
    { name: 'ocean', label: 'Ocean' }
  ];
  
  // Add custom themes if specified
  const customThemes = settings.customThemes || [];
  const allThemes = [...defaultThemes, ...customThemes];
  
  // Build theme buttons HTML
  return allThemes.map(theme => {
    return `<button class="theme-button theme-${theme.name}" data-theme="${theme.name}" title="${theme.label} theme"></button>`;
  }).join('');
}

/**
 * Get output file path
 * @param {string} inputPath - Input file path
 * @param {Object} settings - Document settings
 * @returns {string} Output file path
 */
function getOutputPath(inputPath, settings) {
  // Get output directory
  // const outputDir = settings.outputDir || path.dirname(inputPath);
  
  // Get output filename
  // const inputFilename = path.basename(inputPath, path.extname(inputPath));
  // const outputFilename = settings.outputFilename || `${inputFilename}.html`;
  
  // Return full output path
  // return path.join(outputDir, outputFilename);
  
  // For conceptual implementation, return placeholder
  return 'output.html';
}

/**
 * Copy assets to output directory
 * @param {string|Array} assets - Asset paths
 * @param {string} outputDir - Output directory
 */
function copyAssets(assets, outputDir) {
  // If assets is a string, convert to array
  const assetPaths = Array.isArray(assets) ? assets : [assets];
  
  // Copy each asset
  /*
  assetPaths.forEach(assetPath => {
    const isDirectory = fs.lstatSync(assetPath).isDirectory();
    
    if (isDirectory) {
      // Copy directory recursively
      fs.cpSync(assetPath, path.join(outputDir, path.basename(assetPath)), { recursive: true });
    } else {
      // Copy file
      fs.copyFileSync(assetPath, path.join(outputDir, path.basename(assetPath)));
    }
  });
  */
}

/**
 * Format date
 * @param {Date} date - Date to format
 * @returns {string} Formatted date
 */
function formatDate(date) {
  return date.toISOString().split('T')[0];
}

// Export the main function
module.exports = {
  convertMarkdownToHtml
};

// For conceptual implementation, log example output
console.log('Markdown Converter Enhanced - Conceptual Implementation');
console.log('Usage: convertMarkdownToHtml(inputPath, options)');
console.log('New feature: Social media preview generation for messaging apps');