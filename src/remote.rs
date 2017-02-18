use url::Url;

pub struct Remote {
  url: Url,
}

impl Remote {
  pub fn from_url(url: Url) -> Remote {
    Remote { url: url }
  }

  pub fn url(&self) -> &Url {
    &self.url
  }
}
