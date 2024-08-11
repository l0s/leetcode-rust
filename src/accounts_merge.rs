// 721. Accounts Merge
// https://leetcode.com/problems/accounts-merge

pub struct Solution;

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let accounts = accounts
            .into_iter()
            .map(|vec| vec.into())
            .collect::<Vec<Account>>();
        let mut map: HashMap<String, Rc<RefCell<Account>>> = HashMap::with_capacity(accounts.len());
        for account in accounts {
            #[allow(clippy::mutable_key_type)]
            let mut duplicate_accounts = account
                .emails
                .iter()
                .filter_map(|email| map.get(email))
                .cloned()
                .map(HashedAccount)
                .collect::<HashSet<HashedAccount>>();
            duplicate_accounts.insert(HashedAccount(Rc::new(RefCell::new(account))));
            let mut prioritised_accounts = duplicate_accounts
                .into_iter()
                .collect::<Vec<HashedAccount>>();
            prioritised_accounts
                .sort_unstable_by_key(|y| std::cmp::Reverse(y.0.borrow().num_emails()));

            let primary = &prioritised_accounts[0];
            for subordinate in &prioritised_accounts[1..] {
                primary
                    .0
                    .borrow_mut()
                    .absorb(subordinate.0.borrow().deref());
            }
            // TODO can we skip updating the emails already pointing to the primary?
            // TODO do we know if primary is already mapped?

            for email in &primary.0.borrow().emails
            /*.difference(&original_emails)*/
            {
                map.insert(email.clone(), primary.0.clone());
            }
        }

        #[allow(clippy::mutable_key_type)]
        map.values()
            .cloned()
            .map(HashedAccount)
            .collect::<HashSet<HashedAccount>>()
            .into_iter()
            .map(|account| account.0.borrow().format())
            .collect()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Account {
    name: String,
    emails: BTreeSet<String>,
}

impl Account {
    fn num_emails(&self) -> usize {
        self.emails.len()
    }

    fn absorb(&mut self, other: &Self) {
        assert_eq!(self.name, other.name);
        for other_email in &other.emails {
            self.emails.insert(other_email.clone());
        }
    }

    fn format(&self) -> Vec<String> {
        [
            vec![self.name.clone()],
            self.emails.iter().cloned().collect(),
        ]
        .concat()
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

#[derive(Eq, PartialEq, Debug)]
struct HashedAccount(Rc<RefCell<Account>>);

impl Hash for HashedAccount {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state)
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

    #[test]
    fn example3() {
        // given
        let accounts = [
            vec!["Alex", "Alex5@m.co", "Alex4@m.co", "Alex0@m.co"],
            vec!["Ethan", "Ethan3@m.co", "Ethan3@m.co", "Ethan0@m.co"],
            vec!["Kevin", "Kevin4@m.co", "Kevin2@m.co", "Kevin2@m.co"],
            vec!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe2@m.co"],
            vec!["Gabe", "Gabe3@m.co", "Gabe4@m.co", "Gabe2@m.co"],
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
            vec!["Alex", "Alex5@m.co", "Alex4@m.co", "Alex0@m.co"],
            vec!["Ethan", "Ethan3@m.co", "Ethan3@m.co", "Ethan0@m.co"],
            vec!["Kevin", "Kevin4@m.co", "Kevin2@m.co", "Kevin2@m.co"],
            vec![
                "Gabe",
                "Gabe0@m.co",
                "Gabe3@m.co",
                "Gabe2@m.co",
                "Gabe4@m.co",
            ],
        ];
        let expected = expected_accounts(&expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn example50() {
        // given
        let accounts = [
            vec!["David", "Avid0@m.co", "David0@m.co", "David1@m.co"],
            vec!["David", "Gvid3@m.co", "David3@m.co", "David4@m.co"],
            vec!["David", "David4@m.co", "David5@m.co"],
            vec!["David", "David2@m.co", "David3@m.co"],
            vec!["David", "David1@m.co", "David2@m.co"],
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
        let expected = vec![vec![
            "David",
            "Avid0@m.co",
            "David0@m.co",
            "David1@m.co",
            "Gvid3@m.co",
            "David3@m.co",
            "David4@m.co",
            "David5@m.co",
            "David2@m.co",
        ]];
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
