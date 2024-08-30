use passwords::PasswordGenerator;

pub async fn generate_password(count: i32) -> Vec<String> {
    let pg = PasswordGenerator {
        length: 12,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    pg.generate(count).unwrap_or_else(|_| vec!["Failed to generate password".to_string()])
}