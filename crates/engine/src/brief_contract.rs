//! The `brief` contract (builtin-tools §brief): the host reading of a
//! strict, reference-native brief. Atoms carry a kind and cite source
//! aliases; the goal references atom ids; completion is explicit or, when the
//! model leaves it empty, derived by the host as the goal atoms that are not
//! facts — a fact stays useful to the goal without becoming completion.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BriefAtomKind {
    Intent,
    Fact,
    Constraint,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BriefAtom {
    pub id: String,
    pub kind: BriefAtomKind,
    pub text: String,
    pub source_refs: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BriefContract {
    pub atoms: Vec<BriefAtom>,
    pub goal_atom_ids: Vec<String>,
    pub explicit_completion_atom_ids: Vec<String>,
    pub effective_completion_atom_ids: Vec<String>,
}

#[derive(Deserialize)]
struct Wire {
    atoms: Vec<BriefAtom>,
    goal: IdList,
    completion: IdList,
}

#[derive(Deserialize)]
struct IdList {
    atom_ids: Vec<String>,
}

/// Read and cross-check a schema-valid brief against the turn's source
/// aliases. Rejections name the first violated rule.
pub fn read_brief(arguments: &Value, valid_aliases: &[String]) -> Result<BriefContract, String> {
    let wire: Wire = serde_json::from_value(arguments.clone())
        .map_err(|error| format!("brief shape: {error}"))?;
    if wire.atoms.is_empty() {
        return Err("brief needs at least one atom".to_owned());
    }
    let mut ids = BTreeSet::new();
    for atom in &wire.atoms {
        if atom.id.trim().is_empty() || atom.text.trim().is_empty() {
            return Err("brief atoms need a non-empty id and text".to_owned());
        }
        if !ids.insert(atom.id.as_str()) {
            return Err(format!("brief atom id {} is repeated", atom.id));
        }
        if atom.source_refs.is_empty() {
            return Err(format!("brief atom {} cites no source", atom.id));
        }
        for alias in &atom.source_refs {
            if !valid_aliases.iter().any(|valid| valid == alias) {
                return Err(format!(
                    "brief atom {} cites unknown source alias {alias}; valid: {}",
                    atom.id,
                    valid_aliases.join(", ")
                ));
            }
        }
    }
    let defined = |list: &[String], role: &str| -> Result<(), String> {
        let mut seen = BTreeSet::new();
        for id in list {
            if !ids.contains(id.as_str()) {
                return Err(format!("brief {role} references undefined atom {id}"));
            }
            if !seen.insert(id.as_str()) {
                return Err(format!("brief {role} repeats atom {id}"));
            }
        }
        Ok(())
    };
    defined(&wire.goal.atom_ids, "goal")?;
    defined(&wire.completion.atom_ids, "completion")?;
    if wire.goal.atom_ids.is_empty() {
        return Err("brief goal references no atom".to_owned());
    }
    let effective = if wire.completion.atom_ids.is_empty() {
        wire.goal
            .atom_ids
            .iter()
            .filter(|id| {
                wire.atoms
                    .iter()
                    .any(|atom| &atom.id == *id && atom.kind != BriefAtomKind::Fact)
            })
            .cloned()
            .collect()
    } else {
        wire.completion.atom_ids.clone()
    };
    Ok(BriefContract {
        atoms: wire.atoms,
        goal_atom_ids: wire.goal.atom_ids,
        explicit_completion_atom_ids: wire.completion.atom_ids,
        effective_completion_atom_ids: effective,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn brief(completion: Vec<&str>) -> Value {
        json!({
            "atoms": [
                {"id":"a1","kind":"intent","text":"Write the migration plan in Chinese","source_refs":["s1"]},
                {"id":"a2","kind":"fact","text":"The kernel ledger is append-only; current phase is live UAT","source_refs":["s2"]},
                {"id":"a3","kind":"constraint","text":"Do not modify product code","source_refs":["s1"]}
            ],
            "goal": {"atom_ids": ["a1","a2","a3"]},
            "completion": {"atom_ids": completion}
        })
    }

    #[test]
    fn effective_completion_is_the_goal_minus_facts_when_explicit_is_empty() {
        let contract = read_brief(&brief(vec![]), &["s1".into(), "s2".into()]).expect("valid");
        assert_eq!(contract.explicit_completion_atom_ids, Vec::<String>::new());
        assert_eq!(contract.effective_completion_atom_ids, vec!["a1", "a3"]);
        let explicit = read_brief(&brief(vec!["a1"]), &["s1".into(), "s2".into()]).expect("valid");
        assert_eq!(explicit.effective_completion_atom_ids, vec!["a1"]);
    }

    #[test]
    fn cross_references_are_checked() {
        let aliases = ["s1".to_owned(), "s2".to_owned()];
        let mut dangling = brief(vec![]);
        dangling["goal"]["atom_ids"] = json!(["a9"]);
        assert!(
            read_brief(&dangling, &aliases)
                .unwrap_err()
                .contains("undefined atom a9")
        );
        let mut foreign = brief(vec![]);
        foreign["atoms"][0]["source_refs"] = json!(["s7"]);
        assert!(
            read_brief(&foreign, &aliases)
                .unwrap_err()
                .contains("unknown source alias s7")
        );
        let mut repeated = brief(vec![]);
        repeated["atoms"][2]["id"] = json!("a1");
        assert!(
            read_brief(&repeated, &aliases)
                .unwrap_err()
                .contains("repeated")
        );
    }
}
