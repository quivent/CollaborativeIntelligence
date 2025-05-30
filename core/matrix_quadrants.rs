// Matrix Quadrant System for Collaborative Intelligence
// Implements flexible quadrant-based matrix analysis and visualization

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuadrantType {
    Performance,     // High/Low Performance vs High/Low Complexity
    Priority,        // Urgent/Not Urgent vs Important/Not Important
    Capability,      // Current Skill vs Learning Potential
    Risk,           // High/Low Impact vs High/Low Probability
    Decision,       // Quick/Slow vs Simple/Complex
    Knowledge,      // Known/Unknown vs Critical/Non-Critical
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadrantPosition {
    pub x: f64,  // Horizontal axis (-1.0 to 1.0)
    pub y: f64,  // Vertical axis (-1.0 to 1.0)
}

impl QuadrantPosition {
    pub fn new(x: f64, y: f64) -> Self {
        QuadrantPosition {
            x: x.clamp(-1.0, 1.0),
            y: y.clamp(-1.0, 1.0),
        }
    }

    pub fn get_quadrant(&self) -> u8 {
        match (self.x >= 0.0, self.y >= 0.0) {
            (true, true) => 1,   // Top-right
            (false, true) => 2,  // Top-left
            (false, false) => 3, // Bottom-left
            (true, false) => 4,  // Bottom-right
        }
    }

    pub fn get_quadrant_name(&self, matrix_type: &QuadrantType) -> String {
        match (matrix_type, self.get_quadrant()) {
            (QuadrantType::Performance, 1) => "High Performance, High Complexity".to_string(),
            (QuadrantType::Performance, 2) => "High Performance, Low Complexity".to_string(),
            (QuadrantType::Performance, 3) => "Low Performance, Low Complexity".to_string(),
            (QuadrantType::Performance, 4) => "Low Performance, High Complexity".to_string(),
            
            (QuadrantType::Priority, 1) => "Important & Urgent".to_string(),
            (QuadrantType::Priority, 2) => "Important, Not Urgent".to_string(),
            (QuadrantType::Priority, 3) => "Not Important, Not Urgent".to_string(),
            (QuadrantType::Priority, 4) => "Not Important, Urgent".to_string(),
            
            (QuadrantType::Capability, 1) => "High Skill, High Potential".to_string(),
            (QuadrantType::Capability, 2) => "High Skill, Low Potential".to_string(),
            (QuadrantType::Capability, 3) => "Low Skill, Low Potential".to_string(),
            (QuadrantType::Capability, 4) => "Low Skill, High Potential".to_string(),
            
            (QuadrantType::Risk, 1) => "High Impact, High Probability".to_string(),
            (QuadrantType::Risk, 2) => "High Impact, Low Probability".to_string(),
            (QuadrantType::Risk, 3) => "Low Impact, Low Probability".to_string(),
            (QuadrantType::Risk, 4) => "Low Impact, High Probability".to_string(),
            
            (QuadrantType::Decision, 1) => "Complex, Time-Sensitive".to_string(),
            (QuadrantType::Decision, 2) => "Complex, Low Time Pressure".to_string(),
            (QuadrantType::Decision, 3) => "Simple, Low Time Pressure".to_string(),
            (QuadrantType::Decision, 4) => "Simple, Time-Sensitive".to_string(),
            
            (QuadrantType::Knowledge, 1) => "Known & Critical".to_string(),
            (QuadrantType::Knowledge, 2) => "Unknown & Critical".to_string(),
            (QuadrantType::Knowledge, 3) => "Unknown & Non-Critical".to_string(),
            (QuadrantType::Knowledge, 4) => "Known & Non-Critical".to_string(),
            _ => "Unknown Quadrant".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub position: QuadrantPosition,
    pub metadata: HashMap<String, String>,
    pub score: Option<f64>,
}

impl MatrixItem {
    pub fn new(id: String, name: String, x: f64, y: f64) -> Self {
        MatrixItem {
            id,
            name,
            description: None,
            position: QuadrantPosition::new(x, y),
            metadata: HashMap::new(),
            score: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_score(mut self, score: f64) -> Self {
        self.score = Some(score);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadrantMatrix {
    pub matrix_type: QuadrantType,
    pub title: String,
    pub x_axis_label: String,
    pub y_axis_label: String,
    pub items: Vec<MatrixItem>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl QuadrantMatrix {
    pub fn new(
        matrix_type: QuadrantType,
        title: String,
        x_axis_label: String,
        y_axis_label: String,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        QuadrantMatrix {
            matrix_type,
            title,
            x_axis_label,
            y_axis_label,
            items: Vec::new(),
            created_at: timestamp,
            updated_at: timestamp,
        }
    }

    pub fn add_item(&mut self, item: MatrixItem) {
        self.items.push(item);
        self.update_timestamp();
    }

    pub fn remove_item(&mut self, id: &str) -> Option<MatrixItem> {
        let position = self.items.iter().position(|item| item.id == id)?;
        self.update_timestamp();
        Some(self.items.remove(position))
    }

    pub fn get_items_in_quadrant(&self, quadrant: u8) -> Vec<&MatrixItem> {
        self.items
            .iter()
            .filter(|item| item.position.get_quadrant() == quadrant)
            .collect()
    }

    pub fn get_quadrant_summary(&self) -> HashMap<u8, Vec<String>> {
        let mut summary = HashMap::new();
        
        for quadrant in 1..=4 {
            let items: Vec<String> = self.get_items_in_quadrant(quadrant)
                .iter()
                .map(|item| item.name.clone())
                .collect();
            summary.insert(quadrant, items);
        }
        
        summary
    }

    pub fn update_item_position(&mut self, id: &str, new_position: QuadrantPosition) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.position = new_position;
            self.update_timestamp();
            true
        } else {
            false
        }
    }

    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn render_ascii(&self) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("📊 {}\n", self.title));
        output.push_str(&format!("🔄 {}\n", "=".repeat(60)));
        output.push_str(&format!("📈 X-Axis: {} | Y-Axis: {}\n\n", self.x_axis_label, self.y_axis_label));

        // Quadrant headers
        output.push_str("┌─────────────────────────────┬─────────────────────────────┐\n");
        output.push_str(&format!("│ Q2: {} │ Q1: {} │\n", 
            self.get_quadrant_label(2), self.get_quadrant_label(1)));
        output.push_str("├─────────────────────────────┼─────────────────────────────┤\n");

        // Items in quadrants 2 and 1
        let q2_items = self.get_items_in_quadrant(2);
        let q1_items = self.get_items_in_quadrant(1);
        let max_items = q2_items.len().max(q1_items.len()).max(3);

        for i in 0..max_items {
            let q2_item = q2_items.get(i).map(|item| item.name.as_str()).unwrap_or("");
            let q1_item = q1_items.get(i).map(|item| item.name.as_str()).unwrap_or("");
            output.push_str(&format!("│ {:27} │ {:27} │\n", 
                truncate_string(q2_item, 27), truncate_string(q1_item, 27)));
        }

        output.push_str("├─────────────────────────────┼─────────────────────────────┤\n");
        output.push_str(&format!("│ Q3: {} │ Q4: {} │\n", 
            self.get_quadrant_label(3), self.get_quadrant_label(4)));
        output.push_str("├─────────────────────────────┼─────────────────────────────┤\n");

        // Items in quadrants 3 and 4
        let q3_items = self.get_items_in_quadrant(3);
        let q4_items = self.get_items_in_quadrant(4);
        let max_items = q3_items.len().max(q4_items.len()).max(3);

        for i in 0..max_items {
            let q3_item = q3_items.get(i).map(|item| item.name.as_str()).unwrap_or("");
            let q4_item = q4_items.get(i).map(|item| item.name.as_str()).unwrap_or("");
            output.push_str(&format!("│ {:27} │ {:27} │\n", 
                truncate_string(q3_item, 27), truncate_string(q4_item, 27)));
        }

        output.push_str("└─────────────────────────────┴─────────────────────────────┘\n");

        output
    }

    fn get_quadrant_label(&self, quadrant: u8) -> String {
        let position = match quadrant {
            1 => QuadrantPosition::new(0.5, 0.5),
            2 => QuadrantPosition::new(-0.5, 0.5),
            3 => QuadrantPosition::new(-0.5, -0.5),
            4 => QuadrantPosition::new(0.5, -0.5),
            _ => QuadrantPosition::new(0.0, 0.0),
        };
        
        let full_name = position.get_quadrant_name(&self.matrix_type);
        // Truncate for display
        if full_name.len() > 25 {
            format!("{}...", &full_name[..22])
        } else {
            full_name
        }
    }

    pub fn export_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn import_from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

// Predefined matrix templates
pub struct MatrixTemplates;

impl MatrixTemplates {
    pub fn agent_performance_matrix() -> QuadrantMatrix {
        QuadrantMatrix::new(
            QuadrantType::Performance,
            "Agent Performance Analysis".to_string(),
            "Task Complexity →".to_string(),
            "Performance Level ↑".to_string(),
        )
    }

    pub fn priority_matrix() -> QuadrantMatrix {
        QuadrantMatrix::new(
            QuadrantType::Priority,
            "Task Priority Matrix (Eisenhower)".to_string(),
            "Urgency →".to_string(),
            "Importance ↑".to_string(),
        )
    }

    pub fn capability_matrix() -> QuadrantMatrix {
        QuadrantMatrix::new(
            QuadrantType::Capability,
            "Agent Capability Assessment".to_string(),
            "Learning Potential →".to_string(),
            "Current Skill Level ↑".to_string(),
        )
    }

    pub fn risk_matrix() -> QuadrantMatrix {
        QuadrantMatrix::new(
            QuadrantType::Risk,
            "Risk Assessment Matrix".to_string(),
            "Probability →".to_string(),
            "Impact ↑".to_string(),
        )
    }

    pub fn knowledge_matrix() -> QuadrantMatrix {
        QuadrantMatrix::new(
            QuadrantType::Knowledge,
            "Knowledge Gap Analysis".to_string(),
            "Criticality →".to_string(),
            "Knowledge Status ↑".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadrant_positioning() {
        let pos = QuadrantPosition::new(0.5, 0.8);
        assert_eq!(pos.get_quadrant(), 1);
        
        let pos = QuadrantPosition::new(-0.3, 0.2);
        assert_eq!(pos.get_quadrant(), 2);
    }

    #[test]
    fn test_matrix_creation() {
        let mut matrix = MatrixTemplates::priority_matrix();
        
        let item = MatrixItem::new("task1".to_string(), "Critical Bug Fix".to_string(), 0.8, 0.9)
            .with_description("Fix authentication bypass".to_string())
            .with_score(9.5);
            
        matrix.add_item(item);
        assert_eq!(matrix.items.len(), 1);
        assert_eq!(matrix.get_items_in_quadrant(1).len(), 1);
    }

    #[test]
    fn test_quadrant_summary() {
        let mut matrix = MatrixTemplates::priority_matrix();
        
        matrix.add_item(MatrixItem::new("t1".to_string(), "Important & Urgent".to_string(), 0.5, 0.5));
        matrix.add_item(MatrixItem::new("t2".to_string(), "Important Only".to_string(), -0.5, 0.5));
        
        let summary = matrix.get_quadrant_summary();
        assert_eq!(summary.get(&1).unwrap().len(), 1);
        assert_eq!(summary.get(&2).unwrap().len(), 1);
    }
}