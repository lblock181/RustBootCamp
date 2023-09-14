fn main() {

    // Need to define type annotation
    let v = Vec::new();

    // type definition
    let v: Vec<String> = Vec::new();

    // assumed from adding elements
    let mut v = Vec::new();
    v.push(String::from"One");
    v.push(String::from"Two");
    v.push(String::from"Three");
    v.push(String::from"Four");
    v.push(String::from"Five");
    v.push(String::from"Six");

    // similar to defining array
    // easier to use vs pushing into vector as you can populate values at same time
    let v2 = vec![1,2,3];


    // accessing values
    let s = &v2[2]; // can panic if out of bounds & cannot move element out of vector like this
    // let ms = v.remove(0);
    let s2: Option<&int32> = v2.get(2);

    // iterating
    for x in &mut v {
        x.push_str("!");
    }

    let mut v3: Vec<String> = vec![];
    for x in v {
        v3.push(x);
    }

    // above code is same as below just easier to read
    for x in v.into_iter() {
        v3.push(x);
    }
}
