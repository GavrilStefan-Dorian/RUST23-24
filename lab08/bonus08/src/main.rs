use {anyhow::Result, serde_derive::Deserialize, std::io};

#[derive(Debug, Deserialize)]
struct ListAPI {
    results: Vec<SpellInfo>,
}
#[derive(Debug, Deserialize)]
struct SpellInfo {
    name: String,
    url: String,
}
#[derive(Debug, Deserialize)]
struct Details {
    level: isize,
    desc: Vec<String>,
}

fn main() -> Result<()> {
    let body = ureq::get("https://www.dnd5eapi.co/api/spells")
        .call()?
        .into_string()?;

    let head = String::from("https://www.dnd5eapi.co");

    let mut input = String::new();

    println!("Type a word to lookup:");

    io::stdin().read_line(&mut input)?;

    input.remove(input.len() - 1);

    input = input.to_ascii_lowercase();

    let str = input.trim();

    let spelllist: ListAPI = serde_json::from_str(&body).unwrap();

    let mut count = 0;
    for i in spelllist.results {
        if i.name.to_ascii_lowercase().contains(str) {
            count += 1;
            println!("{}. Name: {}", count, i.name);

            let urldetails = head.clone() + &i.url;
            let details_json = ureq::get(&urldetails).call()?.into_string()?;

            let details: Details = serde_json::from_str(&details_json).unwrap();

            println!("Level: {}", details.level);

            let mut strdetails = String::new();

            for word in details.desc {
                strdetails.push_str(&word);
            }
            println! {"Description: {}",strdetails};
            println!();
        }
    }

    Ok(())
}
