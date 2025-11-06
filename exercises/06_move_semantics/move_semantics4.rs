fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut w = Vec::new();

    for i in v.iter().rev() {
        w.push(i);
    }

    w.sort();

    println!("{:?}", w);
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x; // here y gets deleted out of scope, as z = y (= &mut x)
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}

/*
    Low level explaination
    
    ```rs
    1   let mut x = Vec::new();
    2   let y = &mut x;
    3   let z = &mut x; 
    ```

    instruction at 0:
        - stack is empty 
        - heap is empty

    instruction at 1:
        - space is allocated in stack for the mutable x, which contains the following
            - pointer to the data (constant) dangling to a safe space
            - space to store length (constant)
            - space to store capacity (constant)
        - space is allocated in heap memory to store the actual data which the stack pointer points to

    instruction at 2:
        - space is allocated in stack for the mutable y
        - y points to the same heap location as x 

    ...got lost
*/
