pub struct URLBuilder {
    base_url: String,
    queries: Vec<(String, String)>,
}

impl<'a> URLBuilder {
    pub fn new(base_url: String) -> URLBuilder {
        URLBuilder {
            base_url,
            queries: vec![],
        }
    }

    pub fn add_queries(&mut self, queries: &mut Vec<(String, String)>) -> &mut Self {
        self.queries.append(queries);
        self
    }

    pub fn _join_url(&mut self, urls: Vec<&'a str>) -> &mut Self {
        let urls_len = urls.len();
        let mut url_strs: Vec<&'a str> = Vec::with_capacity(urls_len);
        for url in urls {
            let start_pos = if url.starts_with("/") { 1 } else { 0 };
            let end_pos = url.len();
            let end_pos = if url.ends_with("/") {
                end_pos - 1
            } else {
                end_pos
            };
            url_strs.push(url[start_pos..end_pos].into());
        }
        let join_url = url_strs.join("/");
        self.base_url = self.base_url.to_string() + &join_url;
        self
    }

    pub fn to_string(&self) -> String {
        let queries_len = self.queries.len();
        let mut query_strs: Vec<String> = Vec::with_capacity(queries_len);
        let query_iter = self.queries.iter();
        for (key, value) in query_iter {
            query_strs.push(format!("{}={}", key, value));
        }

        format!("{}?{}", self.base_url, query_strs.join("&"))
    }
}
