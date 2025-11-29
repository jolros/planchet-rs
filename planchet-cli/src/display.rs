use planchet::models::{
    CoinSide, Demonetization, Issuer, IssuingEntity, NumistaType, Printer, Reference, RelatedType,
    RulingAuthority,
};
use url::Url;

// Centralized printing function.
fn print_indented(msg: &str, indent: usize) {
    println!("{:indent$}{}", "", msg, indent = indent);
}

fn print_key_value<T: std::fmt::Display>(key: &str, value: Option<T>, indent: usize) {
    if let Some(v) = value {
        print_indented(&format!("{}: {}", key.replace('_', " "), v), indent);
    }
}

fn print_coin_side(label: &str, side: Option<&CoinSide>, indent: usize) {
    if let Some(s) = side {
        print_indented(&format!("{}:", label.replace('_', " ")), indent);
        let next_indent = indent + 2;
        if !s.engravers.is_empty() {
            print_indented(&format!("engravers: {}", s.engravers.join(", ")), next_indent);
        }
        if !s.designers.is_empty() {
            print_indented(&format!("designers: {}", s.designers.join(", ")), next_indent);
        }
        print_key_value("description", s.description.as_ref(), next_indent);
        print_key_value("lettering", s.lettering.as_ref(), next_indent);
        print_key_value("unabridged_legend", s.unabridged_legend.as_ref(), next_indent);
        print_key_value("lettering_translation", s.lettering_translation.as_ref(), next_indent);
        print_key_value("picture", s.picture.as_ref().map(Url::as_str), next_indent);
    }
}

fn print_demonetization(label: &str, demonetization: Option<&Demonetization>, indent: usize) {
    if let Some(d) = demonetization {
        print_indented(&format!("{}:", label.replace('_', " ")), indent);
        let next_indent = indent + 2;
        print_indented(&format!("is demonetized: {}", d.is_demonetized), next_indent);
        if let Some(date) = d.demonetization_date {
            print_indented(
                &format!("demonetization date: {}", date.format("%Y-%m-%d")),
                next_indent,
            );
        }
    }
}

fn print_issuer(label: &str, issuer: Option<&Issuer>, indent: usize) {
    if let Some(i) = issuer {
        print_indented(&format!("{}:", label.replace('_', " ")), indent);
        let next_indent = indent + 2;
        print_indented(&format!("code: {}", i.code), next_indent);
        print_indented(&format!("name: {}", i.name), next_indent);
    }
}

fn print_issuing_entity(label: &str, entity: Option<&IssuingEntity>, indent: usize) {
    if let Some(e) = entity {
        print_indented(&format!("{}:", label.replace('_', " ")), indent);
        let next_indent = indent + 2;
        print_indented(&format!("name: {}", e.name), next_indent);
    }
}

fn print_ruling_authorities(label: &str, authorities: Option<&Vec<RulingAuthority>>, indent: usize) {
    if let Some(a) = authorities {
        if !a.is_empty() {
            print_indented(&format!("{}:", label.replace('_', " ")), indent);
            let next_indent = indent + 2;
            for authority in a {
                print_indented(&format!("- {}", authority.name), next_indent);
            }
        }
    }
}

fn print_references(label: &str, references: Option<&Vec<Reference>>, indent: usize) {
    if let Some(r) = references {
        if !r.is_empty() {
            print_indented(&format!("{}:", label.replace('_', " ")), indent);
            let next_indent = indent + 2;
            for reference in r {
                print_indented(
                    &format!("- {}: {}", reference.catalogue.code, reference.number),
                    next_indent,
                );
            }
        }
    }
}

fn print_related_types(label: &str, related_types: Option<&Vec<RelatedType>>, indent: usize) {
    if let Some(r) = related_types {
        if !r.is_empty() {
            print_indented(&format!("{}:", label.replace('_', " ")), indent);
            let next_indent = indent + 2;
            for type_ in r {
                print_indented(&format!("- [{}] {}", type_.id, type_.title), next_indent);
            }
        }
    }
}

fn print_printers(label: &str, printers: Option<&Vec<Printer>>, indent: usize) {
    if let Some(p) = printers {
        if !p.is_empty() {
            print_indented(&format!("{}:", label.replace('_', " ")), indent);
            let next_indent = indent + 2;
            for printer in p {
                print_indented(&format!("- {}", printer.name), next_indent);
            }
        }
    }
}

pub fn print_numista_type(type_: Option<&NumistaType>, indent: usize) {
    if let Some(t) = type_ {
        // Print the required fields first
        print_indented(&format!("id: {}", t.id), indent);
        if let Some(url) = &t.url {
            print_indented(&format!("url: {}", url), indent);
        }
        print_indented(&format!("title: {}", t.title), indent);
        print_indented(&format!("category: {}", t.category), indent);

        print_issuer("issuer", t.issuer.as_ref(), indent);
        print_issuing_entity("issuing_entity", t.issuing_entity.as_ref(), indent);
        print_issuing_entity(
            "secondary_issuing_entity",
            t.secondary_issuing_entity.as_ref(),
            indent,
        );
        print_key_value("min_year", t.min_year, indent);
        print_key_value("max_year", t.max_year, indent);
        print_key_value("type_name", t.type_name.as_ref(), indent);

        if let Some(v) = &t.value {
            if let Some(text) = &v.text {
                print_indented(&format!("value: {}", text), indent);
            }
        }

        print_ruling_authorities("ruling_authorities", t.ruler.as_ref(), indent);
        print_key_value("shape", t.shape.as_ref(), indent);

        if let Some(c) = &t.composition {
            if let Some(text) = &c.text {
                print_indented(&format!("composition: {}", text), indent);
            }
        }

        if let Some(tech) = &t.technique {
            if let Some(text) = &tech.text {
                print_indented(&format!("technique: {}", text), indent);
            }
        }

        print_demonetization("demonetization", t.demonetization.as_ref(), indent);
        print_key_value("weight", t.weight, indent);
        print_key_value("size", t.size, indent);
        print_key_value("size2", t.size2, indent);
        print_key_value("thickness", t.thickness, indent);

        print_coin_side("obverse", t.obverse.as_ref(), indent);
        print_coin_side("reverse", t.reverse.as_ref(), indent);
        print_coin_side("edge", t.edge.as_ref(), indent);
        print_coin_side("watermark", t.watermark.as_ref(), indent);

        if let Some(mints) = &t.mints {
            if !mints.is_empty() {
                let mint_names: Vec<String> = mints.iter().map(|m| m.name.clone()).collect();
                print_indented(&format!("mints: {}", mint_names.join(", ")), indent);
            }
        }

        print_printers("printers", t.printers.as_ref(), indent);
        print_key_value("series", t.series.as_ref(), indent);
        print_key_value("commemorated_topic", t.commemorated_topic.as_ref(), indent);
        print_key_value("comments", t.comments.as_ref(), indent);

        if let Some(tags) = &t.tags {
            if !tags.is_empty() {
                print_indented(&format!("tags: {}", tags.join(", ")), indent);
            }
        }

        print_related_types("related_types", t.related_types.as_ref(), indent);
        print_references("references", t.references.as_ref(), indent);
    }
}
