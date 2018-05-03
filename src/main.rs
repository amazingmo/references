use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

/// t has to be a &mut type, otherwise you can't change it
fn sort_works(t : &mut Table)
{
    for (_artist, works) in t
    {
        works.sort();
    }
}

///  If you don't need to change it, just put
/// ampersand and it is a borrrow that doesn't take
/// ownership, but doesn't permit modification either.
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
    
    // -------
    
    // You can create references that act like aliases
    
    let x = 10;
    let r = &x;         // implicit type deduction, preferred style. Thinking in C++ is a problem because the '&' is part of a type, so it's "type deduction but change something."
    assert!(*r == 10);  // explicit dereference
    
    let mut y =32;
    {
        let m = &mut y;     // ditto about the weird partial type deduction
        *m += 32;
    }
    assert!(y == 64);
    
    let mut z = 0;
    {
        let mut m = &mut y;
        m = &mut z;     // This would be impossible in C++.  pointing m at a different value.
        *m += 10;
    }
    assert!(y == 64 && z == 10);
    
    // ------
    
    // The dot operator works equally well on objects and on references, which it
    // automatically deferences or borrows and dereferences as required.
    
    let mut v = vec![1973, 1968];
    v.sort();  // otherwise this wouldn't work.
    
    // ------
    
    // The dot operator is magic. It collapses any number of layers of references.
    {
        struct Point { x: i32, y: i32}
        let p = Point{x: 1000, y: 729};
        let r : &Point = &p;
        let rr : &&Point = &r;
        let rrr : &&&Point = &rr;
        assert_eq!(rrr.y, 729);
    }
}
