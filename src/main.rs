use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn sort_works(t : &mut Table)
{
    for (_artist, works) in t
    {
        works.sort();
    }
}

fn show(t : &Table)
{
    for (artist, works) in t
    {
        println!("Works by {}:", artist);
        for w in works
        {
            println!("    {}", w);
        }
    }
}

fn main()
{
    let mut table = Table::new();
    
    table.insert("Gesualdo".to_string(),
                 vec!["Many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
                      
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The calling of St Matthew".to_string()]);
                      
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(),
                      "A salt cellar".to_string()]);
                      
    show(&table);
    sort_works(&mut table);
    show(&table);
}
