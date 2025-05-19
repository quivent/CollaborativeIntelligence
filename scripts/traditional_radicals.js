/**
 * Traditional Chinese Radical System for categorizing kanji
 * 
 * This module implements the Traditional Chinese Radical System (214 radicals)
 * condensed into major radical families with a balanced distribution.
 * 
 * The classification is based on the traditional system used in dictionaries,
 * with radicals grouped into semantic families to create a balanced categorization.
 */

// Map kanji to their primary radicals (simplified mapping for demonstration)
// A complete implementation would use a full database of kanji-to-radical mappings
const KANJI_RADICAL_MAP = {
  // This is a simplified mapping for demonstration purposes
  // In a production system, this would be a comprehensive database
};

// Define the radical families (condensed from 214 traditional radicals)
const RADICAL_FAMILIES = {
  // Group 1: Person-related radicals (人, 亻, 女, 子, etc.)
  PERSON: {
    name: "Person Radicals",
    description: "Kanji containing radicals related to people and human features",
    radicals: [1, 9, 10, 11, 28, 29, 30, 33, 38, 57, 60, 61, 62, 63, 64, 65, 67, 68, 73, 80, 85, 88, 94, 97, 159, 181, 185, 186, 187, 188, 189, 192],
    strokeRange: [1, 11]
  },
  
  // Group 2: Speech and language radicals (言, 口, 舌, etc.)
  SPEECH: {
    name: "Speech and Language Radicals",
    description: "Kanji containing radicals related to speaking, language, and communication",
    radicals: [7, 30, 73, 89, 135, 149, 183],
    strokeRange: [2, 7]
  },
  
  // Group 3: Body parts radicals (目, 耳, 手, 足, etc.)
  BODY: {
    name: "Body Part Radicals",
    description: "Kanji containing radicals representing parts of the body",
    radicals: [10, 55, 56, 57, 61, 62, 63, 64, 66, 76, 105, 109, 128, 130, 131, 132, 157, 181, 185, 187, 189, 194, 207, 209],
    strokeRange: [3, 16]
  },
  
  // Group 4: Nature and elements radicals (山, 川, 水, 火, etc.)
  NATURE: {
    name: "Nature and Elements Radicals",
    description: "Kanji containing radicals related to natural elements and phenomena",
    radicals: [15, 17, 46, 47, 49, 51, 53, 75, 81, 85, 86, 112, 113, 114, 119, 120, 122, 142, 143, 146, 173, 195, 196, 197, 210, 211],
    strokeRange: [2, 9]
  },
  
  // Group 5: Plants radicals (木, 竹, 草, 米, etc.)
  PLANT: {
    name: "Plant Radicals",
    description: "Kanji containing radicals related to plants, trees, and vegetation",
    radicals: [5, 25, 45, 75, 108, 115, 118, 119, 140, 144, 145, 151, 154, 159, 160, 162, 179, 188, 190, 212],
    strokeRange: [2, 11]
  },
  
  // Group 6: Animal radicals (犬, 馬, 鳥, 魚, etc.)
  ANIMAL: {
    name: "Animal Radicals",
    description: "Kanji containing radicals related to animals and creatures",
    radicals: [94, 123, 124, 129, 141, 142, 152, 153, 156, 172, 177, 189, 195, 197, 198, 201, 205, 206, 207, 208, 212, 213],
    strokeRange: [3, 17]
  },
  
  // Group 7: Building and enclosure radicals (門, 宀, 广, 囗, etc.)
  ENCLOSURE: {
    name: "Enclosure and Building Radicals",
    description: "Kanji containing radicals related to buildings, structures, and enclosures",
    radicals: [13, 16, 17, 22, 23, 24, 27, 31, 32, 40, 42, 44, 53, 63, 69, 70, 91, 102, 163, 169, 170],
    strokeRange: [2, 8]
  },
  
  // Group 8: Tools and weapons radicals (刀, 力, 弓, 戈, etc.)
  TOOL: {
    name: "Tool and Weapon Radicals",
    description: "Kanji containing radicals related to tools, weapons, and implements",
    radicals: [12, 18, 19, 20, 26, 32, 34, 35, 48, 55, 56, 57, 79, 87, 91, 95, 96, 98, 101, 103, 104, 113, 118, 128, 137, 144, 154, 167, 187, 210],
    strokeRange: [1, 9]
  },
  
  // Group 9: Movement radicals (走, 辵, 足, etc.)
  MOVEMENT: {
    name: "Movement Radicals",
    description: "Kanji containing radicals related to movement and direction",
    radicals: [2, 34, 35, 54, 58, 59, 60, 61, 69, 77, 77, 121, 138, 156, 157, 162, 178, 182, 187, 191, 194, 207],
    strokeRange: [2, 11]
  },
  
  // Group 10: Numerals and measurements radicals (一, 二, 寸, etc.)
  NUMBER: {
    name: "Number and Measurement Radicals",
    description: "Kanji containing radicals related to numbers, counting, and measurement",
    radicals: [1, 2, 3, 4, 5, 6, 41, 48, 66, 68, 71, 77, 79, 92, 97, 101, 105, 165, 166, 182, 199, 200],
    strokeRange: [1, 8]
  },
  
  // Group 11: Abstract concept radicals (心, 忄, 礻, etc.)
  ABSTRACT: {
    name: "Abstract Concept Radicals",
    description: "Kanji containing radicals related to abstract concepts and ideas",
    radicals: [8, 39, 50, 54, 60, 61, 65, 72, 80, 92, 103, 104, 108, 113, 114, 145, 164, 189, 191, 192, 209],
    strokeRange: [2, 13]
  },
  
  // Group 12: Materials and objects radicals (玉, 金, 缶, etc.)
  MATERIAL: {
    name: "Material and Object Radicals",
    description: "Kanji containing radicals related to materials and physical objects",
    radicals: [14, 15, 96, 98, 108, 112, 119, 120, 121, 125, 126, 137, 143, 154, 159, 163, 165, 166, 167, 168, 169, 170, 191, 211],
    strokeRange: [3, 14]
  }
};

// Kanji radical data (mapping radicals by number to their form)
const RADICAL_DATA = {
  1: { form: "一", meaning: "one" },
  2: { form: "丨", meaning: "line" },
  3: { form: "丶", meaning: "dot" },
  4: { form: "丿", meaning: "slash" },
  5: { form: "乙", meaning: "second" },
  6: { form: "亅", meaning: "hook" },
  7: { form: "二", meaning: "two" },
  8: { form: "亠", meaning: "lid" },
  9: { form: "人", meaning: "person" },
  10: { form: "儿", meaning: "legs" },
  // ... and so on for all 214 radicals
};

/**
 * Lookup table for stroke-based radical classification
 * For fast classification when radical information isn't available
 */
const STROKE_TO_RADICAL_FAMILY = {
  1: ["PERSON", "NUMBER"],  // Single-stroke kanji often relate to people or numbers
  2: ["NUMBER", "SPEECH"],
  3: ["PERSON", "NUMBER", "TOOL"],
  4: ["BODY", "SPEECH", "TOOL"],
  5: ["NATURE", "ENCLOSURE", "TOOL"],
  6: ["PERSON", "NATURE", "ENCLOSURE"],
  7: ["MOVEMENT", "ABSTRACT", "MATERIAL"],
  8: ["PERSON", "BODY", "PLANT"],
  9: ["TOOL", "MATERIAL", "ABSTRACT"],
  10: ["NATURE", "BODY", "ANIMAL"],
  11: ["PERSON", "MOVEMENT", "PLANT"],
  12: ["ENCLOSURE", "MATERIAL", "ANIMAL"],
  13: ["ABSTRACT", "MATERIAL", "ANIMAL"],
  14: ["NATURE", "MATERIAL", "ANIMAL"],
  15: ["PLANT", "ANIMAL", "MATERIAL"],
  16: ["BODY", "ANIMAL", "ABSTRACT"],
  17: ["ANIMAL", "ABSTRACT", "MATERIAL"],
  18: ["ANIMAL", "ABSTRACT"],
  19: ["ANIMAL"],
  20: ["ANIMAL"],
  21: ["ANIMAL"],
  22: ["ANIMAL"],
  23: ["ANIMAL"]
  // Higher stroke counts typically correlate with animal or complex material kanji
};

/**
 * Determine radical family based on kanji properties
 * 
 * This function attempts multiple strategies:
 * 1. First tries to find a direct kanji-to-radical mapping
 * 2. Falls back to stroke count categorization
 * 3. Uses Unicode range as a last resort
 * 
 * @param {string} kanji - The kanji character to classify
 * @param {Object} data - Optional kanji data object
 * @returns {string} The radical family key
 */
function getRadicalFamily(kanji, data = null) {
  // Strategy 1: Direct kanji-to-radical mapping
  if (KANJI_RADICAL_MAP[kanji]) {
    const radical = KANJI_RADICAL_MAP[kanji];
    
    // Check which family this radical belongs to
    for (const [family, info] of Object.entries(RADICAL_FAMILIES)) {
      if (info.radicals.includes(radical)) {
        return family;
      }
    }
  }
  
  // Strategy 2: Use stroke count if available
  if (data && data.stroke_count) {
    const strokeCount = data.stroke_count;
    const possibleFamilies = STROKE_TO_RADICAL_FAMILY[strokeCount] || [];
    
    if (possibleFamilies.length > 0) {
      // Use some heuristics based on meaning if available
      if (data.meanings) {
        const meanings = Array.isArray(data.meanings) ? data.meanings.join(' ') : data.meanings;
        
        // Simple keyword matching for classification
        const keywords = {
          PERSON: ["person", "people", "man", "woman", "child", "human"],
          SPEECH: ["speak", "say", "word", "talk", "language"],
          BODY: ["body", "eye", "ear", "hand", "foot", "mouth", "nose"],
          NATURE: ["water", "fire", "earth", "air", "nature", "sun", "moon"],
          PLANT: ["plant", "tree", "flower", "grass", "wood"],
          ANIMAL: ["animal", "bird", "fish", "insect", "beast"],
          ENCLOSURE: ["house", "door", "gate", "building", "room"],
          TOOL: ["tool", "weapon", "knife", "sword", "arrow"],
          MOVEMENT: ["move", "walk", "run", "go", "come"],
          NUMBER: ["one", "two", "three", "number", "count", "measure"],
          ABSTRACT: ["heart", "mind", "soul", "spirit", "idea"],
          MATERIAL: ["gold", "metal", "stone", "jade", "silk", "cloth"]
        };
        
        for (const family of possibleFamilies) {
          const familyKeywords = keywords[family] || [];
          for (const keyword of familyKeywords) {
            if (meanings.toLowerCase().includes(keyword)) {
              return family;
            }
          }
        }
      }
      
      // If no keyword match, return first possible family
      return possibleFamilies[0];
    }
  }
  
  // Strategy 3: Use Unicode code point ranges
  const code = kanji.charCodeAt(0);
  
  // These ranges are approximate and based on unicode block organization
  if (code >= 0x4E00 && code <= 0x56FF) return "PERSON";
  if (code >= 0x5700 && code <= 0x5FFF) return "NATURE";
  if (code >= 0x6000 && code <= 0x68FF) return "ENCLOSURE";
  if (code >= 0x6900 && code <= 0x71FF) return "TOOL";
  if (code >= 0x7200 && code <= 0x7AFF) return "ABSTRACT";
  if (code >= 0x7B00 && code <= 0x7FFF) return "SPEECH";
  if (code >= 0x8000 && code <= 0x86FF) return "BODY";
  if (code >= 0x8700 && code <= 0x8DFF) return "PLANT";
  if (code >= 0x8E00 && code <= 0x93FF) return "MOVEMENT";
  if (code >= 0x9400 && code <= 0x9BFF) return "ANIMAL";
  if (code >= 0x9C00 && code <= 0x9FFF) return "MATERIAL";
  
  // Default to NUMBER for any remaining characters
  return "NUMBER";
}

/**
 * Get all available radical family keys
 * 
 * @returns {Array<string>} Array of family keys
 */
function getAllRadicalFamilies() {
  return Object.keys(RADICAL_FAMILIES);
}

/**
 * Gets the name and description for a radical family
 * 
 * @param {string} familyKey - The family key (e.g., "PERSON", "NATURE")
 * @returns {Object} Object with name and description properties
 */
function getRadicalFamilyInfo(familyKey) {
  if (RADICAL_FAMILIES[familyKey]) {
    return {
      name: RADICAL_FAMILIES[familyKey].name,
      description: RADICAL_FAMILIES[familyKey].description
    };
  }
  return {
    name: "Unknown Family",
    description: "Unknown radical family"
  };
}

module.exports = {
  RADICAL_FAMILIES,
  RADICAL_DATA,
  getRadicalFamily,
  getAllRadicalFamilies,
  getRadicalFamilyInfo
};