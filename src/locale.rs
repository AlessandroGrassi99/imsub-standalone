use fluent::{bundle::FluentBundle, FluentResource};
use fluent_bundle::FluentArgs;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, sync::Arc};
use teloxide::prelude::Message;
use thiserror::Error;
use tokio::fs::ReadDir;

use unic_langid::LanguageIdentifier;

#[derive(Deserialize, Serialize, Clone, Copy, Eq, Hash, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Locale {
    En,
    It,
}

impl TryFrom<&str> for Locale {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "en" => Ok(Self::En),
            "it" => Ok(Self::It),
            _ => Err(Error::Locale),
        }
    }
}

impl Into<LanguageIdentifier> for Locale {
    fn into(self) -> LanguageIdentifier {
        match self {
            Locale::En => "en".parse().unwrap(),
            Locale::It => "it".parse().unwrap(),
        }
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::En
    }
}

type FluentBundleSafe = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

#[derive(Clone)]
pub struct LocaleManager {
    bundles: Arc<HashMap<(Locale, String), FluentBundleSafe>>,
    pub(crate) local_locale: Locale,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid locale")]
    Locale,
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("fluent error")]
    Fluent,
    #[error("other error")]
    Other,
}

impl LocaleManager {
    pub async fn new(locale_path: &str, default_locale: &str) -> Result<Self, Error> {
        let locale_dir = tokio::fs::read_dir(Path::new(locale_path)).await?;
        let bundles = Self::load_locale_local_database(locale_dir).await?;

        Ok(Self {
            bundles: Arc::new(bundles),
            local_locale: Locale::try_from(default_locale)?,
        })
    }

    async fn load_locale_local_database(
        mut locale_dir: ReadDir,
    ) -> Result<HashMap<(Locale, String), FluentBundleSafe>, Error> {
        let mut bundles: HashMap<(Locale, String), FluentBundleSafe> = HashMap::new();

        while let Some(locale_entry) = locale_dir.next_entry().await? {
            if let Ok(locale_meta) = locale_entry.metadata().await {
                if !locale_meta.is_dir() {
                    continue;
                }

                let locale_name = match locale_entry.file_name().into_string() {
                    Ok(name) => name,
                    Err(_) => continue,
                };
                let locale = match Locale::try_from(locale_name.as_str()) {
                    Ok(locale) => locale,
                    Err(_) => continue,
                };

                let locale_res_iter = match tokio::fs::read_dir(locale_entry.path()).await {
                    Ok(value) => value,
                    Err(_) => continue,
                };

                let locale_bundles = Self::build_locale_res(locale, locale_res_iter).await?;
                for (res_name, bundle) in locale_bundles {
                    bundles.insert((locale, res_name), bundle);
                }
            }
        }
        Ok(bundles)
    }

    async fn build_locale_res(
        locale: Locale,
        mut locale_res_dir: ReadDir,
    ) -> Result<Vec<(String, FluentBundleSafe)>, Error> {
        let mut locale_res = Vec::new();
        while let Some(locale_res_entry) = locale_res_dir.next_entry().await? {
            if locale_res_entry
                .path()
                .extension()
                .ok_or(Error::Other)?
                .to_str()
                != Some("ftl")
            {
                continue;
            }
            let locale_res_file_name = match locale_res_entry
                .path()
                .file_stem()
                .ok_or(Error::Other)?
                .to_str()
            {
                Some(file_name) => file_name.to_string(),
                None => continue,
            };

            let mut locale_bundle = FluentBundleSafe::new_concurrent(vec![locale.into()]);
            let res_str: String = tokio::fs::read_to_string(locale_res_entry.path()).await?;
            let fluent_res = FluentResource::try_new(res_str).map_err(|_| Error::Fluent)?;
            locale_bundle
                .add_resource(fluent_res)
                .map_err(|_| Error::Fluent)?;

            locale_res.push((locale_res_file_name, locale_bundle));
        }
        Ok(locale_res)
    }

    /// Load the language of the current conversation, if the Locale not exists the default value is english.
    pub async fn set_chat_locale(&mut self, chat_locale: Locale) {
        self.local_locale = chat_locale;
    }

    pub(crate) fn set_chat_locale_from_message(&mut self, message: &Message) {
        self.local_locale = Self::get_language(message);
    }

    pub(crate) fn get_message(
        &self,
        res: &str,
        id: &str,
        args: Option<Vec<(&str, &str)>>,
    ) -> Option<String> {
        let bundle = self.get_local_bundle(res)?;
        let mut message = None;

        if let Some(value) = bundle.get_message(id)?.value() {
            let mut error = Vec::new();
            let mut fluent_args_opt = None;
            let mut fluent_args;
            if let Some(args) = args {
                fluent_args = FluentArgs::new();
                args.into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .for_each(|(k, v)| fluent_args.set(k, v));
                fluent_args_opt = Some(&fluent_args);
            }
            let format_res = bundle
                .format_pattern(value, fluent_args_opt, &mut error)
                .to_string();
            if error.is_empty() {
                message = Some(format_res);
            }
        }

        message
    }

    fn get_local_bundle(&self, res: &str) -> Option<&FluentBundleSafe> {
        self.get_bundle(self.local_locale, res)
    }

    fn get_bundle(&self, locale: Locale, res: &str) -> Option<&FluentBundleSafe> {
        self.bundles.get(&(locale, res.to_string()))
    }

    fn get_language(message: &Message) -> Locale {
        let mut locale = Locale::default();
        if let Some(user) = message.from() {
            if let Some(lang) = &user.language_code {
                if let Ok(chat_locale) = Locale::try_from(lang.as_str()) {
                    locale = chat_locale;
                }
            }
        }

        locale
    }
}
