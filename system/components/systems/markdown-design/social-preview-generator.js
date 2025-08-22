/**
 * Social Preview Generator
 * Designer Agent - Advanced Markdown System
 * 
 * This module generates social media and messaging app preview metadata based on
 * document content and design settings. It creates embedded images and appropriate
 * metadata tags for platforms like Facebook, Twitter, WhatsApp, and Telegram.
 */

/**
 * Generate social media preview metadata
 * @param {Object} document - Document properties (title, description, etc.)
 * @param {Object} design - Design variables from design-language-parser
 * @returns {Object} Social preview metadata tags
 */
function generateSocialPreviewMetadata(document, design) {
  const metadata = {
    // Basic Open Graph
    ogType: 'website',
    ogUrl: document.url || 'https://example.com/document',
    ogTitle: document.title || 'Document',
    ogDescription: document.description || '',
    ogSiteName: document.siteName || 'Documentation',
    
    // Twitter Card
    twitterCard: 'summary_large_image',
    twitterTitle: document.title || 'Document',
    twitterDescription: document.description || '',
    
    // WhatsApp/Telegram specific
    ogImageAlt: document.title || 'Document Preview',
    ogLocale: document.locale || 'en_US',
    
    // iOS/Mobile specific
    appleMobileWebAppCapable: 'yes',
    appleMobileWebAppStatusBarStyle: 'default',
    appleMobileWebAppTitle: document.shortTitle || document.title || 'Document',
    themeColor: design['--primary-color'] || '#5b9aa0',
    
    // Images
    ogImage: generateSocialPreviewImage(document, design),
    ogImageWidth: '1200',
    ogImageHeight: '630',
    twitterImage: generateSocialPreviewImage(document, design)
  };
  
  return metadata;
}

/**
 * Generate HTML metadata tags
 * @param {Object} metadata - Metadata properties
 * @returns {string} HTML tags as string
 */
function generateMetadataHTML(metadata) {
  let html = '';
  
  // Open Graph / Facebook
  html += `<meta property="og:type" content="${metadata.ogType}">\n`;
  html += `<meta property="og:url" content="${metadata.ogUrl}">\n`;
  html += `<meta property="og:title" content="${metadata.ogTitle}">\n`;
  html += `<meta property="og:description" content="${metadata.ogDescription}">\n`;
  html += `<meta property="og:image" content="${metadata.ogImage}">\n`;
  html += `<meta property="og:image:width" content="${metadata.ogImageWidth}">\n`;
  html += `<meta property="og:image:height" content="${metadata.ogImageHeight}">\n`;
  html += `<meta property="og:site_name" content="${metadata.ogSiteName}">\n`;
  
  // Twitter / X
  html += `<meta name="twitter:card" content="${metadata.twitterCard}">\n`;
  html += `<meta name="twitter:title" content="${metadata.twitterTitle}">\n`;
  html += `<meta name="twitter:description" content="${metadata.twitterDescription}">\n`;
  html += `<meta name="twitter:image" content="${metadata.twitterImage}">\n`;
  
  // WhatsApp/Telegram specific
  html += `<meta property="og:image:alt" content="${metadata.ogImageAlt}">\n`;
  html += `<meta property="og:locale" content="${metadata.ogLocale}">\n`;
  
  // iOS/Mobile specific
  html += `<meta name="apple-mobile-web-app-capable" content="${metadata.appleMobileWebAppCapable}">\n`;
  html += `<meta name="apple-mobile-web-app-status-bar-style" content="${metadata.appleMobileWebAppStatusBarStyle}">\n`;
  html += `<meta name="apple-mobile-web-app-title" content="${metadata.appleMobileWebAppTitle}">\n`;
  html += `<meta name="theme-color" content="${metadata.themeColor}">\n`;
  
  return html;
}

/**
 * Generate a social preview image as embedded SVG data URL
 * @param {Object} document - Document properties
 * @param {Object} design - Design variables
 * @returns {string} Data URL of SVG image
 */
function generateSocialPreviewImage(document, design) {
  // Extract colors from design
  const primaryColor = design['--primary-color'] || '#5b9aa0';
  const textColor = design['--text-color'] || '#333333';
  const bgColor = design['--background-color'] || '#ffffff';
  const accentColor = design['--accent-color'] || '#f0ca62';
  
  // Build SVG image
  const svgWidth = 1200;
  const svgHeight = 630;
  const title = document.title || 'Document';
  const description = document.description || '';
  
  // Create SVG image
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${svgWidth}" height="${svgHeight}" viewBox="0 0 ${svgWidth} ${svgHeight}">`;
  
  // Background
  svg += `<rect width="${svgWidth}" height="${svgHeight}" fill="${primaryColor}"/>`;
  
  // Add decorative elements based on design
  if (design['--form-factor'] === 'rounded' || design['--border-radius-lg'] > 10) {
    // Rounded style with circles
    svg += `<circle cx="${svgWidth * 0.5}" cy="${svgHeight * 0.5}" r="${svgHeight * 0.25}" fill="${bgColor}" opacity="0.8"/>`;
    if (document.icon) {
      // If document has an icon, use it
      svg += document.icon;
    } else {
      // Default icon shape based on document type
      const iconType = document.type || 'document';
      if (iconType === 'agent') {
        svg += `<path d="M${svgWidth * 0.5},${svgHeight * 0.32} C${svgWidth * 0.46},${svgHeight * 0.32} ${svgWidth * 0.425},${svgHeight * 0.38} ${svgWidth * 0.425},${svgHeight * 0.46} C${svgWidth * 0.425},${svgHeight * 0.52} ${svgWidth * 0.446},${svgHeight * 0.575} ${svgWidth * 0.475},${svgHeight * 0.595} L${svgWidth * 0.475},${svgHeight * 0.68} L${svgWidth * 0.525},${svgHeight * 0.68} L${svgWidth * 0.525},${svgHeight * 0.595} C${svgWidth * 0.554},${svgHeight * 0.575} ${svgWidth * 0.575},${svgHeight * 0.52} ${svgWidth * 0.575},${svgHeight * 0.46} C${svgWidth * 0.575},${svgHeight * 0.38} ${svgWidth * 0.54},${svgHeight * 0.32} ${svgWidth * 0.5},${svgHeight * 0.32} z" fill="${primaryColor}"/>`;
      } else {
        // Document icon
        svg += `<path d="M${svgWidth * 0.425},${svgHeight * 0.35} L${svgWidth * 0.425},${svgHeight * 0.65} L${svgWidth * 0.575},${svgHeight * 0.65} L${svgWidth * 0.575},${svgHeight * 0.35} Z" fill="${primaryColor}"/>`;
        svg += `<path d="M${svgWidth * 0.445},${svgHeight * 0.4} L${svgWidth * 0.555},${svgHeight * 0.4}" stroke="${primaryColor}" stroke-width="4"/>`;
        svg += `<path d="M${svgWidth * 0.445},${svgHeight * 0.45} L${svgWidth * 0.555},${svgHeight * 0.45}" stroke="${primaryColor}" stroke-width="4"/>`;
        svg += `<path d="M${svgWidth * 0.445},${svgHeight * 0.5} L${svgWidth * 0.555},${svgHeight * 0.5}" stroke="${primaryColor}" stroke-width="4"/>`;
      }
    }
  } else {
    // More geometric style
    svg += `<rect x="${svgWidth * 0.25}" y="${svgHeight * 0.25}" width="${svgWidth * 0.5}" height="${svgHeight * 0.5}" fill="${bgColor}" opacity="0.9"/>`;
  }
  
  // Add title text
  svg += `<text x="${svgWidth * 0.5}" y="${svgHeight * 0.5 + 180}" font-family="Arial, sans-serif" font-size="48" font-weight="bold" text-anchor="middle" fill="${bgColor}">${escapeXml(truncateText(title, 40))}</text>`;
  
  // Add description if not too long
  if (description && description.length < 100) {
    svg += `<text x="${svgWidth * 0.5}" y="${svgHeight * 0.5 + 240}" font-family="Arial, sans-serif" font-size="24" text-anchor="middle" fill="${bgColor}">${escapeXml(truncateText(description, 80))}</text>`;
  }
  
  // Close SVG
  svg += `</svg>`;
  
  // Return as data URL
  return `data:image/svg+xml,${encodeURIComponent(svg)}`;
}

/**
 * Truncate text to a maximum length
 * @param {string} text - Text to truncate
 * @param {number} maxLength - Maximum length
 * @returns {string} Truncated text
 */
function truncateText(text, maxLength) {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength - 3) + '...';
}

/**
 * Escape XML special characters
 * @param {string} str - String to escape
 * @returns {string} Escaped string
 */
function escapeXml(str) {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&apos;');
}

/**
 * Generate a favicon as embedded SVG data URL
 * @param {Object} document - Document properties
 * @param {Object} design - Design variables
 * @returns {string} Data URL of SVG favicon
 */
function generateFavicon(document, design) {
  // Extract colors
  const primaryColor = design['--primary-color'] || '#5b9aa0';
  const bgColor = design['--background-color'] || '#ffffff';
  
  // Create simple SVG favicon
  const svgSize = 100;
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${svgSize} ${svgSize}">`;
  
  if (design['--form-factor'] === 'rounded' || design['--border-radius-lg'] > 10) {
    // Rounded favicon
    svg += `<rect width="${svgSize}" height="${svgSize}" rx="${svgSize/2}" fill="${primaryColor}"/>`;
    
    // Simple icon based on document type
    if (document.type === 'agent') {
      svg += `<path d="${svgSize/2},${svgSize*0.25} C${svgSize*0.367},${svgSize*0.25} ${svgSize*0.26},${svgSize*0.357} ${svgSize*0.26},${svgSize*0.49} C${svgSize*0.26},${svgSize*0.623} ${svgSize*0.367},${svgSize*0.73} ${svgSize/2},${svgSize*0.73} C${svgSize*0.633},${svgSize*0.73} ${svgSize*0.74},${svgSize*0.623} ${svgSize*0.74},${svgSize*0.49} C${svgSize*0.74},${svgSize*0.357} ${svgSize*0.633},${svgSize*0.25} ${svgSize/2},${svgSize*0.25} z M${svgSize/2},${svgSize*0.69} C${svgSize*0.39},${svgSize*0.69} ${svgSize*0.3},${svgSize*0.6} ${svgSize*0.3},${svgSize*0.49} C${svgSize*0.3},${svgSize*0.38} ${svgSize*0.39},${svgSize*0.29} ${svgSize/2},${svgSize*0.29} C${svgSize*0.61},${svgSize*0.29} ${svgSize*0.7},${svgSize*0.38} ${svgSize*0.7},${svgSize*0.49} C${svgSize*0.7},${svgSize*0.6} ${svgSize*0.61},${svgSize*0.69} ${svgSize/2},${svgSize*0.69} z" fill="${bgColor}"/>`;
      svg += `<path d="${svgSize/2},${svgSize*0.33} C${svgSize*0.45},${svgSize*0.33} ${svgSize*0.41},${svgSize*0.37} ${svgSize*0.41},${svgSize*0.42} C${svgSize*0.41},${svgSize*0.459} ${svgSize*0.435},${svgSize*0.491} ${svgSize*0.47},${svgSize*0.503} L${svgSize*0.47},${svgSize*0.66} L${svgSize*0.53},${svgSize*0.66} L${svgSize*0.53},${svgSize*0.503} C${svgSize*0.565},${svgSize*0.491} ${svgSize*0.59},${svgSize*0.459} ${svgSize*0.59},${svgSize*0.42} C${svgSize*0.59},${svgSize*0.37} ${svgSize*0.55},${svgSize*0.33} ${svgSize/2},${svgSize*0.33} z" fill="${bgColor}"/>`;
    } else {
      // Document-type icon
      svg += `<path d="${svgSize/2},${svgSize*0.25} C${svgSize*0.367},${svgSize*0.25} ${svgSize*0.26},${svgSize*0.357} ${svgSize*0.26},${svgSize*0.49} C${svgSize*0.26},${svgSize*0.623} ${svgSize*0.367},${svgSize*0.73} ${svgSize/2},${svgSize*0.73} C${svgSize*0.633},${svgSize*0.73} ${svgSize*0.74},${svgSize*0.623} ${svgSize*0.74},${svgSize*0.49} C${svgSize*0.74},${svgSize*0.357} ${svgSize*0.633},${svgSize*0.25} ${svgSize/2},${svgSize*0.25} z" fill="${bgColor}"/>`;
    }
  } else {
    // More angular favicon
    const borderRadius = Math.min(parseInt(design['--border-radius-lg'] || 0, 10), 15);
    svg += `<rect width="${svgSize}" height="${svgSize}" rx="${borderRadius}" fill="${primaryColor}"/>`;
    
    // Simple centered initial or symbol
    const initial = (document.title || 'D').charAt(0).toUpperCase();
    svg += `<text x="${svgSize/2}" y="${svgSize*0.65}" font-family="Arial, sans-serif" font-size="${svgSize*0.5}" font-weight="bold" text-anchor="middle" fill="${bgColor}">${initial}</text>`;
  }
  
  // Close SVG
  svg += `</svg>`;
  
  // Return as data URL
  return `data:image/svg+xml,${encodeURIComponent(svg)}`;
}

module.exports = {
  generateSocialPreviewMetadata,
  generateMetadataHTML,
  generateSocialPreviewImage,
  generateFavicon
};