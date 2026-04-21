use crate::iterators::ValidWordIteratorStrategy;
use std::collections::BTreeMap;

#[test]
pub fn basic_grammar_word_generation_test() {
    use crate::grammar::Grammar;
    let non_terminals: Vec<String> = vec!["S", "A", "B", "X", "Y"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let terminals: Vec<String> = vec!["a", "b", "c", "slovo1", "slovo2"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut rules = BTreeMap::new();
    rules.insert(
        vec!["S".to_string()],
        vec![vec!["a".to_string(), "A".to_string(), "b".to_string()]],
    );
    rules.insert(
        vec!["a".to_string(), "A".to_string()],
        vec![vec!["X".to_string()]],
    );
    rules.insert(
        vec!["A".to_string(), "b".to_string()],
        vec![vec!["Y".to_string()]],
    );
    rules.insert(
        vec!["A".to_string()],
        vec![
            vec!["Y".to_string(), "A".to_string(), "b".to_string()],
            vec!["X".to_string(), "A".to_string(), "a".to_string()],
        ],
    );
    rules.insert(vec!["X".to_string()], vec![vec!["slovo1".to_string()]]);
    rules.insert(vec!["Y".to_string()], vec![vec!["slovo2".to_string()]]);

    let grammar = Grammar::new(non_terminals, terminals, rules, "S".to_string());
    let words = grammar
        .valid_words_iter(ValidWordIteratorStrategy::DepthFirst {
            recursion_limit: 10,
        })
        .take(13)
        .collect::<Vec<_>>();
    assert_eq!(words.len(), 13);
    assert!(words.iter().any(|w| {
        w.symbols
            .iter()
            .map(|s| s.as_str().to_string())
            .collect::<Vec<String>>()
            == vec![
                "a".to_string(),
                "slovo2".to_string(),
                "slovo2".to_string(),
                "slovo2".to_string(),
                "slovo2".to_string(),
                "b".to_string(),
                "b".to_string(),
                "b".to_string(),
            ]
    }));
}

#[test]
pub fn foo() {
    use crate::grammar::Grammar;
    let non_terminals: Vec<String> = vec!["S", "A", "B", "X", "Y"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let terminals: Vec<String> = vec!["a", "b", "c", "slovo1", "slovo2"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut rules = BTreeMap::new();
    rules.insert(
        vec!["S".to_string()],
        vec![vec!["a".to_string(), "A".to_string(), "b".to_string()]],
    );
    rules.insert(
        vec!["a".to_string(), "A".to_string()],
        vec![vec!["X".to_string()]],
    );
    rules.insert(
        vec!["A".to_string(), "b".to_string()],
        vec![vec!["Y".to_string()]],
    );
    rules.insert(
        vec!["A".to_string()],
        vec![
            vec!["Y".to_string(), "A".to_string(), "b".to_string()],
            vec!["X".to_string(), "A".to_string(), "a".to_string()],
        ],
    );
    rules.insert(vec!["X".to_string()], vec![vec!["slovo1".to_string()]]);
    rules.insert(vec!["Y".to_string()], vec![vec!["slovo2".to_string()]]);

    let grammar = Grammar::new(non_terminals, terminals, rules, "S".to_string());
    let words = grammar
        .valid_words_iter(ValidWordIteratorStrategy::BreadthFirst)
        .take(13)
        .collect::<Vec<_>>();

    assert_eq!(words.len(), 13);
    assert!(words.iter().any(
        |w| w.symbols().iter().map(|s| s.as_str()).collect::<Vec<_>>() == vec!["a", "slovo2"]
    ));
}
