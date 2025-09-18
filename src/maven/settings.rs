use std::{fs, path::PathBuf, str::FromStr};

use crate::{
    maven::{
        CUSTOM_SETTINGS_LOCATION, HOME, MAVEN_HOME,
        common_model::{Repository, get_repositories},
    },
    xml::dom_parser::{Node, get_document},
};

pub fn get_settings() -> Result<Settings, String> {
    let settings_path = get_settings_path().map_err(|e| e.to_string())?;
    get_settings_from_path(settings_path)
}

pub fn get_settings_from_path(settings_path: PathBuf) -> Result<Settings, String> {
    let settings = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    get_settings_from_string(settings)
}

pub fn get_settings_from_string(settings: String) -> Result<Settings, String> {
    let mut local_repository = None;
    let mut interactive_mode = true;
    let mut use_plugin_registry = false;
    let mut offline = false;
    let mut proxies = vec![];
    let mut servers = vec![];
    let mut mirrors = vec![];
    let mut profiles = vec![];
    let mut active_profiles = vec![];
    let mut plugin_groups = vec![];

    let root = get_document(settings).map_err(|err| err.to_string())?.root;
    for child in root.children {
        match child.name.as_str() {
            "localRepository" => local_repository = child.text,
            "interactiveMode" => interactive_mode = child.text.map(|b| b == "true").unwrap_or(true),
            "usePluginRegistry" => {
                use_plugin_registry = child.text.map(|b| b == "true").unwrap_or(false)
            }
            "offline" => offline = child.text.map(|b| b == "true").unwrap_or(false),
            "proxies" => proxies = get_proxies(child),
            "servers" => servers = get_servers(child),
            "mirrors" => mirrors = get_mirrors(child),
            "profiles" => profiles = get_profiles(child),
            "activeProfiles" => active_profiles = get_active_profiles(child),
            "pluginGroups" => plugin_groups = get_plugin_groups(child),
            _ => {}
        };
    }

    Ok(Settings {
        local_repository,
        interactive_mode,
        use_plugin_registry,
        offline,
        proxies,
        servers,
        mirrors,
        profiles,
        active_profiles,
        plugin_groups,
    })
}

fn get_proxies(element: Node) -> Vec<Proxy> {
    let mut proxies = vec![];
    for child in element.children {
        proxies.push(get_proxy(child));
    }
    proxies
}

fn get_active_profiles(element: Node) -> Vec<String> {
    let mut active_profiles = vec![];
    for child in element.children {
        if let Some(active_profile) = child.text {
            active_profiles.push(active_profile);
        }
    }
    active_profiles
}

fn get_plugin_groups(element: Node) -> Vec<String> {
    let mut plugin_groups = vec![];
    for child in element.children {
        if let Some(plugin_group) = child.text {
            plugin_groups.push(plugin_group);
        }
    }
    plugin_groups
}

fn get_servers(servers_element: Node) -> Vec<Server> {
    let mut servers = vec![];
    for server_element in servers_element.children {
        servers.push(get_server(server_element));
    }
    servers
}

fn get_mirrors(mirrors_element: Node) -> Vec<Mirror> {
    let mut mirrors = vec![];
    for mirror_element in mirrors_element.children {
        mirrors.push(get_mirror(mirror_element));
    }
    mirrors
}

fn get_profiles(profiles_element: Node) -> Vec<Profile> {
    let mut profiles = vec![];
    for mirror_element in profiles_element.children {
        profiles.push(get_profile(mirror_element));
    }
    profiles
}

fn get_server(server_element: Node) -> Server {
    let mut id = None;
    let mut username = None;
    let mut password = None;
    let mut private_key = None;
    let mut passphrase = None;
    let mut file_permissions = None;
    let mut directory_permissions = None;
    let mut configuration = None;
    for child in server_element.children {
        match child.name.as_str() {
            "id" => id = child.text,
            "username" => username = child.text,
            "password" => password = child.text,
            "private_key" => private_key = child.text,
            "passphrase" => passphrase = child.text,
            "filePermissions" => file_permissions = child.text,
            "directoryPermissions" => directory_permissions = child.text,
            "configuration" => configuration = Some(child),
            _ => {}
        }
    }
    Server {
        id,
        username,
        password,
        private_key,
        passphrase,
        file_permissions,
        directory_permissions,
        configuration,
    }
}

fn get_proxy(element: Node) -> Proxy {
    let mut active = false;
    let mut protocol = "http".to_owned();
    let mut username = None;
    let mut password = None;
    let mut port: usize = 8080;
    let mut host = None;
    let mut non_proxy_hosts = None;
    let mut id = None;

    for child in element.children {
        match child.name.as_str() {
            "active" => active = child.text.map(|b| b == "true").unwrap_or(false),
            "protocol" => protocol = child.text.unwrap_or("http".to_owned()),
            "username" => username = child.text,
            "password" => password = child.text,
            "port" => {
                port = child
                    .text
                    .map(|i| {
                        usize::from_str(&i).expect(&format!("Illegal value for port: '{}'", i))
                    })
                    .unwrap_or(8080)
            }
            "host" => host = child.text,
            "non_proxy_hosts" => non_proxy_hosts = child.text,
            "id" => id = child.text,
            _ => {}
        }
    }

    Proxy {
        active,
        protocol,
        username,
        password,
        port,
        host,
        non_proxy_hosts,
        id,
    }
}

fn get_mirror(mirror_element: Node) -> Mirror {
    let mut id = None;
    let mut mirror_of = None;
    let mut url = None;
    let mut name = None;
    for child in mirror_element.children {
        match child.name.as_str() {
            "id" => id = child.text,
            "mirror_of" => mirror_of = child.text,
            "url" => url = child.text,
            "name" => name = child.text,
            _ => {}
        }
    }

    Mirror {
        id,
        mirror_of,
        url,
        name,
    }
}

fn get_profile(profile_element: Node) -> Profile {
    let mut id = None;
    let mut activation = None;
    let mut properties = vec![];
    let mut repositories = vec![];
    let mut plugin_repositories = vec![];

    for child in profile_element.children {
        match child.name.as_str() {
            "id" => id = child.text,
            "activation" => activation = Some(get_activation(child)),
            "properties" => properties.append(&mut get_properties(child)),
            "repositories" => repositories = get_repositories(child),
            "pluginRepositories" => plugin_repositories = get_repositories(child),
            _ => {}
        }
    }

    Profile {
        id,
        activation,
        properties,
        repositories,
        plugin_repositories,
    }
}

fn get_activation(activation_element: Node) -> Activation {
    let mut active_by_default = false;
    let mut jdk = None;
    let mut os = None;
    let mut property = None;
    let mut file = None;
    for child in activation_element.children {
        match child.name.as_str() {
            "activeByDefault" => {
                active_by_default = child.text.map(|b| b == "true").unwrap_or(false)
            }
            "jdk" => jdk = child.text,
            "os" => os = Some(get_activation_os(child)),
            "property" => property = Some(get_activation_property(child)),
            "file" => file = Some(get_activation_file(child)),
            _ => {}
        }
    }

    Activation {
        active_by_default,
        jdk,
        os,
        property,
        file,
    }
}

fn get_properties(element: Node) -> Vec<Property> {
    let mut properties = vec![];
    for child in element.children {
        properties.push(Property {
            name: child.name,
            value: child.text,
        });
    }
    properties
}

fn get_activation_os(element: Node) -> ActivationOs {
    let mut name = None;
    let mut family = None;
    let mut arch = None;
    let mut version = None;
    for child in element.children {
        match child.name.as_str() {
            "name" => name = child.text,
            "family" => family = child.text,
            "arch" => arch = child.text,
            "version" => version = child.text,
            _ => {}
        }
    }

    ActivationOs {
        name,
        family,
        arch,
        version,
    }
}

fn get_activation_property(element: Node) -> ActivationProperty {
    let mut name = None;
    let mut value = None;
    for child in element.children {
        match child.name.as_str() {
            "name" => name = child.text,
            "value" => value = child.text,
            _ => {}
        }
    }

    ActivationProperty { name, value }
}

fn get_activation_file(element: Node) -> ActivationFile {
    let mut missing = None;
    let mut exists = None;
    for child in element.children {
        match child.name.as_str() {
            "missing" => missing = child.text,
            "exists" => exists = child.text,
            _ => {}
        }
    }

    ActivationFile { missing, exists }
}

fn get_settings_path() -> Result<PathBuf, String> {
    let mut settings = PathBuf::from_str(HOME.as_str()).map_err(|e| e.to_string())?;
    settings.push(".m2/settings.xml");
    if !settings.exists() {
        settings = PathBuf::from_str(MAVEN_HOME.as_str()).map_err(|e| e.to_string())?;
        settings.push("conf/settings.xml");
    }
    if !settings.exists() {
        settings =
            PathBuf::from_str(CUSTOM_SETTINGS_LOCATION.as_str()).map_err(|e| e.to_string())?;
        if settings.is_dir() {
            settings.push("settings.xml");
        }
    }
    Ok(settings)
}

impl Settings {
    pub fn get_active_profiles(&self) -> Vec<&Profile> {
        self.profiles
            .iter()
            .filter(|p| {
                if let Some(activation) = &p.activation {
                    activation.active_by_default //TODO other activation types are possible
                } else if let Some(id) = &p.id {
                    self.active_profiles.contains(id)
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn get_repositories(&self) -> Vec<Repository> {
        self.get_active_profiles()
            .iter()
            .map(|p| &p.repositories)
            .flatten()
            .cloned()
            .collect()
    }

    pub fn get_plugin_repositories(&self) -> Vec<Repository> {
        self.get_active_profiles()
            .iter()
            .map(|p| &p.plugin_repositories)
            .flatten()
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
pub struct Settings {
    pub local_repository: Option<String>,
    pub interactive_mode: bool,
    pub use_plugin_registry: bool,
    pub offline: bool,
    pub proxies: Vec<Proxy>,
    pub servers: Vec<Server>,
    pub mirrors: Vec<Mirror>,
    pub profiles: Vec<Profile>,
    pub active_profiles: Vec<String>,
    pub plugin_groups: Vec<String>,
}

#[derive(Debug)]
pub struct Server {
    pub id: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub private_key: Option<String>,
    pub passphrase: Option<String>,
    pub file_permissions: Option<String>,
    pub directory_permissions: Option<String>,
    pub configuration: Option<Node>, //xsd:any
}

#[derive(Debug)]
pub struct Mirror {
    pub id: Option<String>,
    pub mirror_of: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug)]
pub struct Proxy {
    pub active: bool,
    pub protocol: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub port: usize,
    pub host: Option<String>,
    pub non_proxy_hosts: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug)]
pub struct Profile {
    pub id: Option<String>,
    pub activation: Option<Activation>,
    pub properties: Vec<Property>,
    pub repositories: Vec<Repository>,
    pub plugin_repositories: Vec<Repository>,
}

#[derive(Debug)]
pub struct Activation {
    pub active_by_default: bool,
    pub jdk: Option<String>,
    pub os: Option<ActivationOs>,
    pub property: Option<ActivationProperty>,
    pub file: Option<ActivationFile>,
}

#[derive(Debug)]
pub struct ActivationOs {
    pub name: Option<String>,
    pub family: Option<String>,
    pub arch: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug)]
pub struct ActivationProperty {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct ActivationFile {
    pub missing: Option<String>,
    pub exists: Option<String>,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: Option<String>,
}
