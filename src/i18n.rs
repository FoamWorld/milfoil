use fluent::{FluentArgs, FluentBundle, FluentResource};
use std::{collections::HashMap, fs};
use unic_langid::LanguageIdentifier;

pub struct LocaleTexts {
    bundles: HashMap<String, FluentBundle<FluentResource>>,
    // pub fallback_lang: String,
    pub current_lang: String, // uses ISO 639-1 (not strictly)
}

impl LocaleTexts {
    pub fn new(current_lang: String) -> Self {
        Self {
            bundles: HashMap::new(),
            // fallback_lang: "zh".to_string(),
            current_lang,
        }
    }

    pub fn load_file(&mut self, path: &str) -> Option<()> {
        let ftl_path = self.current_lang.clone() + "/" + path;
        self.load_generic_file(&self.current_lang.clone(), ftl_path.as_str())
    }

    pub fn load_generic_file(&mut self, lang: &str, ftl_path: &str) -> Option<()> {
        let path = "./assets/locales/".to_string() + ftl_path;
        let content = fs::read_to_string(path).ok()?;
        let resource = FluentResource::try_new(content).ok()?;

        if !self.bundles.contains_key(lang) {
            let langid: LanguageIdentifier = lang.parse().unwrap();
            self.bundles
                .insert(lang.to_string(), FluentBundle::new(vec![langid]));
        }

        let bundle = self.bundles.get_mut(lang)?;
        bundle.add_resource(resource).ok()?;
        Some(())
    }

    // pub fn load_entry_from_file(&mut self, lang: &str, path: &str) -> Option<()> {
    // }

    pub fn try_translate(&self, key: &str, args: Option<&FluentArgs>) -> Option<String> {
        if let Some(str) = key.strip_prefix('#') {
            return Some(str.to_string());
        }

        let bundle = self.bundles.get(&self.current_lang)?;
        if let Some(message) = bundle.get_message(key) {
            let mut errors = vec![];
            let pattern = message.value()?;
            let value = bundle.format_pattern(pattern, args, &mut errors);
            Some(value.to_string())
        } else {
            None
        }
    }

    pub fn translate(&self, key: &str, args: Option<&FluentArgs>) -> String {
        if let Some(translation) = self.try_translate(key, args) {
            return translation;
        } else {
            return key.to_string();
        }
    }
}
