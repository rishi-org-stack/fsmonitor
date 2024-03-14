pub enum Method<'a> {
    Contains(&'a str),
    HasExt(&'a str),
    StartsWith(&'a str),
    EndsWith(&'a str),
}

impl<'a> Method<'a> {
    pub fn matches(&self, case: &str) -> bool {
        match self {
            Method::Contains(input) => case.contains(*input),
            Method::HasExt(s) => {
                let case_broken: Vec<&str> = case.split_terminator(".").collect();
                if case_broken.contains(&"part") {
                    return false;
                }
                let ext_found = *case_broken.last().unwrap();
                ext_found.eq(*s)
            }
            Method::StartsWith(input) => case.starts_with(*input),
            Method::EndsWith(input) => case.ends_with(*input),
        }
    }
}
