use std::io;

fn main() {
    println!("Enter a feminine word:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let word = input.trim();
    let ending = word.chars().last().unwrap();

    let (base_long, base_short, extra_a) = match ending {
        'a' => (replace_last_vowel(word, 'A'), replace_last_vowel(word, 'a'), false),
        'A' => (word.to_string(), replace_last_vowel(word, 'a'), false),
        'i' => (replace_last_vowel(word, 'I'), replace_last_vowel(word, 'i'), false),
        'I' => (word.to_string(), replace_last_vowel(word, 'i'), true),
        'u' => (replace_last_vowel(word, 'U'), replace_last_vowel(word, 'u'), false),
        'U' => (word.to_string(), replace_last_vowel(word, 'u'), false),
        _ => (word.to_string(), word.to_string(), false),
    };

    let is_A_ending = ending == 'A';
    let is_iIUUEnding = matches!(ending, 'i' | 'I' | 'u' | 'U');

    let mut forms: Vec<(&str, Vec<String>, Vec<String>)> = vec![
        ("First Case",
            if is_A_ending {
                vec![base_long.clone()] // No "I" added for A-ending words
            } else if is_iIUUEnding {
                let mut v = vec![base_long.clone()];
                if extra_a { v.push(format!("{}A", word)); }
                v
            } else {
                vec![format!("{}", base_long), format!("{}u", base_long), format!("{}o", base_long)]
            },
            if is_A_ending {
                vec![format!("{}", base_long), format!("{}u", base_long), format!("{}o", base_long)]
            } else if is_iIUUEnding {
                let mut v = vec![base_long.clone()];
                if extra_a { v.push(format!("{}A", word)); }
                v.push(format!("{}u", base_long));
                v.push(format!("{}o", base_long));
                v
            } else {
                vec![format!("{}", base_long), format!("{}u", base_long), format!("{}o", base_long)]
            }),

        ("Second Case",
            vec![format!("{}M", base_short)],
            if is_A_ending {
                vec![format!("{}", base_long), format!("{}u", base_long), format!("{}o", base_long)]
            } else if is_iIUUEnding {
                let mut v = vec![format!("{}", base_long)];
                if extra_a { v.push(format!("{}A", word)); }
                v.push(format!("{}u", base_long));
                v.push(format!("{}o", base_long));
                v
            } else {
                vec![format!("{}", base_long), format!("{}u", base_long), format!("{}o", base_long)]
            }),

        ("Third Case",
            if is_A_ending {
                vec![format!("{}a", base_long), format!("{}i", base_long), format!("{}e", base_long)]
            } else {
                vec![format!("{}a", base_long), format!("{}A", base_long), format!("{}i", base_long), format!("{}e", base_long)]
            },
            vec![format!("{}hi", base_long), format!("{}hiM", base_long), format!("{}hi~", base_long)]),

        ("Fourth Case",
            if is_A_ending {
                vec![format!("{}a", base_long), format!("{}i", base_long), format!("{}e", base_long)]
            } else {
                vec![format!("{}a", base_long), format!("{}A", base_long), format!("{}i", base_long), format!("{}e", base_long)]
            },
            vec![format!("{}Na", base_long), format!("{}NaM", base_long)]),

        ("Fifth Case",
            if is_A_ending {
                vec![
                    format!("{}a", base_long),
                    format!("{}i", base_long),
                    format!("{}e", base_long),
                    format!("{}tto", base_short),
                    format!("{}o", base_long),
                    format!("{}u", base_long),
                    format!("{}hinto", base_long),
                ]
            } else {
                vec![
                    format!("{}a", base_long),
                    format!("{}A", base_long),
                    format!("{}i", base_long),
                    format!("{}e", base_long),
                    format!("{}tto", base_short),
                    format!("{}o", base_long),
                    format!("{}u", base_long),
                    format!("{}hinto", base_long),
                ]
            },
            vec![
                format!("{}tto", base_short),
                format!("{}o", base_long),
                format!("{}u", base_long),
                format!("{}hinto", base_long),
                format!("{}sunto", base_long),
            ]),

        ("Sixth Case",
            if is_A_ending {
                vec![format!("{}a", base_long), format!("{}i", base_long), format!("{}e", base_long)]
            } else {
                vec![format!("{}a", base_long), format!("{}A", base_long), format!("{}i", base_long), format!("{}e", base_long)]
            },
            vec![format!("{}Na", base_long), format!("{}NaM", base_long)]),

        ("Seventh Case",
            if is_A_ending {
                vec![format!("{}a", base_long), format!("{}i", base_long), format!("{}e", base_long)]
            } else {
                vec![format!("{}a", base_long), format!("{}A", base_long), format!("{}i", base_long), format!("{}e", base_long)]
            },
            vec![format!("{}su", base_long), format!("{}suM", base_long)]),
    ];

    for (case, singular, plural) in forms.iter() {
        println!("{}", case);
        println!("  Singular:");
        for form in singular {
            println!("    {}", form);
        }
        println!("  Plural:");
        for form in plural {
            println!("    {}", form);
        }
        println!();
    }
}

fn replace_last_vowel(word: &str, to: char) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    for i in (0..chars.len()).rev() {
        if "aiuAIU".contains(chars[i]) {
            chars[i] = to;
            break;
        }
    }
    chars.into_iter().collect()
}