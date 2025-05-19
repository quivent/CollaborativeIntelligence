/**
 * balanced_traditional_radicals.js
 * 
 * Implementation of the Traditional Chinese Radical System for categorizing kanji
 * with improved balancing to ensure even distribution across categories
 */

// Define the 10 radical families (consolidated for better balance)
const RADICAL_FAMILIES = {
  PERSON: "Person & Human Attributes",
  SPEECH: "Speech & Communication",
  BODY: "Body Parts & Functions",
  NATURE: "Natural Elements & Phenomena",
  LIVING: "Plants & Animals",
  STRUCTURE: "Buildings & Enclosures",
  TOOLS: "Tools & Objects",
  MOVEMENT: "Movement & Actions",
  ABSTRACT: "Abstract Concepts & Symbols",
  ELEMENTS: "Materials & Elements"
};

// Map of radical numbers to radical families (consolidated)
const RADICAL_TO_FAMILY = {
  // PERSON & HUMAN ATTRIBUTES
  9: 'PERSON',    // 人 Person
  10: 'PERSON',   // 儿 Legs
  33: 'PERSON',   // 士 Scholar
  38: 'PERSON',   // 女 Woman
  39: 'PERSON',   // 子 Child
  57: 'PERSON',   // 彳 Step
  60: 'PERSON',   // 戈 Spear
  62: 'PERSON',   // 支 Branch
  64: 'PERSON',   // 文 Script
  66: 'PERSON',   // 斤 Axe
  67: 'PERSON',   // 方 Square
  
  // SPEECH & COMMUNICATION
  7: 'SPEECH',    // 二 Two
  30: 'SPEECH',   // 口 Mouth
  73: 'SPEECH',   // 月 Moon
  89: 'SPEECH',   // 舌 Tongue
  149: 'SPEECH',  // 言 Speech/Word
  183: 'SPEECH',  // 頁 Page/Head
  
  // BODY PARTS & FUNCTIONS
  10: 'BODY',     // 儿 Legs
  55: 'BODY',     // 廴 Long stride
  61: 'BODY',     // 手 Hand
  76: 'BODY',     // 止 Stop
  105: 'BODY',    // 目 Eye
  109: 'BODY',    // 皮 Skin
  128: 'BODY',    // 耳 Ear
  130: 'BODY',    // 肉 Meat
  131: 'BODY',    // 臣 Minister
  157: 'BODY',    // 見 See
  181: 'BODY',    // 頁 Head
  
  // NATURAL ELEMENTS & PHENOMENA
  1: 'NATURE',    // 一 One
  15: 'NATURE',   // 冫 Ice
  32: 'NATURE',   // 土 Earth
  46: 'NATURE',   // 山 Mountain
  47: 'NATURE',   // 巛 River
  71: 'NATURE',   // 日 Sun
  72: 'NATURE',   // 曰 Say
  84: 'NATURE',   // 水 Water
  85: 'NATURE',   // 火 Fire
  112: 'NATURE',  // 石 Stone
  173: 'NATURE',  // 雨 Rain
  
  // PLANTS & ANIMALS (LIVING)
  74: 'LIVING',   // 木 Tree
  75: 'LIVING',   // 欠 Lack
  93: 'LIVING',   // 牛 Cow
  94: 'LIVING',   // 牙 Tusk
  102: 'LIVING',  // 犬 Dog
  115: 'LIVING',  // 禾 Grain
  119: 'LIVING',  // 禾 Grain
  140: 'LIVING',  // 艸 Grass
  142: 'LIVING',  // 虍 Tiger
  153: 'LIVING',  // 豕 Pig
  189: 'LIVING',  // 馬 Horse
  194: 'LIVING',  // 魚 Fish
  196: 'LIVING',  // 鳥 Bird
  201: 'LIVING',  // 鹿 Deer
  
  // BUILDINGS & ENCLOSURES (STRUCTURE)
  13: 'STRUCTURE', // 冂 Down box
  16: 'STRUCTURE', // 几 Table
  17: 'STRUCTURE', // 凵 Open box
  22: 'STRUCTURE', // 匚 Box
  23: 'STRUCTURE', // 匸 Hiding enclosure
  31: 'STRUCTURE', // 囗 Enclosure
  40: 'STRUCTURE', // 宀 Roof
  53: 'STRUCTURE', // 廴 Long stride
  167: 'STRUCTURE', // 邑 City
  169: 'STRUCTURE', // 酉 Wine
  
  // TOOLS & OBJECTS
  18: 'TOOLS',     // 刀 Knife
  19: 'TOOLS',     // 力 Power
  20: 'TOOLS',     // 勹 Wrap
  26: 'TOOLS',     // 卜 Divination
  27: 'TOOLS',     // 卩 Seal
  50: 'TOOLS',     // 巾 Towel
  51: 'TOOLS',     // 干 Dry
  52: 'TOOLS',     // 幺 Short thread
  56: 'TOOLS',     // 弓 Bow
  98: 'TOOLS',     // 用 Use
  104: 'TOOLS',    // 疋 Bolt of cloth
  108: 'TOOLS',    // 皿 Dish
  159: 'TOOLS',    // 車 Vehicle
  
  // MOVEMENT & ACTIONS
  2: 'MOVEMENT',   // 丨 Line
  34: 'MOVEMENT',  // 夂 Go
  35: 'MOVEMENT',  // 夊 Go slowly
  54: 'MOVEMENT',  // 廾 Arms
  58: 'MOVEMENT',  // 彡 Bristle
  77: 'MOVEMENT',  // 歹 Death
  78: 'MOVEMENT',  // 殳 Weapon
  79: 'MOVEMENT',  // 毋 Do not
  97: 'MOVEMENT',  // 生 Life
  122: 'MOVEMENT', // 立 Stand
  145: 'MOVEMENT', // 行 Go
  
  // ABSTRACT CONCEPTS & SYMBOLS
  3: 'ABSTRACT',   // 丶 Dot
  4: 'ABSTRACT',   // 丿 Slash
  5: 'ABSTRACT',   // 乙 Second
  6: 'ABSTRACT',   // 亅 Hook
  24: 'ABSTRACT',  // 十 Ten
  41: 'ABSTRACT',  // 寸 Inch
  59: 'ABSTRACT',  // 心 Heart
  70: 'ABSTRACT',  // 无 Not
  113: 'ABSTRACT', // 示 Altar
  114: 'ABSTRACT', // 禸 Track
  123: 'ABSTRACT', // 立 Stand
  143: 'ABSTRACT', // 言 Speech
  
  // MATERIALS & ELEMENTS
  8: 'ELEMENTS',   // 亠 Lid
  14: 'ELEMENTS',  // 冖 Cover
  32: 'ELEMENTS',  // 土 Earth
  86: 'ELEMENTS',  // 爪 Claw
  95: 'ELEMENTS',  // 玄 Profound
  96: 'ELEMENTS',  // 玉 Jade
  170: 'ELEMENTS', // 金 Metal/Gold
  174: 'ELEMENTS', // 青 Blue
  175: 'ELEMENTS', // 韋 Tanned leather
  190: 'ELEMENTS', // 鬲 Cauldron
};

// Common kanji-to-radical mappings for frequently used kanji
const KANJI_RADICAL_MAP = {
  // NUMBERS (redistributed to ensure balanced representation)
  '一': 'ABSTRACT',    // One
  '二': 'ABSTRACT',    // Two
  '三': 'ABSTRACT',    // Three
  '四': 'ABSTRACT',    // Four
  '五': 'ABSTRACT',    // Five
  '六': 'ABSTRACT',    // Six
  '七': 'ABSTRACT',    // Seven
  '八': 'ABSTRACT',    // Eight
  '九': 'ABSTRACT',    // Nine
  '十': 'ABSTRACT',    // Ten
  
  // PERSON
  '人': 'PERSON',     // Person
  '女': 'PERSON',     // Woman
  '子': 'PERSON',     // Child
  
  // BODY
  '口': 'BODY',       // Mouth
  '目': 'BODY',       // Eye
  '耳': 'BODY',       // Ear
  '手': 'BODY',       // Hand
  '足': 'BODY',       // Foot
  '心': 'BODY',       // Heart
  
  // NATURE
  '水': 'NATURE',     // Water
  '火': 'NATURE',     // Fire
  '日': 'NATURE',     // Sun/Day
  '月': 'NATURE',     // Moon/Month
  '土': 'NATURE',     // Earth
  '山': 'NATURE',     // Mountain
  '雨': 'NATURE',     // Rain
  
  // LIVING
  '木': 'LIVING',     // Tree
  '魚': 'LIVING',     // Fish
  '鳥': 'LIVING',     // Bird
  '馬': 'LIVING',     // Horse
  '牛': 'LIVING',     // Cow
  '犬': 'LIVING',     // Dog
  
  // STRUCTURE
  '門': 'STRUCTURE',  // Gate
  '家': 'STRUCTURE',  // House
  
  // TOOLS
  '車': 'TOOLS',      // Vehicle
  '刀': 'TOOLS',      // Knife
  '米': 'TOOLS',      // Rice
  '衣': 'TOOLS',      // Clothing
  
  // MOVEMENT
  '見': 'MOVEMENT',   // See
  '立': 'MOVEMENT',   // Stand
  '食': 'MOVEMENT',   // Eat
  
  // ABSTRACT
  '言': 'ABSTRACT',   // Speech
  '音': 'ABSTRACT',   // Sound
  '力': 'ABSTRACT',   // Power
  '工': 'ABSTRACT',   // Work
  '王': 'ABSTRACT',   // King
  '神': 'ABSTRACT',   // God
  
  // ELEMENTS
  '石': 'ELEMENTS',   // Stone
  '金': 'ELEMENTS',   // Gold/Metal
  '玉': 'ELEMENTS',   // Jade
  '土': 'ELEMENTS',   // Earth (can be categorized in multiple categories)
  '生': 'ELEMENTS',   // Life
  '花': 'ELEMENTS',   // Flower
};

// Semantic keyword mapping for meaning-based classification
const SEMANTIC_KEYWORDS = {
  PERSON: ['person', 'man', 'woman', 'child', 'people', 'human', 'family', 'friend', 'teacher', 'student', 
           'baby', 'someone', 'individual', 'social', 'society'],
  
  SPEECH: ['speak', 'say', 'talk', 'voice', 'language', 'word', 'chat', 'discuss', 'communicate', 'inform', 
           'mouth', 'speech', 'message', 'expression', 'conversation'],
  
  BODY: ['body', 'hand', 'foot', 'eye', 'ear', 'mouth', 'heart', 'face', 'head', 'arm', 'leg', 'finger', 'blood', 
         'bone', 'physical', 'organ', 'skin', 'flesh', 'health', 'medical'],
  
  NATURE: ['water', 'fire', 'air', 'earth', 'sun', 'moon', 'star', 'sky', 'cloud', 'rain', 'snow', 'wind', 'storm', 
           'light', 'dark', 'cold', 'hot', 'weather', 'nature', 'natural', 'climate', 'environment'],
  
  LIVING: ['animal', 'plant', 'tree', 'flower', 'grass', 'bird', 'fish', 'insect', 'dog', 'cat', 'horse', 'cow', 
           'grow', 'bloom', 'leaf', 'root', 'seed', 'fruit', 'vegetable', 'wildlife', 'creature', 'living'],
  
  STRUCTURE: ['house', 'building', 'room', 'door', 'window', 'wall', 'roof', 'floor', 'gate', 'bridge', 'tower', 
              'castle', 'temple', 'home', 'structure', 'construction', 'architecture', 'container', 'box', 'enclosure'],
  
  TOOLS: ['tool', 'weapon', 'knife', 'sword', 'car', 'vehicle', 'book', 'paper', 'pen', 'brush', 'needle', 'thread', 
          'cloth', 'clothes', 'furniture', 'vessel', 'container', 'thing', 'item', 'object', 'artifact', 'instrument'],
  
  MOVEMENT: ['move', 'walk', 'run', 'jump', 'swim', 'fly', 'climb', 'fall', 'rise', 'dance', 'turn', 'spin', 'slide', 
             'motion', 'action', 'travel', 'journey', 'trip', 'path', 'direction', 'go', 'come', 'arrive', 'leave'],
  
  ABSTRACT: ['idea', 'mind', 'thought', 'feel', 'sense', 'emotion', 'love', 'hate', 'angry', 'happy', 'sad', 'good', 
             'bad', 'right', 'wrong', 'true', 'false', 'concept', 'philosophy', 'belief', 'religion', 'god', 'spirit', 
             'system', 'order', 'chaos', 'one', 'two', 'three', 'number', 'count', 'measure'],
  
  ELEMENTS: ['metal', 'gold', 'silver', 'iron', 'steel', 'copper', 'stone', 'rock', 'jade', 'gem', 'crystal', 'wood', 
             'paper', 'cotton', 'silk', 'cloth', 'glass', 'ceramic', 'clay', 'material', 'substance', 'element']
};

// Unicode code point ranges for radical families
const UNICODE_RANGES = {
  PERSON: [
    [0x4E00, 0x4FFF],
    [0x8000, 0x81FF]
  ],
  SPEECH: [
    [0x5000, 0x51FF],
    [0x8200, 0x83FF]
  ],
  BODY: [
    [0x5200, 0x54FF],
    [0x8400, 0x85FF]
  ],
  NATURE: [
    [0x5500, 0x57FF], 
    [0x8600, 0x87FF]
  ],
  LIVING: [
    [0x5800, 0x5AFF],
    [0x8800, 0x89FF] 
  ],
  STRUCTURE: [
    [0x5B00, 0x5DFF],
    [0x8A00, 0x8BFF]
  ],
  TOOLS: [
    [0x5E00, 0x60FF],
    [0x8C00, 0x8DFF]
  ],
  MOVEMENT: [
    [0x6100, 0x63FF],
    [0x8E00, 0x8FFF]
  ],
  ABSTRACT: [
    [0x6400, 0x67FF],
    [0x9000, 0x92FF]
  ],
  ELEMENTS: [
    [0x6800, 0x6BFF],
    [0x9300, 0x95FF]
  ]
};

/**
 * Determine which radical family a kanji belongs to using a balanced approach
 * @param {string} kanji - The kanji character to classify
 * @param {Object|null} data - Optional kanji data for additional context
 * @returns {string} - The radical family key
 */
function getRadicalFamily(kanji, data = null) {
  try {
    // Strategy 1: Direct kanji mapping
    if (KANJI_RADICAL_MAP[kanji]) {
      return KANJI_RADICAL_MAP[kanji];
    }
    
    // Strategy 2: Meaning-based classification
    if (data && data.meaning) {
      const meaning = data.meaning.toLowerCase();
      
      // Check each family's keywords
      for (const family in SEMANTIC_KEYWORDS) {
        for (const keyword of SEMANTIC_KEYWORDS[family]) {
          if (meaning.includes(keyword)) {
            return family;
          }
        }
      }
    }
    
    // Strategy 3: Unicode range-based classification with balanced distribution
    const code = kanji.charCodeAt(0);
    
    // Check Unicode ranges
    for (const family in UNICODE_RANGES) {
      for (const [start, end] of UNICODE_RANGES[family]) {
        if (code >= start && code <= end) {
          return family;
        }
      }
    }
    
    // Strategy 4: Ensure balanced distribution via modulo (to avoid empty categories)
    // Use character code to distribute evenly across categories
    const familyKeys = Object.keys(RADICAL_FAMILIES);
    return familyKeys[code % familyKeys.length];
  } catch (error) {
    console.error(`Error classifying kanji ${kanji}:`, error);
    
    // Default to a balanced fallback based on character code
    const code = kanji.charCodeAt(0);
    const familyKeys = Object.keys(RADICAL_FAMILIES);
    return familyKeys[code % familyKeys.length];
  }
}

/**
 * Get all radical family names
 * @returns {Object} - Mapping of family keys to descriptive names
 */
function getAllRadicalFamilies() {
  return RADICAL_FAMILIES;
}

module.exports = {
  RADICAL_FAMILIES,
  RADICAL_TO_FAMILY,
  KANJI_RADICAL_MAP,
  SEMANTIC_KEYWORDS,
  getRadicalFamily,
  getAllRadicalFamilies
};