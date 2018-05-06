use std::collections::HashMap;

// Weird, I know.
// STASH, a global reference to an int32,
// is a reference to the constant 128.
// The syntax feels a little odd.
// Particularly the location of the "static mut"
// relative to the rest of the type.
static mut STASH: &i32 = &128;
static VALUE: i32 = 256;

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

// A prop for a later demonstration
fn factorial(n: usize) -> usize
{
    (1 .. n+1).fold(1, |a, b| a * b)
}

// The rules about lifetimes of references mean that
// if you're going to assign to a static reference, you
// need to explicitly name the lifetimes
fn danger(p : &'static i32)
{
    unsafe
    {
        STASH = p;
    }
}

// Rust assumes that if there is one argument and a return
// value, then they have the same lifetime. You don't have
// to write it out explicitly like this:
fn smallest<'a>(v : &'a [i32]) -> &'a i32
{
    let mut tmp = &v[0];
    for r in &v[1..]
    {
        if *r < *tmp
        {
            tmp = r;
        }
    }
    tmp
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

    // Comparisons are also magic. They see the thing being compared, so long as they have the same type
    {
        let p1 : i32 = 729;
        let r1 : &i32 = &p1;
        let rr1 : &&i32 = &r1;
        let rrr1 : &&&i32 = &rr1;
        
        let p2 : i32 = 730;
        let r2 : &i32 = &p2;
        let rr2 : &&i32 = &r2;
        let rrr2 : &&&i32 = &rr2;
        
        assert!(rrr1 < rrr2);
        
        // Does it work for unequal levels of references?
        
        assert!(rr1 < rrr2);
        // sure does!
    }
    
    // The ampersand can be applied in all manner of syntactic constructs
    {
        let r = &factorial(6);
        assert_eq!(r, &720);
    }
    
    // But be aware of the specificity required of lifetimes
    danger(&VALUE);
    
    // In the following, s gets the lifetime of parabola because of the
    // way that smallest is defined, but the explicit definition of
    // lifetimes is only meant to highlight the default behaviour.
    // It will fail if s is defined out of the inner scope.
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(*s, 0);
    }
}
