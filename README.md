# type-meister

### Overview

Type Meister *(name will most likely be changed)* is a small but pretty complicated (on the inside) command-line tool, that allows you to write cross-language interfaces, enums and so on using type meister's readable and pretty easy syntax. You only need to write types once and type meister's cli will generate all needed class, interface and enums definitions in every major programming language *(planned support of TypeScript, Rust and Dart; more to come)*

### Examples

Examples of currently-working .typm (type meister's file format) files:

1. Basic interface with constant value and built-in enum
```
interface User {
    required id: String;
    optional username: String;
    required status: User.Status;
    required const_variable: String { "Constant value, yeehaaaw!!!" };

    enum Status {
        REGISTERED;
        BANNED;
    };
};
```

2. Basic interface with built-in interface
```
interface Parent {
    interface Children {
        ...
    };
};
```

### Todo

- [x] Rewrite parser. We need to get rid of Parser struct, and move back to parse_tokens function. We also need not to skip Whitespace characters - we need to properly parse them. And this (not skipping whitespaces) will require a looot of code rewriting.
- [ ] Rust-like errors
- [x] Rewrite/Optimize entities parsing functions and make them more readable
    - [x] Refactor interface parsing functions
    - [x] Refactor interface's variables parsing functions
    - [x] ...string type parsing functions
    - [x] ...enum parsing functions
    - [x] ...variable type parsing functions
- [ ] Allow enum variants without value declaraation    
    For example:
    ```
    enum TestEnum {
        BANNED;
        ONE_LINE;
    }
    ```
    Because currently you must specify enum value.
- [ ] Add boolean type
- [ ] Add number type
- [ ] Interface/Variable metadata   
    Something like
    ```
    #rename_all = camelCase
    #custom_meta_property = "String const!!!"
    interface User {
        required is_registered: Boolean;
    }
    ```
    Either to store custom information, or to provide additional information and settings
    to codegen. 

    In this example, `rename_all` meta-property is codegen-specific - it'll change all interface's variable names to camelCase. In this example, generated User interface's variable `is_registered` will become `isRegistered`.
- [ ] Write codegen abstract class
- [ ] Write first TypeScript codegen
- [ ] Write normal CLI
- [ ] VSCode Syntax Highlighting extension
- [ ] VSCode Linter

### Future plans

- [ ] Comments  
- [ ] Extendable interfaces  
- [ ] Default values  
- [ ] Method functions, constructors *(very complicated thing, I'll most likely do it in, like, 3000 years)*  
    Example of methods:  
    ```
    interface Transformer_Test {
        required day: Number;
        required month: Number;
        required year: Number;

        method Date() -> String {
            // Idk what language is this, this is just a concept
            let day = this.day;
            let month = this.month;
            let year = this.year;

            if length(day) == 1 {
                day = "0" + day;
            };

            if length(month) == 1 {
                month = "0" + month;
            };

            return format("{}.{}.{}", day, month, year);
        };

        // Or abstract methods??? Idk
        abstract method Create();
    };
    ```

    Example of constructors:  
    *Imagine that the data for this interface is stored in an array in which the first value is the id, the second is the username, and so on. For this case, we make a special constructor that takes this array and rearranges all the values.*  
    ```
    interface Constructor_Test {
        required id: String;
        required username: String;

        constructor(data: Array of String) {
            // This'll probably be a functional-only language with a bunch of built-in functions like
            // len(), element() - to get element of array and so on
            this.id = element(data, 0);
            this.username = element(data, 1);
        };
    };
    ```
