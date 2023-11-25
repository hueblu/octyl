#[derive(Debug, PartialEq, Eq)]
pub struct Tags {
    pub inner: Vec<String>,
}

pub struct Action {
    tags: Tags,
    producer: String,
}

impl Tags {
    pub fn empty() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn from_string<S: ToString>(from: S) -> Self {
        from.to_string()
            .split('.')
            .fold(Tags::empty(), |mut tags, f| {
                tags.inner.push(f.to_string());
                tags
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Tags;

    #[test]
    fn tags_simple() {
        let tag_string = "foo.bar.baz";

        assert_eq!(
            Tags {
                inner: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
            },
            Tags::from_string(tag_string)
        )
    }
}
