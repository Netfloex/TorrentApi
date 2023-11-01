use juniper::GraphQLInputObject;
use qbittorrent_api::AddTorrentOptions;
#[derive(GraphQLInputObject)]
pub struct ApiAddTorrentOptions {
    savepath: Option<String>,
    cookie: Option<String>,
    category: Option<String>,
    tags: Option<String>,
    skip_checking: Option<bool>,
    paused: Option<bool>,
    root_folder: Option<bool>,
    rename: Option<String>,
    up_limit: Option<i32>,
    dl_limit: Option<i32>,
    auto_tmm: Option<bool>,
    sequential_download: Option<bool>,
    first_last_piece_prio: Option<bool>,
}

impl Default for ApiAddTorrentOptions {
    fn default() -> Self {
        Self {
            savepath: None,
            cookie: None,
            category: None,
            tags: None,
            skip_checking: None,
            paused: None,
            root_folder: None,
            rename: None,
            up_limit: None,
            dl_limit: None,
            auto_tmm: None,
            sequential_download: None,
            first_last_piece_prio: None,
        }
    }
}

impl From<ApiAddTorrentOptions> for AddTorrentOptions {
    fn from(value: ApiAddTorrentOptions) -> Self {
        let mut options = AddTorrentOptions::new();

        macro_rules! option_check {
            ($value:expr, $func:ident) => {
                if let Some(val) = $value {
                    options = options.$func(val);
                }
            };
        }

        option_check!(value.savepath, savepath);
        option_check!(value.cookie, cookie);
        option_check!(value.category, category);
        option_check!(value.tags, tags);
        option_check!(value.skip_checking, skip_checking);
        option_check!(value.paused, paused);
        option_check!(value.root_folder, root_folder);
        option_check!(value.rename, rename);
        option_check!(value.up_limit.map(|f| f.into()), up_limit);
        option_check!(value.dl_limit.map(|f| f.into()), dl_limit);
        option_check!(value.auto_tmm, auto_tmm);
        option_check!(value.sequential_download, sequential_download);
        option_check!(value.first_last_piece_prio, first_last_piece_prio);

        options
    }
}