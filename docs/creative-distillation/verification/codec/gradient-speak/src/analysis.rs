//! Text analysis functions for converting regular text into Vector13D representations.
//! This makes gradient-speak more practical by providing analysis capabilities.

use vector13d::{Vector13D, DomainWall, GaugeCoupling};

/// Analyze text and convert to Vector13D representations based on emotional content
pub fn analyze_text_to_vectors(text: &str) -> Vec<Vector13D> {
    text.chars().map(|c| analyze_char_to_vector(c)).collect()
}

/// Analyze a single character and convert to Vector13D
pub fn analyze_char_to_vector(c: char) -> Vector13D {
    // Simple heuristic analysis based on character properties
    let amplitude = char_amplitude(c);
    let frequency = char_frequency(c);
    let composition = char_composition(c);
    let resonance = char_resonance(c);
    let su2_polarity = char_hue(c);
    let torsion = char_torsion(c);
    
    Vector13D {
        amplitude,
        frequency,
        phase: 0.0,
        coherence: 0.7,
        entropy: char_entropy(c),
        composition,
        resonance,
        ozone_buffer: 0.5,
        domain_wall: if c.is_alphabetic() { DomainWall::Linked } else { DomainWall::Broken },
        su2_polarity,
        torsion,
        gauge_coupling: if c.is_uppercase() { GaugeCoupling::Spinning } else { GaugeCoupling::Static },
        closure: 0.8,
    }
}

/// Determine amplitude based on character "weight"
fn char_amplitude(c: char) -> f64 {
    match c {
        'A'..='Z' => 0.8, // Uppercase has more weight
        'a'..='z' => 0.6, // Lowercase has moderate weight
        '0'..='9' => 0.5, // Numbers have moderate weight
        ' ' | '\n' | '\t' => 0.1, // Whitespace has minimal weight
        '!' | '?' | '.' => 0.9, // Punctuation has high weight
        _ => 0.4, // Other characters have lower weight
    }
}

/// Determine frequency based on character "activity"
fn char_frequency(c: char) -> f64 {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => 0.8, // Vowels are high frequency
        ' ' => 0.1, // Spaces are low frequency
        '\n' => 0.2, // Newlines are low frequency
        _ => 0.5, // Other characters are moderate frequency
    }
}

/// Determine composition (truth meter) based on character "expressiveness"
fn char_composition(c: char) -> f64 {
    match c {
        'a'..='z' => 0.7, // Lowercase is moderately expressive
        'A'..='Z' => 0.9, // Uppercase is highly expressive
        '0'..='9' => 0.5, // Numbers are moderately expressive
        '!' | '?' => 0.95, // Exclamation and question marks are highly expressive
        '.' => 0.6, // Period is moderately expressive
        _ => 0.4, // Other characters are less expressive
    }
}

/// Determine resonance (bandwidth/focus) based on character
fn char_resonance(c: char) -> f64 {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => 0.8, // Vowels have high resonance
        ' ' => 0.2, // Spaces have low resonance
        _ => 0.5, // Other characters have moderate resonance
    }
}

/// Determine entropy (disorder) based on character
fn char_entropy(c: char) -> f64 {
    match c {
        ' ' => 0.1, // Spaces have low entropy
        'a' | 'e' | 'i' | 'o' | 'u' => 0.3, // Common vowels have low entropy
        'z' | 'x' | 'q' => 0.8, // Rare letters have high entropy
        _ => 0.5, // Other characters have moderate entropy
    }
}

/// Determine hue (su2_polarity) based on character
fn char_hue(c: char) -> f64 {
    // Map characters to hue values [0, 360)
    let ascii_val = c as u32;
    match c {
        'a'..='z' => ((ascii_val - 97) * 14) as f64 % 360.0, // Spread lowercase across spectrum
        'A'..='Z' => ((ascii_val - 65) * 14) as f64 % 360.0, // Spread uppercase across spectrum
        '0'..='9' => ((ascii_val - 48) * 36) as f64 % 360.0, // Spread numbers across spectrum
        ' ' => 200.0, // Space -> blue
        '\n' => 120.0, // Newline -> green
        _ => (ascii_val * 7) as f64 % 360.0, // Other characters
    }
}

/// Determine torsion (skew) based on character
fn char_torsion(c: char) -> f64 {
    match c {
        'i' | 'j' | 'l' | 't' => -45.0, // Tall thin letters lean left
        'k' | 'r' | 'x' => 45.0, // Angular letters lean right
        'a' | 'c' | 'e' | 'o' => 0.0, // Round letters are balanced
        'm' | 'n' | 'w' => 15.0, // Wide letters lean slightly right
        _ => 0.0, // Other characters are balanced
    }
}

/// Analyze text for emotional tone
pub fn analyze_emotional_tone(text: &str) -> EmotionalTone {
    let lowercase = text.to_lowercase();
    let total_chars = text.chars().filter(|c| c.is_alphabetic()).count();
    
    if total_chars == 0 {
        return EmotionalTone {
            excitement: 0.0,
            calmness: 0.5,
            urgency: 0.0,
            certainty: 0.5,
        };
    }
    
    let exclamations = lowercase.matches('!').count();
    let questions = lowercase.matches('?').count();
    let periods = lowercase.matches('.').count();
    
    let excitement = (exclamations as f64 / total_chars as f64) * 2.0;
    let urgency = (questions as f64 / total_chars as f64) * 1.5;
    let calmness = 1.0 - urgency.min(1.0);
    let certainty = if periods > 0 { 0.8 } else { 0.4 };
    
    EmotionalTone {
        excitement: excitement.min(1.0),
        calmness: calmness.max(0.0),
        urgency: urgency.min(1.0),
        certainty,
    }
}

#[derive(Debug, Clone)]
pub struct EmotionalTone {
    pub excitement: f64,
    pub calmness: f64,
    pub urgency: f64,
    pub certainty: f64,
}

/// Apply emotional tone to Vector13D
pub fn apply_emotional_tone(vector: &mut Vector13D, tone: &EmotionalTone) {
    vector.amplitude = (vector.amplitude + tone.excitement * 0.3).min(1.0);
    vector.composition = (vector.composition + tone.certainty * 0.2).min(1.0);
    vector.frequency = (vector.frequency + tone.urgency * 0.2).min(1.0);
    vector.resonance = (vector.resonance + tone.calmness * 0.2).min(1.0);
}

/// Convert text with emotional analysis to Vector13D
pub fn text_to_emotional_vectors(text: &str) -> Vec<Vector13D> {
    let tone = analyze_emotional_tone(text);
    let mut vectors = analyze_text_to_vectors(text);
    
    for vector in &mut vectors {
        apply_emotional_tone(vector, &tone);
    }
    
    vectors
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_char_analysis() {
        let vector = analyze_char_to_vector('A');
        assert!(vector.amplitude > 0.7); // Uppercase should have high amplitude
        assert!(vector.composition > 0.8); // Uppercase should be highly expressive
    }
    
    #[test]
    fn test_text_analysis() {
        let vectors = analyze_text_to_vectors("Hello");
        assert_eq!(vectors.len(), 5);
        assert!(vectors[0].amplitude > vectors[1].amplitude); // 'H' > 'e'
    }
    
    #[test]
    fn test_emotional_tone() {
        let excited = analyze_emotional_tone("Hello!!!");
        assert!(excited.excitement > 0.5);
        
        let calm = analyze_emotional_tone("Hello world.");
        assert!(calm.calmness > 0.5);
    }
    
    #[test]
    fn test_emotional_vectors() {
        let vectors = text_to_emotional_vectors("URGENT!");
        assert!(vectors.len() == 7);
        // Last character should have high amplitude due to excitement
        assert!(vectors[6].amplitude > 0.8);
    }
    
    #[test]
    fn test_hue_distribution() {
        let vectors = analyze_text_to_vectors("abcdefghijklmnopqrstuvwxyz");
        let hues: Vec<f64> = vectors.iter().map(|v| v.su2_polarity).collect();
        // Should have variety in hues - check standard deviation
        let mean: f64 = hues.iter().sum::<f64>() / hues.len() as f64;
        let variance: f64 = hues.iter().map(|&h| (h - mean).powi(2)).sum::<f64>() / hues.len() as f64;
        let std_dev = variance.sqrt();
        assert!(std_dev > 50.0, "Hues should have variety, but std_dev was {}", std_dev);
    }
}