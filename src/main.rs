use std::collections::HashMap;

use fake::faker::boolean::en::Boolean;
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::faker::phone_number::raw::*;
use fake::{locales::*, Fake};
use rand::random;
use reqwest::Client;

#[derive(Debug)]
struct FakeData {
    first_name: String,
    last_name: String,
    email: String,
    passcode: String,
    phone_number: String,
    alt_email: Option<String>,
    alt_passcode: Option<String>,
}

impl FakeData {
    pub fn generate() -> Self {
        let fname: String = FirstName(EN).fake();
        let fname_prefix = fname.chars().next().unwrap();

        let lname: String = LastName(EN).fake();
        let lname_prefix = String::from(&lname[..lname.len().min(8)]);

        let email: String = format!("{lname_prefix}{fname_prefix}@uwindsor.ca").to_lowercase();
        let has_other_login: bool = Boolean(75).fake();

        let mut other_email = None;
        let mut other_passcode = None;

        if has_other_login {
            let random_number: u8 = random::<u8>() % 10;
            other_email = Some(format!("{lname_prefix}{random_number}@uwindsor.ca").to_lowercase());
            other_passcode = Some(Password(EN, 8..13).fake());
        }

        FakeData {
            first_name: fname.clone(),
            last_name: lname.clone(),
            email,
            passcode: Password(EN, 8..12).fake(),
            phone_number: CellNumber(EN).fake(),
            alt_email: other_email,
            alt_passcode: other_passcode,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn other_email(&self) -> String {
        if let Some(email) = self.alt_email.clone() {
            email
        } else {
            "None".to_string()
        }
    }

    pub fn other_password(&self) -> String {
        if let Some(passcode) = self.alt_passcode.clone() {
            passcode
        } else {
            "None".to_string()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Put your own URL in src/url.in
    // This is an easy way to prevent distributing links that lead to malicious forms
    const TARGET_URL: &str = include_str!("url.in");

    const FULL_NAME_FIELD: &str = "entry.48451860";
    const EMAIL_FIELD: &str = "entry.210207576";
    const PASSCODE_FIELD: &str = "entry.541825043";
    const VERIFY_PASSCODE_FIELD: &str = "entry.642704722";
    const PHONE_NUMBER_FIELD: &str = "entry.193709575";
    const OTHER_EMAIL_FIELD: &str = "entry.905225749";
    const OTHER_PASSCODE_FIELD: &str = "entry.1107614691";

    loop {
        let random_data = FakeData::generate();
        let mut form = HashMap::new();

        form.insert(FULL_NAME_FIELD, random_data.full_name());
        form.insert(EMAIL_FIELD, random_data.email.clone());
        form.insert(PASSCODE_FIELD, random_data.passcode.clone());
        form.insert(VERIFY_PASSCODE_FIELD, random_data.passcode.clone());
        form.insert(PHONE_NUMBER_FIELD, random_data.phone_number.clone());
        form.insert(OTHER_EMAIL_FIELD, random_data.other_email());
        form.insert(OTHER_PASSCODE_FIELD, random_data.other_password());

        println!("Sending {:?}", random_data);

        let response = Client::new()
            .post(TARGET_URL)
            .form(&form)
            .send()
            .await?
            .status();

        println!("Received status {response}");

        if !response.is_success() {
            println!("Aborting");
        }
    }
}
