use planchet::models::{CoinSide, Demonetization, Issuer, NumistaType, Reference, RelatedType, RulingAuthority};
use url::Url;

// Centralized printing function.
fn println_indented(msg: &str, indent: usize) {
    println!("{:indent$}{}", "", msg, indent = indent);
}

fn print_key_value<T: std::fmt::Display>(key: &str, value: Option<T>, indent: usize) {
    if let Some(v) = value {
        println_indented(&format!("{}: {}", key, v), indent);
    }
}

fn print_coin_side(name: &str, side: Option<&CoinSide>, indent: usize) {
    if let Some(s) = side {
        println_indented(&format!("{}:", name), indent);
        let next_indent = indent + 2;
        if !s.engravers.is_empty() {
            println_indented(&format!("engravers: {}", s.engravers.join(", ")), next_indent);
        }
        if !s.designers.is_empty() {
            println_indented(&format!("designers: {}", s.designers.join(", ")), next_indent);
        }
        print_key_value("description", s.description.as_ref(), next_indent);
        print_key_value("lettering", s.lettering.as_ref(), next_indent);
        print_key_value("unabridged_legend", s.unabridged_legend.as_ref(), next_indent);
        print_key_value("lettering_translation", s.lettering_translation.as_ref(), next_indent);
        print_key_value("picture", s.picture.as_ref().map(Url::as_str), next_indent);
    }
}

fn print_demonetization(demonetization: Option<&Demonetization>, indent: usize) {
    if let Some(d) = demonetization {
        println_indented("demonetization:", indent);
        let next_indent = indent + 2;
        println_indented(&format!("is_demonetized: {}", d.is_demonetized), next_indent);
        if let Some(date) = d.demonetization_date {
            println_indented(&format!("demonetization_date: {}", date.format("%Y-%m-%d")), next_indent);
        }
    }
}

fn print_issuer(issuer: Option<&Issuer>, indent: usize) {
    if let Some(i) = issuer {
        println_indented("issuer:", indent);
        let next_indent = indent + 2;
        println_indented(&format!("code: {}", i.code), next_indent);
        println_indented(&format!("name: {}", i.name), next_indent);
    }
}

fn print_ruling_authorities(rulers: Option<&Vec<RulingAuthority>>, indent: usize) {
    if let Some(r) = rulers {
        if !r.is_empty() {
            println_indented("ruler:", indent);
            let next_indent = indent + 2;
            for ruler in r {
                println_indented(&format!("- {}", ruler.name), next_indent);
            }
        }
    }
}

fn print_references(references: Option<&Vec<Reference>>, indent: usize) {
    if let Some(r) = references {
        if !r.is_empty() {
            println_indented("references:", indent);
            let next_indent = indent + 2;
            for reference in r {
                println_indented(&format!("- {}: {}", reference.catalogue.code, reference.number), next_indent);
            }
        }
    }
}

fn print_related_types(related_types: Option<&Vec<RelatedType>>, indent: usize) {
    if let Some(r) = related_types {
        if !r.is_empty() {
            println_indented("related_types:", indent);
            let next_indent = indent + 2;
            for type_ in r {
                 println_indented(&format!("- [{}] {}", type_.id, type_.title), next_indent);
            }
        }
    }
}

pub fn print_numista_type(type_: Option<&NumistaType>, indent: usize) {
    if let Some(t) = type_ {
        // Print the required fields first
        println_indented(&format!("id: {}", t.id), indent);
        if let Some(url) = &t.url {
            println_indented(&format!("url: {}", url), indent);
        }
        println_indented(&format!("title: {}", t.title), indent);
        println_indented(&format!("category: {}", t.category), indent);

        print_issuer(t.issuer.as_ref(), indent);
        print_key_value("min_year", t.min_year, indent);
        print_key_value("max_year", t.max_year, indent);
        print_key_value("type_name", t.type_name.as_ref(), indent);

        if let Some(v) = &t.value {
            if let Some(text) = &v.text {
                println_indented(&format!("value: {}", text), indent);
            }
        }

        print_ruling_authorities(t.ruler.as_ref(), indent);
        print_key_value("shape", t.shape.as_ref(), indent);

        if let Some(c) = &t.composition {
            if let Some(text) = &c.text {
                println_indented(&format!("composition: {}", text), indent);
            }
        }

        if let Some(tech) = &t.technique {
            if let Some(text) = &tech.text {
                println_indented(&format!("technique: {}", text), indent);
            }
        }

        print_demonetization(t.demonetization.as_ref(), indent);
        print_key_value("weight", t.weight, indent);
        print_key_value("size", t.size, indent);
        print_key_value("thickness", t.thickness, indent);

        print_coin_side("obverse", t.obverse.as_ref(), indent);
        print_coin_side("reverse", t.reverse.as_ref(), indent);
        print_coin_side("edge", t.edge.as_ref(), indent);

        if let Some(mints) = &t.mints {
            if !mints.is_empty() {
                let mint_names: Vec<String> = mints.iter().map(|m| m.name.clone()).collect();
                println_indented(&format!("mints: {}", mint_names.join(", ")), indent);
            }
        }

        print_key_value("series", t.series.as_ref(), indent);
        print_key_value("commemorated_topic", t.commemorated_topic.as_ref(), indent);
        print_key_value("comments", t.comments.as_ref(), indent);

        if let Some(tags) = &t.tags {
            if !tags.is_empty() {
                println_indented(&format!("tags: {}", tags.join(", ")), indent);
            }
        }

        print_related_types(t.related_types.as_ref(), indent);
        print_references(t.references.as_ref(), indent);
    }
}
