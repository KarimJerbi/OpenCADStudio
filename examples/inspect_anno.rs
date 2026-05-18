// Inspect DIMSCALE, dimstyles, text heights, and xdata to diagnose
// annotation-scale bugs in paper-space viewports.
//
// cargo run --release --example inspect_anno -- <file>

use acadrust::entities::{Dimension, EntityType};
use acadrust::io::dwg::DwgReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("usage: inspect_anno <file>");
    let doc = DwgReader::from_file(&path)?.read()?;
    println!("file: {}\n", path);

    println!("─── header ──────────────────────────────");
    println!("  DIMSCALE (dim_scale)      = {}", doc.header.dim_scale);
    println!("  DIMTXT   (dim_text_height)= {}", doc.header.dim_text_height);
    println!("  DIMASZ   (dim_arrow_size) = {}", doc.header.dim_arrow_size);
    println!("  TEXTSIZE (text_height)    = {}", doc.header.text_height);
    println!("  LTSCALE                   = {}", doc.header.linetype_scale);
    println!("  PSLTSCALE (psls)          = {}", doc.header.paper_space_linetype_scaling);
    println!("  CELTSCALE                 = {}", doc.header.current_entity_linetype_scale);
    println!("  viewport_scale_factor     = {}", doc.header.viewport_scale_factor);
    println!();
    println!("─── linetypes (first 10) ────────────────");
    let mut shown = 0;
    for lt in doc.line_types.iter() {
        if shown >= 10 { break; }
        println!(
            "  {:<24} pattern_len={:.4}  elements={}",
            lt.name, lt.pattern_length, lt.elements.len()
        );
        shown += 1;
    }
    println!();

    println!("─── dim_styles ──────────────────────────");
    println!(
        "  {:<24} {:>10} {:>10} {:>10} {:>10}",
        "name", "dimscale", "dimtxt", "dimasz", "dimexe"
    );
    for s in doc.dim_styles.iter() {
        println!(
            "  {:<24} {:>10.4} {:>10.4} {:>10.4} {:>10.4}",
            s.name, s.dimscale, s.dimtxt, s.dimasz, s.dimexe
        );
    }
    println!();

    println!("─── text_styles ─────────────────────────");
    for s in doc.text_styles.iter() {
        println!(
            "  {:<24}  height={:.4}  last_height={:.4}  font={:?}",
            s.name, s.height, s.last_height, s.font_file
        );
    }
    println!();

    let mut n_text = 0u32;
    let mut n_text_with_xdata = 0u32;
    let mut n_text_with_acanno = 0u32;
    let mut n_mtext = 0u32;
    let mut n_mtext_with_xdata = 0u32;
    let mut n_mtext_with_acanno = 0u32;
    let mut n_dim = 0u32;
    let mut n_dim_with_xdata = 0u32;
    let mut n_dim_with_acanno = 0u32;
    let mut xd_app_counts: std::collections::BTreeMap<String, u32> = Default::default();

    let mut sample_text_h: Vec<f64> = vec![];
    let mut sample_mtext_h: Vec<f64> = vec![];
    let mut sample_dim_style: std::collections::BTreeMap<String, u32> = Default::default();
    // For MTEXT: bucket by owner block name (paper vs model) and height range.
    let mut mtext_by_owner: std::collections::BTreeMap<String, Vec<f64>> = Default::default();

    let acanno_keys = ["AcAnnoPO", "AcAnnotativeData", "AcAnnoScaleObj", "AcDbXrecObject"];

    for e in doc.entities() {
        match e {
            EntityType::Text(t) => {
                n_text += 1;
                let xd = &t.common.extended_data;
                let has_xd = !xd.records().is_empty();
                if has_xd {
                    n_text_with_xdata += 1;
                    for r in xd.records() {
                        *xd_app_counts.entry(r.application_name.clone()).or_default() += 1;
                    }
                }
                if acanno_keys.iter().any(|k| xd.get_record(k).is_some()) {
                    n_text_with_acanno += 1;
                }
                if sample_text_h.len() < 10 {
                    sample_text_h.push(t.height);
                }
            }
            EntityType::MText(t) => {
                n_mtext += 1;
                let xd = &t.common.extended_data;
                let has_xd = !xd.records().is_empty();
                if has_xd {
                    n_mtext_with_xdata += 1;
                    for r in xd.records() {
                        *xd_app_counts.entry(r.application_name.clone()).or_default() += 1;
                    }
                }
                if acanno_keys.iter().any(|k| xd.get_record(k).is_some()) {
                    n_mtext_with_acanno += 1;
                }
                if sample_mtext_h.len() < 10 {
                    sample_mtext_h.push(t.height);
                }
                // Look up the owning block-record by handle and label as
                // "*Model_Space" / "*Paper_Space*" / "<other>".
                let owner_h = t.common.owner_handle;
                let owner_label = doc
                    .block_records
                    .iter()
                    .find_map(|br: &acadrust::BlockRecord| {
                        if br.handle == owner_h {
                            Some(br.name.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| format!("h{}", owner_h.value()));
                mtext_by_owner
                    .entry(owner_label)
                    .or_default()
                    .push(t.height);
            }
            EntityType::Dimension(d) => {
                n_dim += 1;
                let base = match d {
                    Dimension::Aligned(d) => &d.base,
                    Dimension::Linear(d) => &d.base,
                    Dimension::Radius(d) => &d.base,
                    Dimension::Diameter(d) => &d.base,
                    Dimension::Angular2Ln(d) => &d.base,
                    Dimension::Angular3Pt(d) => &d.base,
                    Dimension::Ordinate(d) => &d.base,
                };
                let xd = &base.common.extended_data;
                let has_xd = !xd.records().is_empty();
                if has_xd {
                    n_dim_with_xdata += 1;
                    for r in xd.records() {
                        *xd_app_counts.entry(r.application_name.clone()).or_default() += 1;
                    }
                }
                if acanno_keys.iter().any(|k| xd.get_record(k).is_some()) {
                    n_dim_with_acanno += 1;
                }
                *sample_dim_style.entry(base.style_name.clone()).or_default() += 1;
            }
            _ => {}
        }
    }

    println!("─── annotation entity counts ────────────");
    println!(
        "  TEXT      total={:<5} xdata={:<5} AcAnno={}",
        n_text, n_text_with_xdata, n_text_with_acanno
    );
    println!(
        "  MTEXT     total={:<5} xdata={:<5} AcAnno={}",
        n_mtext, n_mtext_with_xdata, n_mtext_with_acanno
    );
    println!(
        "  DIMENSION total={:<5} xdata={:<5} AcAnno={}",
        n_dim, n_dim_with_xdata, n_dim_with_acanno
    );
    println!();

    println!("─── xdata application names seen ────────");
    for (k, v) in &xd_app_counts {
        println!("  {:>5}  {}", v, k);
    }
    println!();

    if !sample_text_h.is_empty() {
        println!("─── first 10 TEXT heights ───────────────");
        for h in &sample_text_h {
            println!("  {:.4}", h);
        }
        println!();
    }
    if !sample_mtext_h.is_empty() {
        println!("─── first 10 MTEXT heights ──────────────");
        for h in &sample_mtext_h {
            println!("  {:.4}", h);
        }
        println!();
    }
    if !mtext_by_owner.is_empty() {
        println!("─── MTEXT by owner ──────────────────────");
        for (owner, heights) in &mtext_by_owner {
            let mut h = heights.clone();
            h.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let min = h.first().copied().unwrap_or(0.0);
            let max = h.last().copied().unwrap_or(0.0);
            let median = h[h.len() / 2];
            let n_below_10 = h.iter().filter(|&&x| x < 10.0).count();
            let n_above_100 = h.iter().filter(|&&x| x > 100.0).count();
            println!(
                "  {:<20} n={:<4} min={:.2} med={:.2} max={:.2}  <10:{}  >100:{}",
                owner,
                h.len(),
                min,
                median,
                max,
                n_below_10,
                n_above_100
            );
        }
        println!();
    }

    if !sample_dim_style.is_empty() {
        println!("─── dimension styles used (count) ───────");
        for (k, v) in &sample_dim_style {
            let used_style = doc.dim_styles.iter().find(|s| s.name == *k);
            let ds = used_style.map(|s| s.dimscale).unwrap_or(f64::NAN);
            let dt = used_style.map(|s| s.dimtxt).unwrap_or(f64::NAN);
            let da = used_style.map(|s| s.dimasz).unwrap_or(f64::NAN);
            println!(
                "  {:>5}× style=\"{}\"  dimscale={:.4} dimtxt={:.4} dimasz={:.4}",
                v, k, ds, dt, da
            );
        }
    }

    Ok(())
}
