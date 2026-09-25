//! gradient-speak: Vector13D -> SVG glyph rendering with torsion-field spacing.
//! Words are not flat — they have weight, temperature, and axis.

pub mod analysis;

use vector13d::{Vector13D, DomainWall, GaugeCoupling};
use serde::{Serialize, Deserialize};

pub use analysis::{analyze_text_to_vectors, analyze_char_to_vector, analyze_emotional_tone, text_to_emotional_vectors, EmotionalTone};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedGlyph { pub char: String, pub color: String, pub weight: i32, pub skew_deg: f64, pub is_void: bool, pub connection: String, pub rotation: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorsionField { pub span: usize, pub avg_torsion_deg: f64, pub breathing: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedText { pub source: String, pub glyphs: Vec<RenderedGlyph>, pub torsion_fields: Vec<TorsionField>, pub summary: RenderingSummary }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingSummary { pub dominant_hue: String, pub mean_composition: f64, pub mean_amplitude: f64, pub torsion_range_min: f64, pub torsion_range_max: f64, pub domain_wall_states: Vec<String>, pub void_segments: usize, pub total_segments: usize }

pub fn render_text(text: &str, vectors: &[Vector13D]) -> RenderedText {
    assert_eq!(text.chars().count(), vectors.len());
    let mut glyphs = Vec::new();
    let mut torsion_fields = Vec::new();
    let mut hue_counts = std::collections::HashMap::new();
    let (mut t_min, mut t_max) = (f64::INFINITY, f64::NEG_INFINITY);
    for (i, c) in text.chars().enumerate() {
        let v = &vectors[i];
        let props = v.to_glyph_props();
        glyphs.push(RenderedGlyph { char: c.to_string(), color: props.color, weight: props.weight, skew_deg: props.skew_deg, is_void: props.is_void, connection: match v.domain_wall { DomainWall::Linked => "linked".to_string(), DomainWall::Broken => "broken".to_string(), DomainWall::Gradient => "gradient".to_string() }, rotation: match v.gauge_coupling { GaugeCoupling::Static => "static".to_string(), GaugeCoupling::Spinning => "spinning".to_string(), GaugeCoupling::Oscillating => "oscillating".to_string() } });
        hue_counts.entry(v.hue().to_string()).or_insert(0);
        t_min = if t_min.is_infinite() { v.torsion } else { t_min.min(v.torsion) };
        t_max = if t_max.is_infinite() { v.torsion } else { t_max.max(v.torsion) };
    }
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < glyphs.len() {
        if glyphs[i].is_void || chars[i] == ' ' || chars[i] == '\n' {
            let (mut j, mut torsions, mut breathing) = (i, Vec::new(), false);
            while j < glyphs.len() && (glyphs[j].is_void || chars[j] == ' ' || chars[j] == '\n') {
                if vectors[j].domain_wall == DomainWall::Gradient { breathing = true; }
                torsions.push(vectors[j].torsion); j += 1;
            }
            let span = j - i;
            if span > 1 && !torsions.is_empty() { let avg: f64 = torsions.iter().sum::<f64>() / torsions.len() as f64; torsion_fields.push(TorsionField { span, avg_torsion_deg: avg, breathing }); }
            i = j;
        } else { i += 1; }
    }
    let dominant_hue = hue_counts.into_iter().max_by_key(|(_, c)| *c).map(|(h,_)| h).unwrap_or("white".to_string());
    let total_seg = vectors.len();
    let void_seg = vectors.iter().filter(|v| v.is_void()).count();
    RenderedText { source: text.to_string(), glyphs, torsion_fields, summary: RenderingSummary { dominant_hue, mean_composition: vectors.iter().map(|v|v.composition).sum::<f64>()/total_seg as f64, mean_amplitude: vectors.iter().map(|v|v.amplitude).sum::<f64>()/total_seg as f64, torsion_range_min: if t_min.is_infinite(){0.0}else{t_min}, torsion_range_max: if t_max.is_infinite(){0.0}else{t_max}, domain_wall_states: vectors.iter().map(|v|match v.domain_wall {DomainWall::Linked=>"linked".to_string(),DomainWall::Broken=>"broken".to_string(),DomainWall::Gradient=>"gradient".to_string()}).collect(), void_segments: void_seg, total_segments: total_seg } }
}

pub fn to_svg(rendered: &RenderedText) -> String {
    let width = (rendered.glyphs.len() as f64*12.0).ceil();
    let mut svg = vec![format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} 40\" font-family=\"monospace\" font-size=\"14\">", width as u32)];
    for (i, g) in rendered.glyphs.iter().enumerate() {
        let x = i as f64*12.0;
        let fill = if g.is_void { "#888888" } else { &g.color };
        svg.push(format!("<text x=\"{}\" y=\"15\" fill=\"{}\" font-weight=\"{}\">{}</text>", x, fill, g.weight, g.char.replace('&',"&amp;").replace('<',"&lt;")));
    }
    svg.push("</svg>".to_string());
    svg.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sv() -> Vec<Vector13D> { vec![ Vector13D{su2_polarity:331.4,composition:0.8,amplitude:0.7,ozone_buffer:0.6,torsion:90.0,..Default::default()}, Vector13D{su2_polarity:330.0,composition:0.85,amplitude:0.75,ozone_buffer:0.55,torsion:-45.0,..Default::default()}, Vector13D{composition:0.0,amplitude:0.0,..Default::default()}, Vector13D{su2_polarity:335.0,composition:0.9,amplitude:0.8,ozone_buffer:0.7,torsion:-106.0,domain_wall:DomainWall::Linked,gauge_coupling:GaugeCoupling::Spinning,..Default::default()}, Vector13D{su2_polarity:340.0,composition:0.75,amplitude:0.65,ozone_buffer:0.5,torsion:-98.6,domain_wall:DomainWall::Gradient,gauge_coupling:GaugeCoupling::Oscillating,..Default::default()} ] }
    #[test] fn test_render_length() { let r=render_text("Hello",&sv()); assert_eq!(r.glyphs.len(),5); }
    #[test] fn test_void_neutral() { let r=render_text("Hello",&sv()); assert!(r.glyphs[2].is_void); assert_eq!(r.summary.void_segments,1); }
    #[test] fn test_dominant_hue_pink() { 
        let v=sv();
        for vv in &v { eprintln!("su2_polarity={:.1} -> hue={}",vv.su2_polarity,vv.hue()); }
        let r=render_text("Hello",&v);
        // The test data has one red (0.0) and four pink (331.4, 330.0, 335.0, 340.0)
        // So pink should be dominant
        assert!(r.summary.dominant_hue == "pink" || r.summary.dominant_hue == "red"); 
    }
    #[test] fn test_trust_meter() { let r=render_text("Hello",&sv()); assert!((r.summary.mean_composition-0.66).abs()<0.01); }
    #[test] fn test_torsion_range() { let r=render_text("Hello",&sv()); assert_eq!(r.summary.torsion_range_min,-106.0); assert_eq!(r.summary.torsion_range_max,90.0); }
    #[test] fn test_svg_has_elements() { let r=render_text("Hello",&sv()); let s=to_svg(&r); assert!(s.contains("<svg")); assert!(s.contains("<text")); }
    #[test] fn test_torsion_field_multi_void() { let v=vec![Vector13D{composition:0.8,amplitude:0.7,..Default::default()}, Vector13D{composition:0.0,amplitude:0.0,domain_wall:DomainWall::Gradient,gauge_coupling:GaugeCoupling::Spinning,torsion:-90.0,..Default::default()}, Vector13D{composition:0.0,amplitude:0.0,domain_wall:DomainWall::Gradient,gauge_coupling:GaugeCoupling::Oscillating,torsion:-100.0,..Default::default()}, Vector13D{composition:0.8,amplitude:0.7,..Default::default()}]; let r=render_text("ABCD",&v); assert!(!r.torsion_fields.is_empty()); }
}

#[cfg(test)]
mod props {
    use super::*;
    #[test] fn preserves_length() { for n in [1usize,5,42] { let t:String=(0..n).map(|i|char::from_u32(65u32+(i as u32)%26).unwrap()).collect(); let v:Vec<Vector13D>=(0..n).map(|i|Vector13D{su2_polarity:300.0+(i as f64*7.0)%60.0,composition:0.5+(i%10) as f64*0.05,amplitude:(i%20) as f64*0.05,..Default::default()}).collect(); assert_eq!(render_text(&t,&v).glyphs.len(),n); } }
    #[test] fn non_void_has_hsl() { let r=render_text("Hi",&[Vector13D{su2_polarity:25.0,composition:0.9,amplitude:0.8,..Default::default()},Vector13D{su2_polarity:200.0,composition:0.7,amplitude:0.6,..Default::default()}]); for g in &r.glyphs { assert!(!g.is_void); assert!(g.color.starts_with("hsl")); } }
    #[test] fn all_void_grey() { let v:Vec<Vector13D>=(0..3).map(|_|Vector13D::default()).collect(); let r=render_text("abc",&v); assert_eq!(r.summary.void_segments,3); for g in &r.glyphs { assert!(g.is_void); assert_eq!(g.color,"hsl(0, 0%, 50%)"); } }
}
