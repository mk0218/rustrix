Only integer items are available currently.

## Initialization
```Rust
let mx = mx![
    1, 2, 3;
    4, 5, 6;
];
```
```Rust
let (rows, cols, initial_value) = (2, 3, 1);
let mx = mx!(rows, cols; initial_value);

// 1 1 1
// 1 1 1
```

## Add
```Rust
let m1 = mx!(3, 3; 2);
let m2 = mx!(3, 3; 3);
let mx = m1 + m2;

// 5 5 5
// 5 5 5
// 5 5 5
```

## Subtract
```Rust
let m1 = mx!(3, 3; 2);
let m2 = mx!(3, 3; 3);
let mx = m1 - m2;

// -1 -1 -1
// -1 -1 -1
// -1 -1 -1
```

## Dot product
```Rust
let m1 = mx![
    1, 1, 1;
    2, 2, 2;
];

let m2 = mx![
    1, 1, 1, 1;
    2, 2, 2, 2;
    3, 3, 3, 3;
];

let mx = m1 * m2;

//  6  6  6  6
// 12 12 12 12
```

## Transpose
```Rust
let mx = mx![
    1, 2;
    3, 4;
    5, 6;
];

let tp = mx.tp();

// 1 3 5
// 2 4 6
```
