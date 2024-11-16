use crate::subject::Subject;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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
    pub fn sort_subjects(&self, subjects: impl Iterator<Item = Subject>) -> Box<[Subject]> {
        fn inner<K: Ord>(
            subjects: impl Iterator<Item = Subject>,
            mut f: impl FnMut(&Subject) -> Option<K>,
            direction: SubjectOrderingDirection,
        ) -> Box<[Subject]> {
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
                |subject| Some(subject.name().to_owned()),
                self.direction,
            ),
            SubjectOrderingBy::AccessTime => inner(
                subjects,
                |subject| subject.path().metadata().ok()?.accessed().ok(),
                self.direction,
            ),
            SubjectOrderingBy::ModifyTime => inner(
                subjects,
                |subject| subject.path().metadata().ok()?.modified().ok(),
                self.direction,
            ),
        }
    }
}
