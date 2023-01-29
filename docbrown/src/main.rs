use std::{
    ops::Range,
};

fn main() {
    // println!("{:?}", r_opts(0));

    let v = vec![vec![1, 2, 3]];
    let i = v.iter();
    let res = i.flat_map(|v| v);
    
    println!("{:?}", res.collect::<Vec<_>>());

    let v = Some(vec![1, 2, 3]);
    let i = v.iter();
    let res = i.flat_map(|v| v);
    
    println!("{:?}", res.collect::<Vec<_>>());
    
    let v = vec!(vec![1, 2, 3]);
    let i = Box::new(v.iter());
    let res = i.flat_map(|v| v);
    
    println!("{:?}", res.collect::<Vec<_>>());

    let v = vec![1, 2, 3];
    let s = Some(v.iter());
    let x = s.iter();
    let r = x.collect::<Vec<_>>();
    println!("{:?}", r);

    let v = vec![1, 2, 3];
    let d = 
        v.iter()
        .flat_map(|_| Some(v.iter()))
        .flat_map(|x| x.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", d);

    let i =v.iter();
    v.iter().flat_map(|_| i.copied());
}

// fn r_opts(x: i32) -> Option<Box<dyn Iterator<Item=i32>>>{
//     let a = Some(x)?;
//     let v = vec![1,2,3];
//     let b = if a > 0 {Some(Box::new(v.iter().copied()))} else {None};
//     Some(b?)
// }

fn r_opts(v: &'static Vec<i32>) -> Option<Box<dyn Iterator<Item=&'static i32> + 'static>> {
    Some(Box::new(v.iter()))
}

#[test]
fn test_r_opts() {
    let v = vec![0,1,2,3];
    let itr = Box::new(v.iter());
    let res = itr.flat_map (|x| {
        r_opts(*x)
    }).collect::<Vec<_>>();
    
    println!(
        "{:?}",
        res
    )
}
