/**
 * Design Language Parser
 * Designer Agent - Advanced Markdown System
 * 
 * This module translates natural language design descriptions into CSS properties.
 * It processes aesthetic descriptions, color palettes, typography, and layout preferences
 * expressed in natural language and converts them to concrete CSS values.
 * 
 * DESIGN FREEDOM PRIORITY:
 * The underlying AI understands design principles and can interpret a wide range
 * of descriptive terms, going beyond simple keyword matching. Creative judgment
 * is applied to ensure the resulting design is aesthetically coherent.
 */

/**
 * Color Palette Definitions
 * 
 * Each palette defines a set of colors that work well together based on color theory
 * and established design principles.
 */
const COLOR_PALETTES = {
  // Natural/Environmental Palettes
  'earth tones': {
    primary: '#5E3C2F', // Rich brown
    secondary: '#A67E5B', // Tan/sand
    accent: '#E9C46A', // Soft yellow
    text: '#2B2118', // Dark brown
    background: '#F5F1E9', // Off-white/cream
    muted: '#D5C3AA', // Light taupe
    description: 'Warm, grounding colors inspired by soil, wood, and natural materials'
  },
  'forest': {
    primary: '#2D4F33', // Deep green
    secondary: '#7D9D7F', // Sage green
    accent: '#E3B23C', // Goldenrod
    text: '#1E2923', // Dark forest green
    background: '#F4F7F4', // Pale mint
    muted: '#B4C5B0', // Light sage
    description: 'Rich greens with earthy accents inspired by woodland settings'
  },
  'ocean': {
    primary: '#0077B6', // Deep blue
    secondary: '#48CAE4', // Bright blue
    accent: '#FF9E00', // Orange
    text: '#1E3A5F', // Navy blue
    background: '#F0F7F9', // Pale blue
    muted: '#90E0EF', // Light blue
    description: 'Cool blues with complementary warm accents reminiscent of coastal scenes'
  },
  'desert': {
    primary: '#CB904D', // Adobe orange
    secondary: '#DF9A57', // Sandstone
    accent: '#C05555', // Terra cotta red
    text: '#3D2B1F', // Dark brown
    background: '#F7F3EB', // Sand
    muted: '#E4D6A7', // Light sand
    description: 'Warm oranges and browns inspired by arid landscapes'
  },
  'mountain': {
    primary: '#3A506B', // Slate blue
    secondary: '#556B83', // Medium blue-gray
    accent: '#D4AF37', // Gold
    text: '#202A36', // Dark blue-gray
    background: '#F6F8FA', // Snow white
    muted: '#C3D0DF', // Pale blue-gray
    description: 'Cool blues and grays with golden highlights inspired by alpine settings'
  },
  
  // Style-Based Palettes
  'minimalist': {
    primary: '#333333', // Dark gray
    secondary: '#555555', // Medium gray
    accent: '#000000', // Black
    text: '#1A1A1A', // Nearly black
    background: '#FFFFFF', // Pure white
    muted: '#E5E5E5', // Light gray
    description: 'Monochromatic scheme with minimal contrast focused on typography and space'
  },
  'pastel': {
    primary: '#9CB4CC', // Pastel blue
    secondary: '#BAD7E9', // Lighter blue
    accent: '#FFBD9B', // Pastel orange
    text: '#2E4053', // Dark blue-gray
    background: '#FFF6F6', // Off-white pink
    muted: '#EAFDFC', // Pale cyan
    description: 'Soft, desaturated colors with a gentle, soothing quality'
  },
  'bold': {
    primary: '#1A1A2E', // Deep navy
    secondary: '#16213E', // Dark blue
    accent: '#FF0000', // Bright red
    text: '#121212', // Nearly black
    background: '#FFFFFF', // Pure white
    muted: '#E0E0E0', // Light gray
    description: 'High contrast with striking accents for impactful visuals'
  },
  'vintage': {
    primary: '#7D5A50', // Faded brown
    secondary: '#B4846C', // Dusty rose
    accent: '#E5B299', // Peach
    text: '#3F2E28', // Dark brown
    background: '#FDF6EC', // Cream
    muted: '#D8CFC4', // Beige
    description: 'Muted, desaturated tones reminiscent of aged photographs and materials'
  },
  'vibrant': {
    primary: '#6930C3', // Vibrant purple
    secondary: '#48BFE3', // Bright blue
    accent: '#FF5D8F', // Hot pink
    text: '#2E186A', // Deep purple
    background: '#FFFFFF', // White
    muted: '#E9ECEF', // Pale gray
    description: 'Saturated, energetic colors for high-energy, playful designs'
  },
  
  // Seasonal/Mood Palettes
  'autumn': {
    primary: '#9D5C0D', // Burnt orange
    secondary: '#C7822F', // Pumpkin
    accent: '#D62828', // Crimson
    text: '#402E32', // Dark brown
    background: '#F9F4EF', // Cream
    muted: '#E0C2A2', // Beige
    description: 'Warm oranges, reds, and browns inspired by fall foliage'
  },
  'spring': {
    primary: '#6ECB94', // Fresh green
    secondary: '#ADE792', // Light green
    accent: '#FFDA3D', // Daffodil yellow
    text: '#345448', // Forest green
    background: '#F9FEF6', // Pale green-white
    muted: '#D6EBC8', // Soft green
    description: 'Fresh, rejuvenating greens with bright floral accents'
  },
  'summer': {
    primary: '#52B788', // Teal
    secondary: '#76C893', // Medium green
    accent: '#FF9F1C', // Sunshine yellow
    text: '#1B4332', // Dark green
    background: '#F0FFF1', // Mint white
    muted: '#B7E4C7', // Light mint
    description: 'Bright, vibrant colors inspired by lush summer landscapes'
  },
  'winter': {
    primary: '#3E78B2', // Winter blue
    secondary: '#748CAB', // Gray blue
    accent: '#A4AEBB', // Steel blue
    text: '#263952', // Dark navy
    background: '#F0F4F8', // Ice blue
    muted: '#D3E3F2', // Pale blue
    description: 'Cool blues and whites inspired by snow and ice'
  },
  'calm': {
    primary: '#799496', // Muted teal
    secondary: '#94B0B2', // Light teal
    accent: '#C2A878', // Soft gold
    text: '#2F3E46', // Dark blue-gray
    background: '#F7F9F9', // Off-white
    muted: '#D1DBDB', // Pale teal
    description: 'Subdued, balanced colors that create a peaceful atmosphere'
  },
  'energetic': {
    primary: '#FB5607', // Vibrant orange
    secondary: '#3A86FF', // Electric blue
    accent: '#8338EC', // Purple
    text: '#1F1F1F', // Nearly black
    background: '#FFFFFF', // White
    muted: '#F0F0F0', // Light gray
    description: 'Bright, high-contrast colors that convey movement and excitement'
  },
  
  // Cultural/Stylistic Palettes
  'nordic': {
    primary: '#4A5859', // Slate gray
    secondary: '#7A9E9F', // Muted teal
    accent: '#B8C4BB', // Sage
    text: '#292F33', // Charcoal
    background: '#F9FAFB', // Snow white
    muted: '#E5E9EC', // Pale gray
    description: 'Clean, restrained palette inspired by Scandinavian design'
  },
  'zen': {
    primary: '#2B580C', // Moss green
    secondary: '#639A67', // Medium green
    accent: '#D9AE94', // Clay pink
    text: '#1D3124', // Deep green
    background: '#F5F5F5', // White
    muted: '#CFDBC5', // Pale green
    description: 'Balanced, natural palette inspired by Japanese aesthetics'
  },
  'mediterranean': {
    primary: '#1A508B', // Deep blue
    secondary: '#0881A3', // Turquoise
    accent: '#FED049', // Sunshine yellow
    text: '#1D3461', // Navy blue
    background: '#FFF8F0', // Warm white
    muted: '#DAE7F1', // Pale blue
    description: 'Vivid blues with warm accents inspired by coastal Mediterranean regions'
  },
  'industrial': {
    primary: '#2C3333', // Charcoal
    secondary: '#475657', // Dark gray
    accent: '#F4B942', // Mustard yellow
    text: '#1A1A1A', // Nearly black
    background: '#F2F2F2', // Light gray
    muted: '#D1D1D1', // Medium gray
    description: 'Raw, utilitarian palette inspired by manufacturing and urban settings'
  },
  'artdeco': {
    primary: '#11235A', // Navy blue
    secondary: '#364F6B', // Steel blue
    accent: '#F5BF42', // Gold
    text: '#0D0D0D', // Black
    background: '#F8F9FD', // Off-white
    muted: '#C0C5CD', // Silver gray
    description: 'Rich, luxurious palette inspired by Art Deco aesthetics'
  }
};

/**
 * Typography Definitions
 * 
 * Maps natural language descriptions to font families, sizes, weights, and other
 * typography-related properties.
 */
const TYPOGRAPHY_STYLES = {
  'modern': {
    fontHeading: "'Montserrat', sans-serif",
    fontBody: "'Open Sans', sans-serif",
    fontCode: "'Fira Code', monospace",
    scale: 1.2, // Moderate scale
    lineHeight: 1.6,
    description: 'Clean, contemporary typography with geometric sans-serif fonts'
  },
  'classic': {
    fontHeading: "'Playfair Display', serif",
    fontBody: "'Source Serif Pro', serif",
    fontCode: "'IBM Plex Mono', monospace",
    scale: 1.33, // Perfect fourth
    lineHeight: 1.7,
    description: 'Elegant, traditional typography with serif fonts for a timeless look'
  },
  'minimal': {
    fontHeading: "'Inter', sans-serif",
    fontBody: "'Inter', sans-serif",
    fontCode: "'Roboto Mono', monospace",
    scale: 1.15, // Tight scale
    lineHeight: 1.5,
    description: 'Restrained typography with a single sans-serif family for consistency'
  },
  'playful': {
    fontHeading: "'Nunito', sans-serif",
    fontBody: "'Quicksand', sans-serif",
    fontCode: "'Space Mono', monospace",
    scale: 1.25, // Major third
    lineHeight: 1.6,
    description: 'Friendly, approachable fonts with rounded terminals and open forms'
  },
  'technical': {
    fontHeading: "'IBM Plex Sans', sans-serif",
    fontBody: "'IBM Plex Sans', sans-serif",
    fontCode: "'IBM Plex Mono', monospace",
    scale: 1.2, // Minor third
    lineHeight: 1.6,
    description: 'Rational, precise typography designed for clarity and readability'
  },
  'elegant': {
    fontHeading: "'Cormorant Garamond', serif",
    fontBody: "'Lora', serif",
    fontCode: "'Fira Code', monospace",
    scale: 1.5, // Perfect fifth
    lineHeight: 1.8,
    description: 'Sophisticated, high-contrast typography with refined serifs'
  },
  'creative': {
    fontHeading: "'Abril Fatface', display",
    fontBody: "'Work Sans', sans-serif",
    fontCode: "'JetBrains Mono', monospace",
    scale: 1.4, // Augmented fourth
    lineHeight: 1.7,
    description: 'Expressive headline fonts paired with neutral body text'
  },
  'corporate': {
    fontHeading: "'Noto Sans', sans-serif",
    fontBody: "'Noto Sans', sans-serif",
    fontCode: "'Inconsolata', monospace",
    scale: 1.18, // Minor third
    lineHeight: 1.5,
    description: 'Professional, neutral typography suitable for business contexts'
  },
  'futuristic': {
    fontHeading: "'Space Grotesk', sans-serif",
    fontBody: "'Syne', sans-serif",
    fontCode: "'JetBrains Mono', monospace",
    scale: 1.25, // Major third
    lineHeight: 1.5,
    description: 'Forward-looking typography with geometric and technological influences'
  },
  'handcrafted': {
    fontHeading: "'Caveat', cursive",
    fontBody: "'Noto Sans', sans-serif",
    fontCode: "'Fira Code', monospace",
    scale: 1.33, // Perfect fourth
    lineHeight: 1.7,
    description: 'Personal, human touch with script headings and neutral body text'
  }
};

/**
 * Layout Definitions
 * 
 * Maps natural language descriptions to layout properties like spacing,
 * borders, shadows, and components styling.
 */
const LAYOUT_STYLES = {
  'spacious': {
    spacing: 1.2, // Multiplier for base spacing
    contentWidth: '920px',
    borderRadius: '8px',
    shadowIntensity: 0.7,
    description: 'Open layouts with generous whitespace and breathing room'
  },
  'compact': {
    spacing: 0.8,
    contentWidth: '800px',
    borderRadius: '4px',
    shadowIntensity: 0.5,
    description: 'Efficient layouts with tighter spacing for information density'
  },
  'fluid': {
    spacing: 1.0,
    contentWidth: '90%',
    maxContentWidth: '1200px',
    borderRadius: '12px',
    shadowIntensity: 0.6,
    description: 'Adaptable layouts that gracefully expand across different screen sizes'
  },
  'structured': {
    spacing: 1.0,
    contentWidth: '860px',
    borderRadius: '6px',
    shadowIntensity: 0.8,
    description: 'Organized layouts with clear visual hierarchy and divisions'
  },
  'soft': {
    spacing: 1.1,
    contentWidth: '880px',
    borderRadius: '16px',
    shadowIntensity: 0.4,
    description: 'Gentle layouts with rounded corners and subtle shadows'
  },
  'sharp': {
    spacing: 0.9,
    contentWidth: '840px',
    borderRadius: '2px',
    shadowIntensity: 0.9,
    description: 'Defined layouts with angular corners and distinct edges'
  },
  'asymmetric': {
    spacing: 1.0,
    contentWidth: '75%',
    maxContentWidth: '1100px',
    borderRadius: '8px 0 8px 0', // Alternating corners
    shadowIntensity: 0.7,
    isAsymmetric: true,
    description: 'Dynamic layouts with intentional imbalance for visual interest'
  },
  'balanced': {
    spacing: 1.0,
    contentWidth: '850px',
    borderRadius: '8px',
    shadowIntensity: 0.6,
    description: 'Harmonious layouts with careful attention to proportion and balance'
  }
};

/**
 * Form Factor Definitions
 * 
 * Specifies how components and UI elements should appear based on natural
 * language descriptions of their form.
 */
const FORM_FACTORS = {
  'rounded': {
    buttonRadius: '50px', // Pill-shaped buttons
    cardRadius: '16px',
    inputRadius: '25px',
    shadows: 'soft',
    description: 'Friendly, approachable UI with generously rounded corners'
  },
  'geometric': {
    buttonRadius: '4px',
    cardRadius: '8px',
    inputRadius: '4px',
    shadows: 'defined',
    description: 'Clean, structured UI with subtle rounding and clear boundaries'
  },
  'sharp': {
    buttonRadius: '0px',
    cardRadius: '0px',
    inputRadius: '0px',
    shadows: 'hard',
    description: 'Angular, precise UI with straight edges and defined corners'
  },
  'organic': {
    buttonRadius: 'var(--organic-radius, 12px 8px 16px 4px)',
    cardRadius: 'var(--organic-radius, 16px 8px 20px 12px)',
    inputRadius: 'var(--organic-radius, 8px 12px 4px 16px)',
    shadows: 'natural',
    isOrganic: true,
    description: 'Natural, varied UI with irregular shapes inspired by nature'
  },
  'floating': {
    buttonRadius: '12px',
    cardRadius: '16px',
    inputRadius: '12px',
    shadows: 'lifted',
    elevation: 'high',
    description: 'Elevated UI with strong shadows creating a sense of layers'
  },
  'flat': {
    buttonRadius: '4px',
    cardRadius: '8px',
    inputRadius: '4px',
    shadows: 'none',
    borders: 'subtle',
    description: 'Minimal UI with little to no elevation and subtle differentiation'
  },
  'glossy': {
    buttonRadius: '8px',
    cardRadius: '12px',
    inputRadius: '8px',
    shadows: 'soft',
    effects: 'highlight',
    description: 'Reflective UI with subtle gradients and polish effects'
  },
  'glassmorphic': {
    buttonRadius: '12px',
    cardRadius: '16px',
    inputRadius: '12px',
    shadows: 'soft',
    effects: 'blur',
    description: 'Translucent, frosted glass UI with background blur effects'
  },
  'neumorphic': {
    buttonRadius: '50px',
    cardRadius: '24px',
    inputRadius: '50px',
    shadows: 'neumorphic',
    description: 'Soft UI with emboss/deboss effects giving subtle 3D appearance'
  }
};

/**
 * Core parser for design language
 * 
 * @param {string} designDescription - Natural language description of desired design
 * @returns {Object} Design properties as usable CSS variables
 */
function parseDesignLanguage(designDescription) {
  const design = {
    palette: {},
    typography: {},
    layout: {},
    formFactor: {},
    customProperties: {}
  };
  
  // Convert to lowercase for easier matching
  const description = designDescription.toLowerCase();
  
  // Process color palette
  design.palette = extractPalette(description);
  
  // Process typography
  design.typography = extractTypography(description);
  
  // Process layout
  design.layout = extractLayout(description);
  
  // Process form factors
  design.formFactor = extractFormFactor(description);
  
  // Extract any additional custom properties
  design.customProperties = extractCustomProperties(description);
  
  // Generate final CSS variables
  return generateCSSVariables(design);
}

/**
 * Extract color palette information from design description
 * @param {string} description - Lowercase design description
 * @returns {Object} Color palette properties
 */
function extractPalette(description) {
  let palette = { ...COLOR_PALETTES['minimalist'] }; // Default
  let bestMatch = '';
  let matchScore = 0;
  
  // Check for exact palette names
  for (const [name, values] of Object.entries(COLOR_PALETTES)) {
    if (description.includes(name)) {
      // If exact match found, prioritize it
      if (description.includes(`${name} palette`) || 
          description.includes(`${name} colors`) ||
          description.includes(`${name} theme`)) {
        return values;
      }
      
      // Calculate match relevance
      const nameScore = name.length;
      if (nameScore > matchScore) {
        matchScore = nameScore;
        bestMatch = name;
        palette = { ...values };
      }
    }
  }
  
  // Check for combined palettes (e.g., "earthy with vibrant accents")
  const paletteRegex = /([a-z]+)\s+with\s+([a-z]+)\s+(accents|highlights|notes)/i;
  const paletteMix = description.match(paletteRegex);
  
  if (paletteMix) {
    const basePalette = paletteMix[1].toLowerCase();
    const accentPalette = paletteMix[2].toLowerCase();
    
    if (COLOR_PALETTES[basePalette] && COLOR_PALETTES[accentPalette]) {
      palette = { ...COLOR_PALETTES[basePalette] };
      // Just take the accent color from the second palette
      palette.accent = COLOR_PALETTES[accentPalette].accent;
    }
  }
  
  // Return best match if found, otherwise default
  return bestMatch ? palette : COLOR_PALETTES['minimalist'];
}

/**
 * Extract typography information from design description
 * @param {string} description - Lowercase design description
 * @returns {Object} Typography properties
 */
function extractTypography(description) {
  let typography = { ...TYPOGRAPHY_STYLES['modern'] }; // Default
  let bestMatch = '';
  let matchScore = 0;
  
  // Check for exact typography style names
  for (const [name, values] of Object.entries(TYPOGRAPHY_STYLES)) {
    if (description.includes(name)) {
      // If exact match found, prioritize it
      if (description.includes(`${name} typography`) || 
          description.includes(`${name} fonts`) ||
          description.includes(`${name} type`)) {
        return values;
      }
      
      // Calculate match relevance
      const nameScore = name.length;
      if (nameScore > matchScore) {
        matchScore = nameScore;
        bestMatch = name;
        typography = { ...values };
      }
    }
  }
  
  // Specific typography instructions override
  if (description.includes('serif') && description.includes('headings')) {
    typography.fontHeading = "'Playfair Display', serif";
  }
  
  if (description.includes('sans-serif') && description.includes('body')) {
    typography.fontBody = "'Open Sans', sans-serif";
  }
  
  if (description.includes('monospace') || description.includes('code')) {
    typography.fontCode = "'Fira Code', monospace";
  }
  
  // Detect larger or smaller type
  if (description.includes('large type') || description.includes('bigger type')) {
    typography.scale *= 1.1;
  } else if (description.includes('small type') || description.includes('compact type')) {
    typography.scale *= 0.9;
  }
  
  // Return best match if found, otherwise default
  return bestMatch ? typography : TYPOGRAPHY_STYLES['modern'];
}

/**
 * Extract layout information from design description
 * @param {string} description - Lowercase design description
 * @returns {Object} Layout properties
 */
function extractLayout(description) {
  let layout = { ...LAYOUT_STYLES['balanced'] }; // Default
  let bestMatch = '';
  let matchScore = 0;
  
  // Check for exact layout style names
  for (const [name, values] of Object.entries(LAYOUT_STYLES)) {
    if (description.includes(name)) {
      // If exact match found, prioritize it
      if (description.includes(`${name} layout`) || 
          description.includes(`${name} spacing`) ||
          description.includes(`${name} design`)) {
        return values;
      }
      
      // Calculate match relevance
      const nameScore = name.length;
      if (nameScore > matchScore) {
        matchScore = nameScore;
        bestMatch = name;
        layout = { ...values };
      }
    }
  }
  
  // Adjust spacing based on descriptors
  if (description.includes('more space') || description.includes('spacious')) {
    layout.spacing *= 1.2;
  } else if (description.includes('less space') || description.includes('tight') || description.includes('compact')) {
    layout.spacing *= 0.8;
  }
  
  // Adjust content width
  if (description.includes('narrow') || description.includes('slim')) {
    layout.contentWidth = '750px';
  } else if (description.includes('wide') || description.includes('broad')) {
    layout.contentWidth = '1000px';
  }
  
  // Return best match if found, otherwise default
  return bestMatch ? layout : LAYOUT_STYLES['balanced'];
}

/**
 * Extract form factor information from design description
 * @param {string} description - Lowercase design description
 * @returns {Object} Form factor properties
 */
function extractFormFactor(description) {
  let formFactor = { ...FORM_FACTORS['geometric'] }; // Default
  let bestMatch = '';
  let matchScore = 0;
  
  // Check for exact form factor names
  for (const [name, values] of Object.entries(FORM_FACTORS)) {
    if (description.includes(name)) {
      // If exact match found, prioritize it
      if (description.includes(`${name} buttons`) || 
          description.includes(`${name} corners`) ||
          description.includes(`${name} elements`) ||
          description.includes(`${name} interface`)) {
        return values;
      }
      
      // Calculate match relevance
      const nameScore = name.length;
      if (nameScore > matchScore) {
        matchScore = nameScore;
        bestMatch = name;
        formFactor = { ...values };
      }
    }
  }
  
  // Return best match if found, otherwise default
  return bestMatch ? formFactor : FORM_FACTORS['geometric'];
}

/**
 * Extract any custom properties from specific requests in the description
 * @param {string} description - Lowercase design description
 * @returns {Object} Custom properties
 */
function extractCustomProperties(description) {
  const custom = {};
  
  // Check for specific color requests
  const colorRequests = description.match(/([a-z\s]+) should be ([a-z]+)/g);
  if (colorRequests) {
    colorRequests.forEach(request => {
      const parts = request.split(' should be ');
      if (parts.length === 2) {
        const element = parts[0].trim();
        const color = parts[1].trim();
        
        // Map common elements to CSS properties
        if (element.includes('heading') || element.includes('title')) {
          custom['--heading-color'] = translateColorTerm(color);
        } else if (element.includes('background')) {
          custom['--background-color'] = translateColorTerm(color);
        } else if (element.includes('button')) {
          custom['--button-color'] = translateColorTerm(color);
        } else if (element.includes('link')) {
          custom['--link-color'] = translateColorTerm(color);
        }
      }
    });
  }
  
  // Check for animation preferences
  if (description.includes('animate') || description.includes('animation')) {
    custom['--use-animations'] = 'true';
    
    if (description.includes('subtle animation')) {
      custom['--animation-scale'] = '0.7';
    } else if (description.includes('prominent animation') || description.includes('strong animation')) {
      custom['--animation-scale'] = '1.3';
    }
  }
  
  // Check for specific spacing instructions
  if (description.includes('tight spacing')) {
    custom['--spacing-multiplier'] = '0.8';
  } else if (description.includes('loose spacing')) {
    custom['--spacing-multiplier'] = '1.2';
  }
  
  return custom;
}

/**
 * Translates color terms into actual color values
 * @param {string} colorTerm - Color term like "blue", "warm", etc.
 * @returns {string} CSS color value
 */
function translateColorTerm(colorTerm) {
  // Basic color mapping
  const colorMap = {
    blue: '#0077B6',
    red: '#E63946',
    green: '#2A9D8F',
    yellow: '#FCBF49',
    purple: '#7209B7',
    pink: '#FF5D8F',
    orange: '#FB8500',
    brown: '#6B4226',
    black: '#000000',
    white: '#FFFFFF',
    gray: '#6C757D',
    teal: '#2EC4B6',
    navy: '#1A3A5F',
    gold: '#FFD700',
    silver: '#C0C0C0',
    warm: '#FD8A5E',
    cool: '#86BBD8',
    earthy: '#7D6953',
    bright: '#FF5757',
    muted: '#959EAD',
    dark: '#212529',
    light: '#F8F9FA'
  };
  
  return colorMap[colorTerm] || '#7D7D7D'; // Default gray if not found
}

/**
 * Generate CSS variables from the combined design properties
 * @param {Object} design - Complete design object
 * @returns {Object} CSS variables ready to use
 */
function generateCSSVariables(design) {
  const { palette, typography, layout, formFactor, customProperties } = design;
  
  // Base CSS variables
  const cssVars = {
    // Color variables
    '--primary-color': palette.primary,
    '--primary-color-rgb': hexToRgb(palette.primary),
    '--secondary-color': palette.secondary,
    '--secondary-color-rgb': hexToRgb(palette.secondary),
    '--accent-color': palette.accent,
    '--accent-color-rgb': hexToRgb(palette.accent),
    '--text-color': palette.text,
    '--text-light': palette.muted,
    '--background-color': palette.background,
    '--background-secondary': lightenDarkenColor(palette.background, -5),
    '--border-color': lightenDarkenColor(palette.background, -10),
    '--shadow-color': `rgba(${hexToRgb(palette.text)}, 0.1)`,
    
    // Typography variables
    '--font-heading': typography.fontHeading,
    '--font-body': typography.fontBody,
    '--font-code': typography.fontCode,
    '--font-size-base': '16px',
    '--line-height-base': typography.lineHeight,
    '--scale-ratio': typography.scale,
    
    // Layout variables
    '--spacing-scale': layout.spacing,
    '--content-width': layout.contentWidth,
    '--border-radius-sm': `${4 * (formFactor.buttonRadius === '0px' ? 0 : 1)}px`,
    '--border-radius-md': `${8 * (formFactor.cardRadius === '0px' ? 0 : 1)}px`,
    '--border-radius-lg': `${16 * (formFactor.cardRadius === '0px' ? 0 : 1)}px`,
    
    // Form factor variables
    '--button-radius': formFactor.buttonRadius,
    '--card-radius': formFactor.cardRadius,
    '--input-radius': formFactor.inputRadius,
    '--shadow-intensity': formFactor.shadows === 'none' ? 0 : (layout.shadowIntensity || 0.6)
  };
  
  // Add any custom properties
  Object.assign(cssVars, customProperties);
  
  return cssVars;
}

/**
 * Convert hex color to RGB
 * @param {string} hex - Hex color code
 * @returns {string} RGB values as "r, g, b" string
 */
function hexToRgb(hex) {
  // Remove # if present
  hex = hex.replace('#', '');
  
  // Parse hex values
  const r = parseInt(hex.substring(0, 2), 16);
  const g = parseInt(hex.substring(2, 4), 16);
  const b = parseInt(hex.substring(4, 6), 16);
  
  return `${r}, ${g}, ${b}`;
}

/**
 * Lighten or darken a color
 * @param {string} color - Hex color code
 * @param {number} percent - Percentage to lighten (positive) or darken (negative)
 * @returns {string} Modified hex color
 */
function lightenDarkenColor(color, percent) {
  let R = parseInt(color.substring(1,3), 16);
  let G = parseInt(color.substring(3,5), 16);
  let B = parseInt(color.substring(5,7), 16);

  R = parseInt(R * (100 + percent) / 100);
  G = parseInt(G * (100 + percent) / 100);
  B = parseInt(B * (100 + percent) / 100);

  R = (R < 255) ? R : 255;  
  G = (G < 255) ? G : 255;  
  B = (B < 255) ? B : 255;  

  R = Math.max(0, R);
  G = Math.max(0, G);
  B = Math.max(0, B);

  const RR = ((R.toString(16).length === 1) ? "0" + R.toString(16) : R.toString(16));
  const GG = ((G.toString(16).length === 1) ? "0" + G.toString(16) : G.toString(16));
  const BB = ((B.toString(16).length === 1) ? "0" + B.toString(16) : B.toString(16));

  return "#" + RR + GG + BB;
}

/**
 * Generate CSS string from variables object
 * @param {Object} variables - CSS variables object
 * @returns {string} CSS as string ready to insert into stylesheets
 */
function generateCSSString(variables) {
  let css = ':root {\n';
  
  for (const [key, value] of Object.entries(variables)) {
    css += `  ${key}: ${value};\n`;
  }
  
  css += '}\n';
  return css;
}

module.exports = {
  parseDesignLanguage,
  generateCSSString,
  COLOR_PALETTES,
  TYPOGRAPHY_STYLES,
  LAYOUT_STYLES,
  FORM_FACTORS
};