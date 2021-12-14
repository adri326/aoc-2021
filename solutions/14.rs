use itertools::Itertools;
use std::collections::HashMap;

const DAY: u32 = 14;

#[derive(Debug, Clone)]
struct Recipe(Vec<char>, Vec<(char, char, char)>);

fn parse(raw: String) -> Recipe { // Initial string and (left, right, insert)
    let mut lines = raw.split("\n").filter(|line| *line != "");
    let header = lines.next().unwrap().chars().collect::<Vec<_>>();

    let mut recipes = Vec::with_capacity(lines.size_hint().0);
    for line in lines {
        if let [input, output] = line.split(" -> ").collect::<Vec<_>>()[0..2] {
            if let [left, right] = input.chars().collect::<Vec<_>>()[0..2] {
                if let Some(out) = output.chars().next() {
                    recipes.push((left, right, out));
                }
            }
        }
    }

    Recipe(header, recipes)
}

fn part_1(recipe: Recipe) -> usize {
    let mut current = recipe.0;

    for _ in 0..10 {
        let mut next = Vec::with_capacity(current.len());

        for (&left, &right) in current.iter().zip(current.iter().skip(1)) {
            next.push(left);
            for subst in &recipe.1 {
                if subst.0 == left && subst.1 == right {
                    next.push(subst.2);
                    break;
                }
            }
        }
        next.push(*current.iter().last().unwrap());

        current = next;
    }

    let mut elements = HashMap::new();

    for &element in &current {
        elements.insert(element, elements.get(&element).map(|x| *x + 1).unwrap_or(1));
    }

    let max = elements.iter().map(|(k, c)| *c).max().unwrap_or(0);
    let min = elements.iter().map(|(k, c)| *c).min().unwrap_or(0);

    max - min
}

#[inline]
fn add<K: Eq + std::hash::Hash + Copy>(map: &mut HashMap<K, usize>, key: K, value: usize) {
    map.insert(key, map.get(&key).map(|x| *x + value).unwrap_or(value));
}

#[inline]
fn sub<K: Eq + std::hash::Hash + Copy>(map: &mut HashMap<K, usize>, key: K, value: usize) {
    map.insert(key, map.get(&key).map(|x| *x - value).unwrap_or(0));
}

fn part_2(recipe: Recipe) -> usize {
    let mut pairs = HashMap::new();
    let mut counts = HashMap::new();

    // init pairs and counts
    for (&left, &right) in recipe.0.iter().zip(recipe.0.iter().skip(1)) {
        pairs.insert((left, right), pairs.get(&(left, right)).map(|x| *x + 1).unwrap_or(1));
        counts.insert(left, counts.get(&left).map(|x| *x + 1).unwrap_or(1));
    }
    if let Some(&last) = recipe.0.iter().last() {
        counts.insert(last, counts.get(&last).map(|x| *x + 1).unwrap_or(1));
    }


    for _n in 0..40 {
        let mut new_pairs = pairs.clone();
        let mut new_counts = counts.clone();
        for &(left, right, subst) in recipe.1.iter() {
            let amount = pairs.get(&(left, right)).map(|x| *x).unwrap_or(0);
            add(&mut new_pairs, (left, subst), amount);
            add(&mut new_pairs, (subst, right), amount);
            sub(&mut new_pairs, (left, right), amount);
            add(&mut new_counts, subst, amount);
        }
        pairs = new_pairs;
        counts = new_counts;
    }

    let max = counts.iter().map(|(k, c)| *c).max().unwrap_or(0);
    let min = counts.iter().map(|(k, c)| *c).min().unwrap_or(0);

    max - min
}

fn main() {
    let sample_file = format!("./sample/{:02}.txt", DAY);
    let sample = parse(std::fs::read_to_string(sample_file).unwrap());

    let input_file = format!("./input/{:02}.txt", DAY);
    let input = std::fs::read_to_string(input_file).ok().map(|str| parse(str));

    println!("[04-1] Sample: {}", part_1(sample.clone()));
    println!("[04-2] Sample: {}", part_2(sample));

    if let Some(input) = input {
        println!("[04-1] Input: {}", part_1(input.clone()));
        println!("[04-2] Input: {}", part_2(input));
    }
}
