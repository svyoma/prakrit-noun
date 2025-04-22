use std::io;

fn main() {
    // Take user input
    let mut input = String::new();
    println!("Enter the Prakrit word:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let word = input.trim();

    // Determine the word ending
    let last_char = word.chars().last().unwrap_or_default();

    // Generate transformed forms based on ending
    let without_last_vowel = remove_last_vowel(word);
    let with_a_to_A = replace_vowel(word, "A", last_char, true);
    let with_a_to_e = replace_vowel(word, "e", last_char, true);
    let with_i_to_I_u_to_U = replace_vowel(word, "", last_char, false);

    // Generate and print noun forms
    generate_and_print_forms(
        word,
        &without_last_vowel,
        &with_a_to_A,
        &with_a_to_e,
        &with_i_to_I_u_to_U,
        last_char,
    );
}

// Remove the last vowel from the word
fn remove_last_vowel(word: &str) -> String {
    if let Some(pos) = word.rfind(|c: char| "aeiou".contains(c)) {
        let mut chars: Vec<char> = word.chars().collect();
        chars.remove(pos);
        return chars.into_iter().collect();
    }
    word.to_string()
}

// Replace vowels based on specific rules, targeting only the last vowel
fn replace_vowel(word: &str, replacement: &str, ending: char, is_a: bool) -> String {
    if is_a && ending == 'a' {
        return word.rsplit_once('a').map_or(word.to_string(), |(start, _)| format!("{start}{replacement}"));
    } else if ending == 'i' || ending == 'u' {
        let new_vowel = if replacement == "A" {
            if ending == 'i' { "I" } else { "U" }
        } else if replacement == "" {
            // When replacement is empty, still use I or U
            if ending == 'i' { "I" } else { "U" }
        } else {
            replacement
        };
        return word.rsplit_once(ending).map_or(word.to_string(), |(start, _)| format!("{start}{new_vowel}"));
    }
    word.to_string()
}

// Generate and print forms based on cases and rules
fn generate_and_print_forms(
    original: &str,
    no_vowel: &str,
    a_to_A: &str,
    a_to_e: &str,
    i_to_I_u_to_U: &str,
    ending: char,
) {
    let cases = vec![
        (
            "First Case",
            match ending {
                'i' | 'u' => vec![("M", original)],  // Add parentheses to make it a proper tuple
                'a' => vec![("M", original)],
                _ => vec![],
            },            
            match ending {
                'i' | 'u' => vec![("iM", i_to_I_u_to_U), ("i~", i_to_I_u_to_U), ("Ni", i_to_I_u_to_U)],  // Add parentheses to make it a proper tuple
                'a' => vec![("iM", a_to_A), ("i~", a_to_A), ("Ni", a_to_A)],
                _ => vec![],
            },
        ),
        (
            "Second Case",
            match ending {
                'i' | 'u' => vec![("M", i_to_I_u_to_U)],  // Add parentheses to make it a proper tuple
                'a' => vec![("M", a_to_A)],
                _ => vec![],
            },            
            match ending {
                'i' | 'u' => vec![("iM", i_to_I_u_to_U), ("i~", i_to_I_u_to_U), ("Ni", i_to_I_u_to_U)],  // Add parentheses to make it a proper tuple
                'a' => vec![("iM", a_to_A), ("i~", a_to_A), ("Ni", a_to_A)],
                _ => vec![],
            },
        ),
        
        (
            "Third Case",
            match ending {
                'i' | 'u' => vec![("NA", original)],
                'a' => vec![("Na", a_to_e), ("NaM", a_to_e)],
                _ => vec![],  // Add this case
            },
            match ending {
                'i' | 'u' => vec![("hi", i_to_I_u_to_U), ("hiM", i_to_I_u_to_U), ("hi~", i_to_I_u_to_U)],
                'a' => vec![("hi", a_to_e), ("hiM", a_to_e), ("hi~", a_to_e)],
                _ => vec![],
            },
        ),
        (
            "Fourth Case",
            match ending {
                'i' | 'u' => vec![("ssa", original), ("No", original)],
                'a' => vec![("ssa", original)],
                _ => vec![],  // Add this case
            },
            match ending {
                'i' | 'u' => vec![("Na", i_to_I_u_to_U), ("NaM", i_to_I_u_to_U)],
                'a' => vec![("Na", a_to_A), ("NaM", a_to_A)],
                _ => vec![],
            },
        ),
        (
            "Fifth Case",
            match ending {
                'i' | 'u' => vec![("tto", original), ("o", i_to_I_u_to_U), ("u", i_to_I_u_to_U), ("hinto", i_to_I_u_to_U), ("No", original)],
                _ => vec![("tto", original), ("o", a_to_A), ("u", a_to_A), ("hi", a_to_A), ("hinto", a_to_A), ("", a_to_A)],
            },
            match ending {
                'i' | 'u' => vec![("tto", original), ("o", i_to_I_u_to_U), ("u", i_to_I_u_to_U), ("hinto", i_to_I_u_to_U), ("sunto", i_to_I_u_to_U)],
                'a' => vec![
                    ("tto", original), 
                    ("o", a_to_A), 
                    ("u", a_to_A),
                    ("hi", a_to_A),
                    ("hi", a_to_e),
                    ("hinto", a_to_A),
                    ("hinto", a_to_e),
                    ("sunto", a_to_A),
                    ("sunto", a_to_e),
                ],
                _ => vec![],
            },
        ),
        (
            "Sixth Case",
            match ending {
                'i' | 'u' => vec![("ssa", original), ("No", original)],
                'a' => vec![("ssa", original)],
                _ => vec![],  // Add this case
            },
            match ending {
                'i' | 'u' => vec![("Na", i_to_I_u_to_U), ("NaM", i_to_I_u_to_U)],
                'a' => vec![("Na", a_to_A), ("NaM", a_to_A)],
                _ => vec![],
            },
        ),
        (
            "Seventh Case",
            match ending {
                'a' => vec![("e", no_vowel), ("mmi", original)],
                'i' | 'u' => vec![("mmi", original)],
                _ => vec![],
            },
            match ending {
                'a' => vec![("su", a_to_e), ("suM", a_to_e)],
                'i' | 'u' => vec![("su", i_to_I_u_to_U), ("suM", i_to_I_u_to_U)],
                _ => vec![],
            },
        ),
    ];

    for (case_name, singular_suffixes, plural_suffixes) in cases {
        println!("{}", case_name);

        // Print Singular forms
        println!("  Singular:");
        for (suffix, base_form) in singular_suffixes {
            println!("    {}{}", base_form, suffix);
        }

        // Print Plural forms
        println!("  Plural:");
        for (suffix, base_form) in plural_suffixes {
            println!("    {}{}", base_form, suffix);
        }
    }
}