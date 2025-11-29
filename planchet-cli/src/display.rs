use planchet::models::{Category, CoinSide, Demonetization, Issuer, NumistaType, Reference, RelatedType, RulingAuthority};
use url::Url;

fn print_key_value<T: std::fmt::Display>(key: &str, value: Option<T>, indent: usize) {
    if let Some(v) = value {
        println!("{:indent$}{}: {}", "", key, v, indent = indent);
    }
}

fn print_coin_side(name: &str, side: Option<&CoinSide>, indent: usize) {
    if let Some(s) = side {
        println!("{:indent$}{}:", "", name, indent = indent);
        let next_indent = indent + 2;
        if !s.engravers.is_empty() {
            println!("{:indent$}engravers: {}", "", s.engravers.join(", "), indent = next_indent);
        }
        if !s.designers.is_empty() {
            println!("{:indent$}designers: {}", "", s.designers.join(", "), indent = next_indent);
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
        println!("{:indent$}demonetization:", "", indent = indent);
        let next_indent = indent + 2;
        println!("{:indent$}is_demonetized: {}", "", d.is_demonetized, indent = next_indent);
        if let Some(date) = d.demonetization_date {
            println!("{:indent$}demonetization_date: {}", "", date.format("%Y-%m-%d"), indent = next_indent);
        }
    }
}

fn print_issuer(issuer: Option<&Issuer>, indent: usize) {
    if let Some(i) = issuer {
        println!("{:indent$}issuer:", "", indent = indent);
        let next_indent = indent + 2;
        println!("{:indent$}code: {}", "", i.code, indent = next_indent);
        println!("{:indent$}name: {}", "", i.name, indent = next_indent);
    }
}

fn print_ruling_authorities(rulers: Option<&Vec<RulingAuthority>>, indent: usize) {
    if let Some(r) = rulers {
        if !r.is_empty() {
            println!("{:indent$}ruler:", "", indent = indent);
            let next_indent = indent + 2;
            for ruler in r {
                println!("{:indent$}- {}", "", ruler.name, indent = next_indent);
            }
        }
    }
}

fn print_references(references: Option<&Vec<Reference>>, indent: usize) {
    if let Some(r) = references {
        if !r.is_empty() {
            println!("{:indent$}references:", "", indent = indent);
            let next_indent = indent + 2;
            for reference in r {
                println!("{:indent$}- {}: {}", "", reference.catalogue.code, reference.number, indent = next_indent);
            }
        }
    }
}

fn print_related_types(related_types: Option<&Vec<RelatedType>>, indent: usize) {
    if let Some(r) = related_types {
        if !r.is_empty() {
            println!("{:indent$}related_types:", "", indent = indent);
            let next_indent = indent + 2;
            for type_ in r {
                 println!("{:indent$}- [{}] {}", "", type_.id, type_.title, indent = next_indent);
            }
        }
    }
}

pub fn show_type(type_: &NumistaType) {
    // Print the required fields first
    println!("id: {}", type_.id);
    if let Some(url) = &type_.url {
        println!("url: {}", url);
    }
    println!("title: {}", type_.title);
    println!("category: {}", type_.category);

    let indent = 0;
    print_issuer(type_.issuer.as_ref(), indent);
    print_key_value("min_year", type_.min_year, indent);
    print_key_value("max_year", type_.max_year, indent);
    print_key_value("type_name", type_.type_name.as_ref(), indent);

    if let Some(v) = &type_.value {
        if let Some(text) = &v.text {
            println!("value: {}", text);
        }
    }

    print_ruling_authorities(type_.ruler.as_ref(), indent);
    print_key_value("shape", type_.shape.as_ref(), indent);

    if let Some(c) = &type_.composition {
        if let Some(text) = &c.text {
            println!("composition: {}", text);
        }
    }

    if let Some(t) = &type_.technique {
        if let Some(text) = &t.text {
            println!("technique: {}", text);
        }
    }

    print_demonetization(type_.demonetization.as_ref(), indent);
    print_key_value("weight", type_.weight, indent);
    print_key_value("size", type_.size, indent);
    print_key_value("thickness", type_.thickness, indent);

    print_coin_side("obverse", type_.obverse.as_ref(), indent);
    print_coin_side("reverse", type_.reverse.as_ref(), indent);
    print_coin_side("edge", type_.edge.as_ref(), indent);

    if let Some(mints) = &type_.mints {
        if !mints.is_empty() {
            let mint_names: Vec<String> = mints.iter().map(|m| m.name.clone()).collect();
            println!("mints: {}", mint_names.join(", "));
        }
    }

    print_key_value("series", type_.series.as_ref(), indent);
    print_key_value("commemorated_topic", type_.commemorated_topic.as_ref(), indent);
    print_key_value("comments", type_.comments.as_ref(), indent);

    if let Some(tags) = &type_.tags {
        if !tags.is_empty() {
            println!("tags: {}", tags.join(", "));
        }
    }

    print_related_types(type_.related_types.as_ref(), indent);
    print_references(type_.references.as_ref(), indent);
}
