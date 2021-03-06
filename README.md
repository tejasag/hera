# `hera`

Hera is a simple programming language made to learn more about interpreters and compilers.

![build status](https://github.com/tejasag/hera/actions/workflows/build.yml/badge.svg)
![test status](https://github.com/tejasag/hera/actions/workflows/test.yml/badge.svg)

## Usage

```sh
# To run a .hera file
hera run <filename>.hera

# To open the REPL
hera
```

## Syntax

### Variable declaration

> **All declaration statements should end with a `;`!**

```
let name = value;
```

```
# strings
let a = "hi";

# boolean
let b = true;

# integer
let c = 10;
```

### Updating variables

```
update name = value;
```

```
let a = 10;
update a = 15;
```

### Arrays

```
# declaration
let arr = [1,2,3,4];

# Accessing elements
arr[0] # --> 1
arr[1] # --> 2
arr[-1] # --> 4
```

### Hashes

```
let hash = {
    "one": "Hello",
    "two": "World!",
    3: "number as key",
    3+4: "my key is 7",
    true: "my key is a boolean"
};

hash["one"] # "Hello"
hash[3] # "number as key"
hash[7] # "my key is 7"
hash[true] # "my key is boolean"

```

### Functions

```
let name = fn(<params>) {
    <body>
};
```

```
let double = fn(x) {
    x * 2
};

double(10) # 20
```

You can directly put the expression or value to return **without any keyword or semicolon** or use `return x * 2;`

### Conditions

```
if (<condition>) {
    <body>
}

else {
    <body>
}

else if (<condition>) {
    <body>
}
```

```
if (3 > 4) {
    print("wtf");
} else if (4 > 3) {
    print("correct lol");
} else {
    print("idk man");
}
```

### Imports

The only library currently available is `std`

```
import <lib>;
```

```
import std;
```

## BuiltIn Functions

`print(argument)` - Prints the argument on the screen <br>
`push(array, value)` - Inserts a value in an array <br>
`tail(array)` - Returns a new array without the first element of the given array <br>
`len(argument)` - Returns the length of a string or an array

## `std` library

`map(array, function)` - Runs the function on all elements of the given array and returns a new array with the returned values from the function

Example:

```
let array = [2,3,4,5];
let double = fn(x) {
    x \* 2
};

let new = map(array, double);

# new = [4,6,8,10]
```

`while(condition, body)` - A while loop that runs the body till the condition is true

Example:

```
let i = 1;

let condition = fn() { i < 15 };
while(condition, fn() {
    print(i);
    update i = i+1;
});
```

`range(limit)` - Returns an array with numbers from 1 to limit <br>
`abs(num)` - Returns a positive variant of the given negative or postive number <br>
`first(array)` - Returns the first element of an array <br>
`last(array)` - Returns the last element of an array
