// 721. Accounts Merge
// https://leetcode.com/problems/accounts-merge

pub struct Solution;

use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let accounts = accounts
            .into_iter()
            .map(|vec| vec.into())
            .collect::<Vec<Account>>();
        let mut map: HashMap<String, Rc<Account>> = HashMap::with_capacity(accounts.len());
        for account in accounts {
            let mut merged = account.clone();
            for email in &account.emails {
                if let Some(duplicate) = map.get(email) {
                    merged = merged.merge(duplicate);
                }
            }
            let merged = Rc::new(merged);
            for email in &merged.emails {
                map.insert(email.clone(), merged.clone());
            }
        }

        let unique_accounts = map.values().cloned().collect::<HashSet<Rc<Account>>>();
        unique_accounts
            .iter()
            .map(|account| {
                [
                    vec![account.name.clone()],
                    account.emails.iter().cloned().collect::<Vec<String>>(),
                ]
                .concat()
            })
            .collect::<Vec<Vec<String>>>()
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Account {
    name: String,
    emails: BTreeSet<String>,
}

impl Account {
    fn merge(&self, other: &Self) -> Self {
        assert_eq!(self.name, other.name);
        Self {
            name: self.name.clone(),
            emails: self.emails.union(&other.emails).cloned().collect(),
        }
    }
}

impl From<Vec<String>> for Account {
    fn from(value: Vec<String>) -> Self {
        assert!(!value.is_empty());
        Self {
            name: value[0].clone(),
            emails: value[1..].iter().cloned().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Account, Solution};
    use std::collections::BTreeSet;

    #[test]
    fn example1() {
        // given
        let accounts = [
            vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            vec!["John", "johnsmith@mail.com", "john00@mail.com"],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ];
        let accounts = accounts
            .map(|inner| inner.into_iter().map(String::from).collect())
            .to_vec();

        // when
        let result = Solution::accounts_merge(accounts);
        let result = result
            .into_iter()
            .map(|inner| inner.into())
            .collect::<BTreeSet<Account>>();

        // then
        let expected = vec![
            vec![
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com",
            ],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ];
        let expected = expected_accounts(&expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        // given
        let accounts = [
            vec!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
            vec!["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
            vec!["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
            vec!["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
            vec!["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
        ];
        let accounts = accounts
            .map(|inner| inner.into_iter().map(String::from).collect())
            .to_vec();

        // when
        let result = Solution::accounts_merge(accounts);
        let result = result
            .into_iter()
            .map(|inner| inner.into())
            .collect::<BTreeSet<Account>>();

        // then
        let expected = vec![
            vec!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
            vec!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
            vec!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
            vec!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
            vec!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
        ];
        let expected = expected_accounts(&expected);
        assert_eq!(result, expected);
    }

    fn expected_accounts(expected: &[Vec<&str>]) -> BTreeSet<Account> {
        expected
            .iter()
            .map(|inner| {
                inner
                    .into_iter()
                    .cloned()
                    .map(String::from)
                    .collect::<Vec<String>>()
            })
            .map(|vec| vec.into())
            .collect::<BTreeSet<Account>>()
    }
}
