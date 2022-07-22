use crate::adapter;
use std::fmt::Write;

pub fn to_string_pretty(my_contributions: adapter::MyContributions) -> String {
    let mut result = String::new();
    for (owner, issue_or_prs) in my_contributions {
        let _ = writeln!(result, "\n### {}\n", owner);
        for issue_or_pr in issue_or_prs {
            let _ = writeln!(
                result,
                "- [{}]({}) **{}!**",
                issue_or_pr.title, issue_or_pr.url, issue_or_pr.state
            );
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_to_string_pretty() {
        let cc = serde_json::from_str(include_str!("fixtures/myContributions.json")).unwrap();
        let actual = to_string_pretty(cc);
        assert_snapshot!(
            actual,
            @r###"

        ### rust-lang/rust

        - [Cannot build on Fedora: wrong CPUTYPE?](https://github.com/rust-lang/rust/issues/1218) **open!**
        - [rustc: Fix position of diagnostic highlight lines](https://github.com/rust-lang/rust/issues/1219) **closed!**
        - [Add float support to #fmt.](https://github.com/rust-lang/rust/pull/1168) **merged!**
        - [Prohibit in-scope consts from use as variable names in binders, like nullary tags](https://github.com/rust-lang/rust/pull/1193) **merged!**

        ### daido1976/terakoya

        - [From now to Vercel](https://github.com/daido1976/terakoya/pull/52) **merged!**
        "###
        );
    }
}
