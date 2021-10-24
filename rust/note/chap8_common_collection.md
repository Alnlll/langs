# 8. Common Collections
## 8.1. Vector
Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.

Vectors can only store values of the same type, and they are stored on heap.

### 8.1.1. Creating a New Vector
`let v: Vec<i32> = Vec::new();` create a new empty `i32` vector.

It’s more common to create a `Vec<T>` that has initial values, and Rust provides the `vec!` macro for convenience. 
```
  // create a new vector
  let v: Vec<i32> = Vec::new();
  let v = vec![1, 2, 3];
```

### 8.1.2. Updating a Vector
```rust
  {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v = {:?}", v);
  }
```

### 8.1.3. Dropping a Vector Drops Its Elements
Like any other struct, a vector is freed when it goes out of scope.

### 8.1.4. Reading Elements of Vectors
```rust
  // Reading elements
  {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    match v.get(2) {
      Some(third) => println!("The third elem is {}", third),
      None => println!("There is no the third elem.")
    }
  }
```

### 8.1.5. Iterating over the Values in a Vector
```rust
// iterating elements
  {
    let v = vec![1, 2, 3, 4, 5];
    for e in &v {
      println!("{}", e);
    }
  }
```

### 8.1.6. Using an Enum to Store Multiple Types
```rust
  // Enum to save multiple type
  {
    enum SpreadsheetCell {
      Int(i32),
      Float(f32),
      Text(String),
    }

    let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Float(10.13),
      SpreadsheetCell::Text(String::from("blue"))
    ];
  }
```

## 8.2. Storing UTF-8 Encoded Text with Strings

### 8.2.1. What is a String
 Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`.

When Rustaceans refer to “strings” in Rust, they usually mean the String and the string slice &str types, not just one of those types.

### 8.2.2. Creating a New String
- New
```rust
  {
    let mut s = String::new();
    println!("{}", s);
  }
```
- From string literal
```rust
  // string literal
  {
    let s = "initial contents".to_string();
    println!("{}", s);
    let s = String::from("initial contents - from");
    println!("{}", s);
  }
```
- UTF-8
```rust
  // UTF-8
  {
    let hello0 = String::from("السلام عليكم");
    println!("{}", hello0);

    let hello1 = String::from("你好！");
    println!("{}", hello1);
  }
```

### 8.2.2 Updating a String
- Appending to a String with `push_str` and push
```rust
  // updating a String
  {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("s = {}", s);

    let mut s = String::from("Hello");
    let s1 = ", world!";
    s.push_str(s1);
    println!("s = {}", s);

    let mut s = String::from("lo");
    s.push('l');
    println!("s = {}", s);
  }
```

- Concatennation
  - +
  ```rust
  {
    let s1 = String::from("Hello,");
    let s2 = String::from(" World!");
    let s3 = s1 + &s2;
    println!("s = {}", s3);
    // println!("s = {}", s1); // s1 will be borrowed here, could not be used anymore
  }
  ```
  - format!
  ```rust
  {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // not borrowing happened
    println!("s = {}", s);
    println!("s = {}", s1);
  }
  ```

### 8.2.3. Indexing into Strings

Simply, Rust does not support indexing a `String` because of it is storing UTF-8 encoding.

https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings

### 8.2.4. Slicing strings
Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

The user should be aware of how much length should be taken from the string.

```rust
  {
    let s = String::from("Здравствуйте");
    let s1 = &s[0..4];
    println!("slice = {}.", s1);
  }
```

### 8.2.5. Iterating Over Strings

`chars` returns in char unit, `bytes` returns in byte unit.

```rust
  // iterating
  {
    let s = String::from("नमस्ते");
    for c in s.chars() {
      println!("{}", c);
    }
    for b in s.bytes() {
      println!("{}", b);
    }
  }
```

## 8.3. Storing Keys with Associated Values in Hash Maps
### 8.3.1. Creating a New Hash Map
We could use `new` to create a new hasp map and add elements with `insert`.

```rust
  // new hash map
  {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    println!("scores = {:?}", scores);
  }
```

Another way of constructing a hash map is by using iterators and the `collect` method on a vector of tuples, where each tuple consists of a key and its value.

```rust
  {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];

    let mut scores: HashMap<_,_> =
      teams.into_iter().`zip(init_scores.into_iter()).collect();

    println!("scores = {:?}", scores);
  }
```

### 8.3.2. Hash Maps and Ownership

For types that implement the Copy trait, like i32, the values are copied into the hash map. 

For owned values like String, the values will be moved and the hash map will be the owner of those values.

### 8.3.3. Accessing Values in a Hash Map

- get
```rust
  {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // score = Some(10)
    println!("score = {:?}", score);
  }
```
- Iter
```rust
  {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    for (k, v) in &scores {
      println!("{} = {}", k, v);
    }
  }
```

### 8.3.4. Updating a Hash Map
- Overwriting a Value
  The default behavior is overwriting the old value with new one no matter the key exists in the map or not
  ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 30);
    println!("scores = {:?}", scores);
  ```
- Only Inserting a Value If the Key Has No Value
  
  Hash maps has a special API for this called `entry` that takes the key you want to check as parameter. The return value of `entry` method is an enum called `Entry` that represents a value that might or might not exist.

  The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists.

  ```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores = {:?}", scores);
  ```

- Updating a Value Based on the Old Value
  ```rust
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
    }
    println!("map = {:?}", map);
  ```