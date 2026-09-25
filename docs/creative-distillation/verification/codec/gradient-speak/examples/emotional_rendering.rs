//! Example showing emotional text analysis and rendering with gradient-speak.
//! This demonstrates the practical application of the inclusive communication mission.

use gradient_speak::{
    render_text, to_svg, text_to_emotional_vectors, analyze_emotional_tone, EmotionalTone, RenderingSummary
};

fn main() {
    println!("🎨 Gradient-Speak Emotional Rendering Example");
    println!("==============================================\n");
    
    // Example 1: Excited text
    let excited_text = "This is AMAZING!!! I can't believe it works!";
    println!("📝 Excited text: \"{}\"", excited_text);
    
    let excited_tone = analyze_emotional_tone(excited_text);
    println!("🎭 Emotional analysis:");
    print_tone(&excited_tone);
    
    let excited_vectors = text_to_emotional_vectors(excited_text);
    let excited_rendered = render_text(excited_text, &excited_vectors);
    
    println!("🎨 Rendering summary:");
    print_rendering_summary(&excited_rendered.summary);
    
    // Example 2: Calm text
    let calm_text = "The system operates smoothly and reliably.";
    println!("\n📝 Calm text: \"{}\"", calm_text);
    
    let calm_tone = analyze_emotional_tone(calm_text);
    println!("🎭 Emotional analysis:");
    print_tone(&calm_tone);
    
    let calm_vectors = text_to_emotional_vectors(calm_text);
    let calm_rendered = render_text(calm_text, &calm_vectors);
    
    println!("🎨 Rendering summary:");
    print_rendering_summary(&calm_rendered.summary);
    
    // Example 3: Urgent text
    let urgent_text = "We need to fix this immediately! Time is critical!";
    println!("\n📝 Urgent text: \"{}\"", urgent_text);
    
    let urgent_tone = analyze_emotional_tone(urgent_text);
    println!("🎭 Emotional analysis:");
    print_tone(&urgent_tone);
    
    let urgent_vectors = text_to_emotional_vectors(urgent_text);
    let urgent_rendered = render_text(urgent_text, &urgent_vectors);
    
    println!("🎨 Rendering summary:");
    print_rendering_summary(&urgent_rendered.summary);
    
    // Example 4: Inclusive communication scenario
    println!("\n🌟 Inclusive Communication Scenario");
    println!("==================================");
    
    let inclusive_text = "I understand your perspective. Let me rephrase that to be clearer.";
    println!("📝 Inclusive text: \"{}\"", inclusive_text);
    
    let inclusive_tone = analyze_emotional_tone(inclusive_text);
    println!("🎭 Emotional analysis:");
    print_tone(&inclusive_tone);
    
    let inclusive_vectors = text_to_emotional_vectors(inclusive_text);
    let inclusive_rendered = render_text(inclusive_text, &inclusive_vectors);
    
    println!("🎨 Rendering summary:");
    print_rendering_summary(&inclusive_rendered.summary);
    
    // Demonstrate the difference between plain text and gradient-speak
    println!("\n📊 Traditional Text vs Gradient-Speak");
    println!("====================================");
    
    println!("Traditional text: \"{}\"", inclusive_text);
    println!("  - Flat characters");
    println!("  - No emotional weight");
    println!("  - No temperature or axis");
    println!("  - Lossy compression of intent");
    
    println!("\nGradient-speak:");
    println!("  - {} characters with emotional weight", inclusive_rendered.glyphs.len());
    println!("  - Dominant hue: {}", inclusive_rendered.summary.dominant_hue);
    println!("  - Mean composition (truth meter): {:.2}", inclusive_rendered.summary.mean_composition);
    println!("  - Mean amplitude (weight): {:.2}", inclusive_rendered.summary.mean_amplitude);
    println!("  - Torsion range: {:.1}° to {:.1}°", inclusive_rendered.summary.torsion_range_min, inclusive_rendered.summary.torsion_range_max);
    println!("  - Domain wall states: {:?}", inclusive_rendered.summary.domain_wall_states);
    println!("  - Void segments: {} / {}", inclusive_rendered.summary.void_segments, inclusive_rendered.summary.total_segments);
    
    println!("\n🌟 The Advantage:");
    println!("   Gradient-speak preserves what traditional text strips away:");
    println!("   - Emotional weight and intensity");
    println!("   - Temperature and urgency");
    println!("   - Axis and skew (temporal lean)");
    println!("   - Connection states (linked/broken/gradient)");
    println!("   - Rotation states (static/spinning/oscillating)");
    println!("   - This enables more inclusive and nuanced communication.");
    
    // Generate SVG output (truncated for display)
    println!("\n🎨 SVG Output (first 200 chars):");
    let svg = to_svg(&inclusive_rendered);
    let svg_preview: String = svg.chars().take(200).collect();
    println!("{}", svg_preview);
}

fn print_tone(tone: &EmotionalTone) {
    println!("  Excitement: {:.2}", tone.excitement);
    println!("  Calmness: {:.2}", tone.calmness);
    println!("  Urgency: {:.2}", tone.urgency);
    println!("  Certainty: {:.2}", tone.certainty);
}

fn print_rendering_summary(summary: &RenderingSummary) {
    println!("  Dominant hue: {}", summary.dominant_hue);
    println!("  Mean composition: {:.2}", summary.mean_composition);
    println!("  Mean amplitude: {:.2}", summary.mean_amplitude);
    println!("  Torsion range: {:.1}° to {:.1}°", summary.torsion_range_min, summary.torsion_range_max);
    println!("  Domain wall states: {:?}", summary.domain_wall_states);
    println!("  Void segments: {} / {}", summary.void_segments, summary.total_segments);
}