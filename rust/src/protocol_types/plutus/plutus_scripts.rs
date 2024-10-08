use crate::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, JsonSchema,
)]
pub struct PlutusScripts(pub(crate) Vec<PlutusScript>);

impl_to_from!(PlutusScripts);

impl NoneOrEmpty for PlutusScripts {
    fn is_none_or_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[wasm_bindgen]
impl PlutusScripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusScript {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: &PlutusScript) {
        self.0.push(elem.clone());
    }

    #[allow(dead_code)]
    pub(crate) fn by_version(&self, language: &Language) -> PlutusScripts {
        PlutusScripts(
            self.0
                .iter()
                .filter(|s| s.language_version().eq(language))
                .map(|s| s.clone())
                .collect(),
        )
    }

    pub(crate) fn has_version(&self, language: &Language) -> bool {
        self.0.iter().any(|s| s.language_version().eq(language))
    }

    pub(crate) fn merge(&self, other: &PlutusScripts) -> PlutusScripts {
        let mut res = self.clone();
        for s in &other.0 {
            res.add(s);
        }
        res
    }

    pub(crate) fn view(&self, version: &Language) -> Vec<&PlutusScript> {
        let mut res = Vec::new();
        for script in &self.0 {
            if !script.language_version().eq(version) {
                continue;
            }
            res.push(script);
        }
        res
    }

    pub(crate) fn deduplicated_view(&self, version: Option<&Language>) -> Vec<&PlutusScript> {
        let mut dedup = BTreeSet::new();
        let mut res = Vec::new();
        for script in &self.0 {
            if let Some(version) = version {
                if !script.language_version().eq(version) {
                    continue;
                }
            }
            if dedup.insert(script) {
                res.push(script);
            }
        }
        res
    }

    pub(crate) fn deduplicated_clone(&self) -> PlutusScripts {
        let mut dedup = BTreeSet::new();
        let mut scripts = Vec::new();
        for script in &self.0 {
            if dedup.insert(script.clone()) {
                scripts.push(script.clone());
            }
        }
        PlutusScripts(scripts)
    }

    #[allow(dead_code)]
    pub(crate) fn contains(&self, script: &PlutusScript) -> bool {
        self.0.contains(&script)
    }
}
