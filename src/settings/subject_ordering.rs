use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Deserialize)]
pub struct SubjectOrdering {
    by: SubjectOrderingBy,
    direction: SubjectOrderingDirection,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum SubjectOrderingBy {
    Alphabetical,
    AccessTime,
    ModifyTime,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum SubjectOrderingDirection {
    Ascending,
    Descending,
}

impl Default for SubjectOrdering {
    fn default() -> Self {
        Self {
            by: SubjectOrderingBy::Alphabetical,
            direction: SubjectOrderingDirection::Ascending,
        }
    }
}

impl SubjectOrdering {
    pub fn sort_subjects(
        &self,
        subjects: impl Iterator<Item = PathBuf>,
    ) -> impl Iterator<Item = PathBuf> {
        fn inner<K: Ord>(
            subjects: impl Iterator<Item = PathBuf>,
            mut f: impl FnMut(&Path) -> Option<K>,
            direction: SubjectOrderingDirection,
        ) -> Vec<PathBuf> {
            let mut subjects: Vec<_> = subjects
                .map(|s| {
                    let key = f(&s);
                    (s, key)
                })
                .collect();

            // `None`s appear last
            subjects.sort_unstable_by(|(_s1, key1), (_s2, key2)| match (key1, key2) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(key1), Some(key2)) => (match direction {
                    SubjectOrderingDirection::Ascending => |o: Ordering| o,
                    SubjectOrderingDirection::Descending => |o: Ordering| o.reverse(),
                })(key1.cmp(key2)),
            });
            subjects.into_iter().map(|(s, _key)| s).collect()
        }

        match self.by {
            SubjectOrderingBy::Alphabetical => inner(
                subjects,
                |path| path.file_name().map(OsStr::to_owned),
                self.direction,
            ),
            SubjectOrderingBy::AccessTime => inner(
                subjects,
                |path| path.metadata().ok()?.accessed().ok(),
                self.direction,
            ),
            SubjectOrderingBy::ModifyTime => inner(
                subjects,
                |path| path.metadata().ok()?.modified().ok(),
                self.direction,
            ),
        }
        .into_iter()
    }
}
