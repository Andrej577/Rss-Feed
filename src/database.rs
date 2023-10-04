use std::error::Error;
use reqwest::Client;
use rss::Channel;
use std::fs::File;
use std::io::{Write, BufReader};

use rusqlite::{Connection, Result};

use regex::*;

use htmlentity::entity::{decode,ICodedDataTrait};

pub async fn request_and_save() -> Result<(), Box<dyn Error>> 
{

    println!("Pokrecem funkciju request_and_save()");

    let client = Client::new();
    let response = client.get("https://medium.com/feed/tag/programming")
        .send()
        .await?;

    let content = response.bytes().await?;

    let mut file = File::create("rss.xml")?;
    file.write_all(&content)?;

    println!("Sadržaj uspešno sačuvan u rss.xml");
    
    let feed_url = File::open("rss.xml").unwrap();
    let channel = Channel::read_from(BufReader::new(feed_url))?;

    let conn = baza();
    
    for item in channel.items() 
    {
        //QUERRY KOJI UPISUJE SAMO AKO NE POSTOJI VEĆ VRIJEDNOST KOLONE TITLE
        conn.execute("INSERT INTO Rss (title, description, link, date) SELECT ?1, ?2, ?3, ?4 WHERE NOT EXISTS (SELECT 1 FROM Rss WHERE title = ?1);", 
            [item.title(), item.description(), item.link(), item.pub_date()]).expect("Greska prilikom okidana kverija");
    }
    println!("Zavrsio funkciju request_and_save()");

    Ok(())
}


fn baza() -> Connection
{
    println!("Pokrecem funkciju baza()");
    let conn = Connection::open("baza.db").expect("Failed to create and open baza.db");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Rss(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT UNIQUE,
            link TEXT,
            description TEXT,
            date TEXT);",[]
    ).expect("Nisam uspio kreirat bazu");

    println!("Zavrsio funkciju baza()");

    conn 
}

pub struct RssItems
{
    pub(crate) title: String,
    pub(crate) link: String,
    pub(crate) description: String,
    pub(crate) date: String
}

pub fn print_from_db() -> Vec<RssItems>
{
    println!("Pokrecem funkciju print_from_db()");

    let conn = Connection::open("baza.db").unwrap();

    let mut stmt = conn.prepare("SELECT title, link, description, date FROM Rss ORDER BY date DESC LIMIT 10").unwrap();

    let rss_items: Vec<RssItems> = stmt.query_map([], |row| 
        Ok(RssItems
            {
                title: row.get(0).unwrap(), link: row.get(1).unwrap(), description: regex(row.get(2).unwrap()), date: row.get(3).unwrap()
            }
        )).unwrap().collect::<Result<_>>().unwrap();

    println!("Zavrsio funkciju print_from_db()");

    rss_items
}

fn regex(item2: String) -> String
{
    let re = Regex::new(r"<[^>]+>").unwrap();
    let mut combined_description = String::new();

    for fragment in re.split(&item2) {
        let cleaned_fragment = fragment.trim();
        if !cleaned_fragment.is_empty() && cleaned_fragment != "]]>" {
            // Dekodiraj HTML escape sekvence u fragmentu
            let bytes = cleaned_fragment.as_bytes();
            let decoded_fragment = decode(bytes);
            
            // Kreiraj vlasničku kopiju dekodiranog fragmenta kao String
            let decoded_string = String::from_utf8_lossy(&decoded_fragment.to_bytes()).to_string();

            combined_description.push_str(&decoded_string);
        }
    }

    combined_description
}


