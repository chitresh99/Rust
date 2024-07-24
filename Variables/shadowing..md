**Shadowing in rust**

Shadowing refers to the practice of declaring a new variable with the same name as an existing variable in the same scope. This effectively "shadows" the original variable, meaning that the new variable temporarily hides the original one within its scope.

So basically in the immuatable/immuatable case we add let only while declaring the original variable and later we just type the variable name 

Case of muttable and immuatable
```
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

```
Case of shadowing, in shadowing we have to give let and have the same variable name if we wanna shadow

```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

```
