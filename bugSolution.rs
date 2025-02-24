fn main() {
    let mut v = vec![1, 2, 3];
    // Safe way to modify the vector
    v[0] = 10; 
    println!("{:?}", v);
    
    //Alternative using iterators
    let mut v2 = vec![1,2,3];
    for i in v2.iter_mut(){
        *i = *i * 2;
    }
    println!("{:?}",v2);
} 