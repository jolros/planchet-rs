use planchet::model::{
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
        if let Some(engravers) = &s.engravers {
            if !engravers.is_empty() {
                print_key_value("engravers", Some(engravers.join(", ")), next_indent);
            }
        }
        if let Some(designers) = &s.designers {
            if !designers.is_empty() {
                print_key_value("designers", Some(designers.join(", ")), next_indent);
            }
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
        print_key_value("is_demonetized", Some(d.is_demonetized), next_indent);
        if let Some(date) = d.demonetization_date {
            print_key_value("demonetization_date", Some(date.format("%Y-%m-%d").to_string()), next_indent);
        }
    }
}

fn print_issuer(label: &str, issuer: Option<&Issuer>, indent: usize) {
    if let Some(i) = issuer {
        print_indented(&format!("{}:", label.replace('_', " ")), indent);
        let next_indent = indent + 2;
        print_key_value("code", Some(i.code.clone()), next_indent);
        print_key_value("name", Some(i.name.clone()), next_indent);
    }
}

fn print_issuing_entity(label: &str, entity: Option<&IssuingEntity>, indent: usize) {
    if let Some(e) = entity {
        print_indented(&format!("{}:", label.replace('_', " ")), indent);
        let next_indent = indent + 2;
        print_key_value("name", Some(e.name.clone()), next_indent);
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
        print_key_value("id", Some(t.id), indent);
        print_key_value("url", t.url.as_ref().map(|u| u.to_string()), indent);
        print_key_value("title", Some(t.title.clone()), indent);
        print_key_value("category", Some(t.category.to_string()), indent);

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
            print_key_value("value", v.text.as_ref(), indent);
        }

        print_ruling_authorities("ruling_authorities", t.ruler.as_ref(), indent);
        print_key_value("shape", t.shape.as_ref(), indent);

        if let Some(c) = &t.composition {
            print_key_value("composition", c.text.as_ref(), indent);
        }

        if let Some(tech) = &t.technique {
            print_key_value("technique", tech.text.as_ref(), indent);
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
                print_key_value("mints", Some(mint_names.join(", ")), indent);
            }
        }

        print_printers("printers", t.printers.as_ref(), indent);
        print_key_value("series", t.series.as_ref(), indent);
        print_key_value("commemorated_topic", t.commemorated_topic.as_ref(), indent);
        print_key_value("comments", t.comments.as_ref(), indent);

        if let Some(tags) = &t.tags {
            if !tags.is_empty() {
                print_key_value("tags", Some(tags.join(", ")), indent);
            }
        }

        print_related_types("related_types", t.related_types.as_ref(), indent);
        print_references("references", t.references.as_ref(), indent);
    }
}
