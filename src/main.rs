fn main() {
    println!("Hello, world!");
    println!("Box");
    {
        let t1 = (3, "birds".to_string());
        let mut b1 = Box::new(t1); //Boxポインタ タプルがヒープに移動
        (*b1).0 += 1;
    }
    println!("Vec");
    {
        let mut v1 = vec![1, 3];
        v1.push(4);
        println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
        v1.shrink_to_fit();
        let s1 = v1.into_boxed_slice();
        let v2 = s1.into_vec();
        println!("v2 len: {}, capacity: {}", v2.len(), v2.capacity());
    }
    println!("String");
    {
        let mut a: String = "aaaa".to_string();
        a.push_str("bbb");
        println!("{}", a);
    }
    println!("Option");
    {
        let o1 = Some(10);
        // マッチで外し
        match o1 {
            Some(s) => assert_eq!(s, 10),
            None => unreachable!(),
        }
        let o2 = Some("a");
        // Swiftっぽいif letでOptional外し
        if let Some(s) = o2 {
            println!("{}", s);
        }
    }
    println!("早期リターン");
    {
        // 早期リターンとmapとか
        fn add1(s0: &str, s1: &str) -> Result<i32, String> {
            let s0 = s0.parse::<i32>().map_err(|_e| "s0")?;
            let s1 = s1.parse::<i32>().map_err(|_e| "s1")?;
            Ok(s0 + s1)
        }
        assert_eq!(add1("3", "abc"), Err("s1".to_string()));
    }
    println!("型エイリアス");
    {
        type UserName = String;
        type Id = i64;
        type User = (Id, UserName);
        fn new_user(name: UserName, id: Id) -> User {
            (id, name)
        }
        let u = new_user("aaaa".to_string(), 3);
        println!("{} {}", u.0,u.1);
    }
    println!("構造体");
    {
       struct Polygon {
           vertexes: Vec<(i32,i32)>,
           stroke_width: u8,
           fill: (u8,u8,u8),
       }
       struct Triangle(Vertex,Vertex,Vertex);
       struct Vertex(i32, i32);
    }
    println!("列挙型");
    {
        #[derive(Debug,PartialEq)]
        enum Weekday {
            Monday,Tuesday,Wednesday,Thursday,Friday
        }
        type UserName = String;
        #[derive(Debug)]
        enum Task {
            Open,
            AssignedTo(UserName),
            Working {
                assignee: UserName,
                remaining_hours: u16,
            },
            Done,
        }
        println!("型強制");
        {
            let v1:Vec<u8> = vec![3,4,5];
            assert_eq!(Some(&3u8),v1.first());
        }


    }
}
