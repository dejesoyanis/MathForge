use std::collections::BTreeSet;

/// Endliche Menge ganzer Zahlen. Gleichheit ignoriert Reihenfolge und Duplikate,
/// weil die Elemente in einem `BTreeSet` liegen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntSet {
    elements: BTreeSet<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplementError {
    NotSubset,
}

impl IntSet {
    pub fn empty() -> Self {
        Self {
            elements: BTreeSet::new(),
        }
    }

    pub fn from_slice(items: &[i64]) -> Self {
        Self {
            elements: items.iter().copied().collect(),
        }
    }

    pub fn insert(&mut self, value: i64) -> bool {
        self.elements.insert(value)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn elements(&self) -> Vec<i64> {
        self.elements.iter().copied().collect()
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            elements: self.elements.union(&other.elements).copied().collect(),
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            elements: self.elements.intersection(&other.elements).copied().collect(),
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self {
            elements: self.elements.difference(&other.elements).copied().collect(),
        }
    }

    pub fn symmetric_difference(&self, other: &Self) -> Self {
        Self {
            elements: self
                .elements
                .symmetric_difference(&other.elements)
                .copied()
                .collect(),
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.is_subset(&other.elements)
    }

    pub fn is_proper_subset(&self, other: &Self) -> bool {
        self != other && self.is_subset(other)
    }

    /// Komplement relativ zu einer angegebenen Grundmenge.
    /// Fehler, wenn `self` keine Teilmenge der Grundmenge ist.
    pub fn complement(&self, universe: &Self) -> Result<Self, ComplementError> {
        if !self.is_subset(universe) {
            return Err(ComplementError::NotSubset);
        }
        Ok(universe.difference(self))
    }

    pub fn to_canonical(&self) -> String {
        if self.elements.is_empty() {
            return "∅".to_string();
        }
        let body = self
            .elements
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(",");
        format!("{{{body}}}")
    }

    pub fn to_latex(&self) -> String {
        if self.elements.is_empty() {
            return "\\emptyset".to_string();
        }
        let body = self
            .elements
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(",");
        format!("\\{{{body}\\}}")
    }
}

pub fn inclusion_exclusion_two(card_a: u64, card_b: u64, card_intersection: u64) -> Option<u64> {
    if card_intersection > card_a || card_intersection > card_b {
        return None;
    }
    Some(card_a + card_b - card_intersection)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(items: &[i64]) -> IntSet {
        IntSet::from_slice(items)
    }

    #[test]
    fn equality_ignores_order_and_duplicates() {
        assert_eq!(s(&[1, 2, 3]), s(&[3, 1, 2, 2, 1]));
        assert_eq!(s(&[2, 2, 2]).elements(), vec![2]);
        assert_ne!(s(&[1, 2]), s(&[1, 2, 3]));
    }

    #[test]
    fn empty_subset_and_symmetric_difference() {
        let empty = IntSet::empty();
        let a = s(&[1, 2, 3]);
        let b = s(&[3, 4]);
        assert!(empty.is_subset(&a));
        assert!(empty.is_subset(&empty));
        assert!(!empty.is_proper_subset(&empty));
        assert!(empty.is_proper_subset(&a));
        assert_eq!(a.union(&empty), a);
        assert_eq!(a.intersection(&empty), empty);
        assert_eq!(a.difference(&empty), a);
        assert_eq!(a.symmetric_difference(&a), empty);
        assert_eq!(a.symmetric_difference(&empty), a);
        assert_eq!(a.symmetric_difference(&b).elements(), vec![1, 2, 4]);
        assert_eq!(a.difference(&b).elements(), vec![1, 2]);
        assert_eq!(b.difference(&a).elements(), vec![4]);
    }

    #[test]
    fn de_morgan_on_finite_universe() {
        let universe = s(&[1, 2, 3, 4, 5]);
        let a = s(&[1, 2]);
        let b = s(&[2, 3]);
        let union_complement = a.union(&b).complement(&universe).unwrap();
        let inter_of_complements = a
            .complement(&universe)
            .unwrap()
            .intersection(&b.complement(&universe).unwrap());
        assert_eq!(union_complement, inter_of_complements);
        assert_eq!(union_complement.elements(), vec![4, 5]);

        let inter_complement = a.intersection(&b).complement(&universe).unwrap();
        let union_of_complements = a
            .complement(&universe)
            .unwrap()
            .union(&b.complement(&universe).unwrap());
        assert_eq!(inter_complement, union_of_complements);
        assert_eq!(inter_complement.elements(), vec![1, 3, 4, 5]);
    }

    #[test]
    fn complement_requires_subset_of_universe() {
        let universe = s(&[1, 2, 3]);
        let a = s(&[1, 9]);
        assert_eq!(a.complement(&universe), Err(ComplementError::NotSubset));
    }

    #[test]
    fn canonical_form_sorts_and_uses_empty_symbol() {
        assert_eq!(s(&[3, -1, 3, 0]).to_canonical(), "{-1,0,3}");
        assert_eq!(IntSet::empty().to_canonical(), "∅");
        assert_eq!(s(&[-2, 4]).to_latex(), "\\{-2,4\\}");
    }

    #[test]
    fn inclusion_exclusion_two_sets() {
        assert_eq!(inclusion_exclusion_two(20, 15, 6), Some(29));
        assert_eq!(inclusion_exclusion_two(8, 5, 0), Some(13));
        assert_eq!(inclusion_exclusion_two(3, 3, 4), None);
    }
}
