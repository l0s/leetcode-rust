// 721. Accounts Merge
// https://leetcode.com/problems/accounts-merge

pub struct Solution;

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;
use DsuEntry::{GroupMember, GroupRoot};

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // create account domain objects
        // account ID is each account's index in the vector
        let accounts = accounts
            .into_iter()
            .map(|vec| vec.into())
            .map(|account| Rc::new(RefCell::new(account)))
            .collect::<Vec<Rc<RefCell<Account>>>>();

        // accounts are connected to each other by email address
        // key: email address, a bidirectional edge in the graph of accounts
        // value: all the account IDs associated with the email address, the vertices in the graph
        let mut email_memberships = HashMap::new();
        for (account_id, account) in accounts.iter().enumerate() {
            for email in &account.borrow().emails {
                email_memberships
                    .entry(email.clone())
                    .and_modify(|set: &mut HashSet<usize>| {
                        set.insert(account_id);
                    })
                    .or_insert_with(|| HashSet::from([account_id]));
            }
        }

        // disjoint set / union find to identify the connected components
        let entries = accounts.iter().map(|_| GroupRoot { size: 1 }).collect();
        let mut disjoint_set = DisjointSet { entries, accounts };
        for connected_accounts in email_memberships.values() {
            let mut iter = connected_accounts.iter();
            if let Some(first) = iter.next() {
                for other in iter {
                    disjoint_set.merge(*first, *other);
                }
            }
        }

        disjoint_set
            .get_root_accounts()
            .into_iter()
            .map(|account| account.format())
            .collect()
    }
}

struct DisjointSet {
    entries: Vec<DsuEntry>,
    accounts: Vec<Rc<RefCell<Account>>>,
}

impl DisjointSet {
    pub fn get_root_accounts(&self) -> Vec<Account> {
        let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (index, entry) in self.entries.iter().enumerate() {
            let parent_index = match entry {
                GroupMember { parent_index } => self.find_root(parent_index),
                GroupRoot { .. } => index,
            };
            map.entry(parent_index)
                .and_modify(|bucket| {
                    bucket.insert(index);
                })
                .or_insert_with(|| HashSet::from([index]));
        }
        map.values()
            .map(|member_indices| -> Account {
                let mut name = "".to_string();
                let mut emails = BTreeSet::new();
                for index in member_indices {
                    let account = self.accounts.get(*index).expect("Invalid index");
                    if name.is_empty() {
                        name = account.borrow().name.clone();
                    }
                    emails = emails.union(&account.borrow().emails).cloned().collect();
                }
                Account { name, emails }
            })
            .collect()
    }

    pub fn find_root(&self, index: &usize) -> usize {
        let mut root_index = *index;
        loop {
            if let Some(entry) = self.entries.get(root_index) {
                match entry {
                    GroupMember { parent_index } => root_index = *parent_index,
                    GroupRoot { .. } => break,
                }
            }
        }
        root_index
    }

    /// Indicate that the accounts at indices `x` and `y` are connected (share at least 1 email
    /// address). Merge the two entries, making the larger of the two a `GroupRoot` and the smaller
    /// of the two, a `GroupMember` pointing to the root as its parent.
    ///
    /// Parameters:
    /// - `x` - an index into the entries table
    /// - `y` - an index into the entries table
    pub fn merge(&mut self, x: usize, y: usize) {
        /// Find the root (representative member) of the account at `index` and compress all the
        /// entries along the path such that they point directly to the root as their parent.
        ///
        /// Parameters:
        /// - `disjoint_set` - the set to which the index belongs
        /// - `index` - an account ID
        ///
        /// Returns: the index of the root / representative member account
        fn find_and_compress(disjoint_set: &mut DisjointSet, index: usize) -> (usize, usize) {
            let mut root_index = index;
            let mut path = VecDeque::with_capacity(disjoint_set.entries.len());

            let original_size: usize;
            loop {
                if let Some(entry) = disjoint_set.entries.get(root_index) {
                    match entry {
                        GroupMember { parent_index } => {
                            path.push_back(root_index);
                            root_index = *parent_index;
                        }
                        GroupRoot { size } => {
                            original_size = *size;
                            break;
                        }
                    }
                }
            }

            for node in &path {
                disjoint_set.entries[*node] = GroupMember {
                    parent_index: root_index,
                };
            }

            disjoint_set.entries[root_index] = GroupRoot {
                size: original_size + path.len(),
            };

            (root_index, original_size + path.len())
        }
        let (x_root, x_size) = find_and_compress(self, x);
        let (y_root, y_size) = find_and_compress(self, y);
        if x_root == y_root {
            return;
        }

        // merge the smaller set into the larger
        let (larger_group_index, smaller_group_index) = if x_size < y_size {
            (y_root, x_root)
        } else {
            (x_root, y_root)
        };

        self.entries[smaller_group_index] = GroupMember {
            parent_index: larger_group_index,
        };
        self.entries[larger_group_index] = GroupRoot {
            size: x_size + y_size,
        };
        find_and_compress(self, smaller_group_index);
    }
}

#[derive(Clone, Debug)]
enum DsuEntry {
    GroupMember { parent_index: usize },
    GroupRoot { size: usize },
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Account {
    name: String,
    emails: BTreeSet<String>,
}

impl Account {
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
