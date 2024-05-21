const AUTH_PATH: &str = "/auth";
const USERS_PATH: &str = "/users";
const GROUP_PATH: &str = "/group";
const CHAT_PATH: &str = "/chat";

pub enum RoutePath {
    AUTH,
    USERS,
    GROUP,
    CHAT,
}

impl RoutePath {
    pub fn get_path(&self) -> &'static str {
        match self {
            RoutePath::AUTH => AUTH_PATH,
            RoutePath::USERS => USERS_PATH,
            RoutePath::GROUP => GROUP_PATH,
            RoutePath::CHAT => CHAT_PATH,
        }
    }
}
