pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut buf: Vec<String> = Vec::new();
        
        let mut tmp = String::new();
        for c in path.chars() {
            if c=='/' {
                match &tmp[..] {
                    "" | "." => {},
                    ".." => {buf.pop();},
                    _ => {buf.push(tmp.to_string());}
                }
                tmp.clear();
            } else {
                tmp.push(c);
            }
        }
        
        match &tmp[..] {
            "" | "." => {},
            ".." => {buf.pop();},
            _ => {buf.push(tmp.to_string());}
        }
        
        format!("/{path}", path=buf.join("/"))
    }
}