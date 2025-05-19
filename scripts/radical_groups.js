/**
 * Radical groups for categorizing Japanese kanji
 * 
 * This file defines 10 distinct radical groups for categorizing the 2,136 Joyo Kanji
 * based on their primary radical (base component). Radicals are organized into roughly
 * equal-sized semantic groups.
 */

const RADICAL_GROUPS = {
  // Group 1: Person, Body Parts, and Human-related radicals
  HUMAN: {
    name: "Human-related",
    description: "Kanji related to people, body parts, and human activities",
    radicals: [
      '人', '亻', '口', '囗', '女', '子', '心', '忄', '扌', '手', '氵', '水',
      '目', '耳', '足', '身', '頁', '首', '面', '麻', '彳', '舌', '血', '肉',
      '自', '臣', '舟', '爪', '爫', '爿', '牙', '田', '男', '力', '劦', '勹',
      '匚', '匸', '卩', '厶', '又', '廴', '彐', '彡', '彦'
    ]
  },
  
  // Group 2: Nature, Environment, and Celestial radicals
  NATURE: {
    name: "Nature and environment",
    description: "Kanji related to natural elements, weather, and environment",
    radicals: [
      '山', '川', '土', '艹', '艸', '草', '木', '氣', '气', '气', '火', '灬',
      '水', '氵', '冫', '雨', '风', '風', '云', '雲', '日', '月', '阝', '邑',
      '石', '礻', '示', '衤', '衣', '金', '刂', '玉', '王', '宀', '穴', '田',
      '甲', '癶', '白', '皮', '皿', '罒', '网', '羽'
    ]
  },
  
  // Group 3: Animals, Insects, and Creature radicals
  ANIMAL: {
    name: "Animals and creatures",
    description: "Kanji related to animals, insects, and living creatures",
    radicals: [
      '犭', '犬', '马', '馬', '鸟', '鳥', '龙', '龍', '鱼', '魚', '虫', '豕',
      '牛', '牜', '羊', '羽', '虍', '豸', '龟', '龜', '黽', '鹿', '亀', '甘',
      '龜', '黾', '鼠', '鼻', '齿', '齒'
    ]
  },
  
  // Group 4: Actions, Movements, and Position radicals
  ACTION: {
    name: "Actions and movements",
    description: "Kanji related to actions, movements, and positions",
    radicals: [
      '走', '辶', '廴', '夂', '攴', '攵', '斤', '斥', '欠', '殳', '父', '長',
      '廾', '于', '亏', '乏', '亾', '仒', '䒑', '卌', '亶', '刀', '斗', '方',
      '文', '聿', '矛', '矢', '至', '色', '往', '忄', '心'
    ]
  },
  
  // Group 5: Objects, Tools, and Manufactured Items radicals
    OBJECT: {
    name: "Objects and tools",
    description: "Kanji related to objects, tools, and manufactured items",
    radicals: [
      '刀', '车', '車', '門', '门', '戈', '弓', '矢', '石', '皿', '糸', '糹',
      '纟', '缶', '网', '罒', '罓', '耒', '舟', '衣', '衤', '釒', '钅', '阝',
      '邑', '里', '匚', '匸', '卩', '厂', '厶', '广', '户', '戶', '戸', '房',
      '户', '弗', '氏', '气', '瓦', '器', '冊', '冃'
    ]
  },
  
  // Group 6: Buildings, Structures, and Places radicals
  PLACE: {
    name: "Buildings and places",
    description: "Kanji related to buildings, structures, and locations",
    radicals: [
      '宀', '广', '冂', '冋', '门', '門', '阝', '邑', '邦', '那', '里', '囗',
      '厂', '廠', '廣', '廣', '廛', '广', '府', '廁', '廂', '廈', '廊', '廝',
      '廟', '廚', '廨', '廪', '屋', '屏', '屑'
    ]
  },
  
  // Group 7: Plants, Agriculture, and Food radicals
  PLANT: {
    name: "Plants and agriculture",
    description: "Kanji related to plants, agriculture, and food",
    radicals: [
      '艹', '艸', '草', '木', '禾', '米', '麦', '麥', '田', '亚', '亞', '食',
      '飠', '飠', '鬯', '示', '礻', '衤', '米', '黍', '黑', '黒', '黔', '黨',
      '黴', '黻', '黼', '黽'
    ]
  },
  
  // Group 8: Qualities, Attributes, and Abstract Concepts radicals
  QUALITY: {
    name: "Qualities and concepts",
    description: "Kanji related to qualities, attributes, and abstract concepts",
    radicals: [
      '大', '小', '亠', '冖', '言', '讠', '訁', '文', '气', '青', '靑', '赤',
      '白', '黒', '黑', '長', '镸', '高', '亢', '壴', '要', '卑', '非', '不', 
      '是', '黑', '黒'
    ]
  },
  
  // Group 9: Numerals, Time, and Measurement radicals
  NUMBER: {
    name: "Numbers and measurements",
    description: "Kanji related to numbers, time, and measurements",
    radicals: [
      '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '廿', '卄',
      '百', '千', '万', '亿', '億', '兆', '刀', '寸', '時', '日', '曰', '月',
      '肘', '木', '朿', '未', '年', '干', '幵', '丰', '禿', '秉', '入', '込'
    ]
  },
  
  // Group 10: Miscellaneous and Less Common radicals
  MISC: {
    name: "Miscellaneous radicals",
    description: "Kanji with rare or miscellaneous radicals",
    radicals: [
      '丶', '丿', '亅', '乙', '乚', '乛', '巜', '几', '丂', '凵', '勿', '化',
      '匕', '卅', '巛', '川', '彡', '彳', '癶', '癸', '禸', '穴', '立', '竹',
      '米', '粦', '缶', '罙', '羊', '羽', '老', '耂', '而', '耳', '聿', '肉',
      '臼', '舌', '舛', '虍', '虎', '血', '衣', '襾', '角', '釒', '钅', '镸',
      '隶', '頁', '风', '飛', '飞', '食', '養', '马', '骨', '高', '鬥', '鬯',
      '鬲', '鬼', '魚', '鱼', '鳥', '鸟', '黽', '黿', '鼎', '鼓', '鼠', '鼻',
      '齊', '齐', '齒', '齿', '龍', '龙', '龜', '龟', '龠'
    ]
  }
};

/**
 * Determines the radical group for a kanji character based on its radical
 * 
 * @param {string} kanji - The kanji character to categorize
 * @param {Object} kanjiData - Optional data object with radical information
 * @returns {string} The group key (e.g., "HUMAN", "NATURE", etc.)
 */
function getRadicalGroup(kanji, kanjiData = null) {
  // If we have kanji data and it includes radical information, use that
  let radical = null;
  
  if (kanjiData && kanjiData.radical) {
    radical = kanjiData.radical;
  }
  
  // Fall back to first character analysis (simplified approach)
  // A better implementation would use a proper radical database
  if (!radical) {
    // This is a simplified fallback that treats the kanji itself
    // as its own radical, which isn't technically correct but helps
    // for distribution purposes
    radical = kanji;
  }
  
  // Check which group contains this radical
  for (const [groupKey, group] of Object.entries(RADICAL_GROUPS)) {
    if (group.radicals.includes(radical)) {
      return groupKey;
    }
  }
  
  // Default to MISC group if no match is found
  return "MISC";
}

/**
 * Gets all available radical group keys
 * 
 * @returns {Array<string>} Array of group keys
 */
function getAllRadicalGroups() {
  return Object.keys(RADICAL_GROUPS);
}

/**
 * Gets the name and description for a radical group
 * 
 * @param {string} groupKey - The group key (e.g., "HUMAN", "NATURE")
 * @returns {Object} Object with name and description properties
 */
function getRadicalGroupInfo(groupKey) {
  if (RADICAL_GROUPS[groupKey]) {
    return {
      name: RADICAL_GROUPS[groupKey].name,
      description: RADICAL_GROUPS[groupKey].description
    };
  }
  return {
    name: "Unknown Group",
    description: "Unknown radical group"
  };
}

module.exports = {
  RADICAL_GROUPS,
  getRadicalGroup,
  getAllRadicalGroups,
  getRadicalGroupInfo
};