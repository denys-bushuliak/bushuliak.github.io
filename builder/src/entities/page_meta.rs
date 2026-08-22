#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageMeta {
    pub title: String,
    pub description: String,
}

impl PageMeta {
    /// Derives the per-page `<title>` and meta description from a markdown file stem.
    ///
    /// `index` gets the site's main title; any other known page gets a
    /// hand-written description, and unknown stems fall back to a humanized
    /// name combined with the site name (used as both title and description).
    pub fn from_file_stem(stem: &str) -> Self {
        let (title, description) = match stem {
            "index" => (
                "Denys Bushuliak | Principal Software Engineer".to_string(),
                "Principal Software Engineer with 20+ years of experience in distributed systems, Rust, and software architecture."
                    .to_string(),
            ),
            "about" => (
                "About Me | Denys Bushuliak".to_string(),
                "Background, education, languages, interests, and volunteering.".to_string(),
            ),
            "projects" => (
                "Projects | Denys Bushuliak".to_string(),
                "Selected projects: an online education platform, ERP systems, a taxi platform, and SafeFrontier."
                    .to_string(),
            ),
            "skills" => (
                "Skills | Denys Bushuliak".to_string(),
                "Technical skills: Rust, Go, JavaScript, CQRS, microservices, DDD, databases, and cloud."
                    .to_string(),
            ),
            "recommendation_letters" => (
                "Recommendation Letters | Denys Bushuliak".to_string(),
                "Letters of recommendation from past employers, with downloadable PDFs.".to_string(),
            ),
            other => {
                let title = format!("{} | Denys Bushuliak", humanize(other));
                (title.clone(), title)
            }
        };

        Self { title, description }
    }
}

fn humanize(stem: &str) -> String {
    stem.split('_')
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    let rest: String = chars.collect();
                    upper + &rest
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
