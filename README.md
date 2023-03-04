# While Programming Language

An implementation of the [While Language](#what-is-the-while-programming-language) written in rust. 
It is also possible to run **For** programs, which are very similar. 

## Usage
1. Clone the repo. 
```git
git clone https://github.com/FelixReinhard/WhileLang.git     
```
2. Ensure that the [rust](https://www.rust-lang.org/tools/install) compiler is installed

3. Move into the folder 
```
cd WhileLang
````

4. Run the interpreter using cargo. One can use the the shell using **shellf** or **shellw** or execute a file directly ending in eighter **.f** ofr **.w**
```
cargo run --release run test.w
```

5. (Optional) The executeble will be located in **/target/release**. You can add them to your bin folder if desired.

## Examples

1. Calculate the *fibonacci* numbers.

```
in x_0
x_11 = 1
x_10 = 0

x_20 = x_0 + x_10
x_1 = 0
x_2 = 1
x_3 = 0
x_4 = 0

while x_20 != 0 Do 
    x_4 = 2
    x_5 = x_0 - x_4
    while x_5 != 0 Do 

        x_3 = x_1 + x_2 
        x_1 = x_2 + x_10 
        x_2 = x_3 + x_10 

        x_4 = x_4 + x_11
        x_5 = x_0 - x_4
    od 
    x_0 = x_2 + x_10
    x_20 = 0
Od

out x_0
```

## Syntax 
The **While Language** has 6 statements.
- Addition 
```
  x_0 = x_1 + x_2 
```
- Subtraction: 
```
  x_0 = x_1 - x_2
```
- Assignment:
```
  x_0 = 0
```
- While loop:
```
  while x_0 != 0 Do 
      ...
  Od
```
- Input:
```
  in x_0 
```
- Output:
```
  out x_0 
```
       
The **For Language** has the same statements, except to *While loop*. Instead it has the
- For Loop
```
  for x_0 Do 
      ...
  Od
```
## What is the While programming language
The **While Language** is a formal model of a simple programming Language. It is mostly used in a theoretical environment  to prove *computability* related topics. There is only one loop, the while loop, which repeats the statements inside until the variable in the condition becomes zero.

There is also the **For Language**, where the only difference is that the while loop is replaced by a for loop. This loop repeats a set number of times when reaching the first point of execution.

## Why does this exist? 
There is no good reason for this implementation to exist. I would not recommend using this for anything.

