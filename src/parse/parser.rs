pub fn parse_for_content(content: &String) -> String {
    let mut res = String::new();
    let mut is_str = false;
    let mut is_tag = false;
    let mut tag = String::new();
    for c in content.chars() {
        if c == '\"' { if is_str { is_str = false; } else { is_str = true; } }
        if !is_str { if c == '<' {
            tag.clear();
            is_tag = true;
        } }
        if is_tag {
            tag.push(c);
        } else { res.push(c); }
        if !is_str { if c == '>' {
            res.push_str(convert_tag_str(&tag).as_str());
            is_tag = false;
        } }
    }
    String::from(res)
}

fn convert_tag_str(tag_str: &String) -> String {
    let tag_str = String::from(&tag_str[1..tag_str.len()-1]);
    let mut is_tsg = true;
    let mut tag_name = String::new();
    let mut params = String::new();
    let mut children = String::new();
    let mut is_str = false;
    let mut is_child = false;
    for c in tag_str.chars() {
        if c == ' ' { is_tsg = false }
        if is_tsg { tag_name.push(c); }
        else {
            if c == '\"' {
                if is_str { is_str = false; } else { is_str = false; }
            }
            if !is_str { if c == '{' { is_child = true; } }
            if is_child { children.push(c); } else { params.push(c); }
            if !is_str { if c == '}' { is_child = false; } }
        }
    }
    let params = params.trim();
    let children = String::from(&children[1..children.len()-1]);
    format!("<{} {}>{}</{}>", &tag_name, &params, &children, &tag_name)
}