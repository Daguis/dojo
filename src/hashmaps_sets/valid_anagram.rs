use std::collections::HashMap;

#[allow(unused)]

pub fn is_anagram(s: String, t: String) -> bool {
    let mut chars_count = HashMap::new();

    if s.len() != t.len() {
        return false;
    }

    // Recorrer las cadenas usando un bucle por índices
    for i in 0..s.len() {
        let ch_s = s.chars().nth(i).unwrap();
        let ch_t = t.chars().nth(i).unwrap();

        // Sumar el carácter de `s`
        chars_count.entry(ch_s).and_modify(|e| *e += 1).or_insert(1);
        // Restar el carácter de `t`
        chars_count.entry(ch_t).and_modify(|e| *e -= 1);

        // Verificar y limpiar el valor resultante
        match chars_count.remove(&ch_t) {
            Some(0) => continue,
            Some(v) => {
                chars_count.insert(ch_t, v);
            }
            None => continue,
        }
    }

    chars_count.is_empty()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ex1() {
        let s = "anagram".into();
        let t = "nagaram".into();
        let result = is_anagram(s, t);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn ex2() {
        let s = "rat".into();
        let t = "car".into();
        let result = is_anagram(s, t);
        let expected = false;
        assert_eq!(result, expected);
    }
}
