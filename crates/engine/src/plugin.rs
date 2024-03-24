use serde::{Deserialize, Serialize};

// TODO: create necessary structs for outputs
/// Plugin Trait - necessary methods that any struct must implement
/// in order to be considered a plugin
pub trait Plugin {
    /// Name of the plugin
    fn name() -> String;

    /// Plugin metadate
    fn metadate() -> MetaData;

    /// Search based on a query
    fn search_manga(query: String) -> ();

    /// Return manga with `id`
    fn manga(id: String) -> ();

    /// Return images of `chapter` from manga with`id`
    fn chapter_pages(id: String, chapter: String) -> ();
}

/// Metadata of a series
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MetaData {
    /// Name of the provider this plugin uses.
    ///
    /// This must be unique for each plugin as it serves as a sort of id
    /// used to differentiate between plugins.
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Description of the plugin (optional)
    pub description: Option<String>,
    /// Url to the homepage of the site/provider
    pub homepage: String,
    /// Wether this plugin primarily features adult contents
    pub is_nsfw: bool,
}
