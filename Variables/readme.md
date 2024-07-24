**Variables in rust**

1) Immutable variables 

In rust once you assign a value to a variable it's fixed

To be able to change the value you have to add the word 'mut' before the variable name

2) Constants
Like immutable variables, constants are values that are bound to a name and are not allowed to change

We can't use mut with constants

To declare a constant we use the 'const' keyword

Also another rule is we have to name the variables in all uppercase

Also we have to specify the type like this for an unsigned integer `: u32`

Code :- 

```
fn main () {
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
}

```