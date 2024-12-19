### Explications de mon point de vue, mes recherches.

As I am starting to work on the parsing step, I am doing some analysis on which way to implement the expression handling, I've come across two different ways as of now. The first one would be an enum holding a value for each kind of expressions possible (variables, arithmetic expressions, function calls, ...), last time i built this project in C, i used a struct that had an enum value field for the expression kind and a union for handling each one of the possibilities.

This time in rust, two ways have crossed my mind. Using an enum with embedded structs and having a struct with optionnal values for each kind of expressions.
First the enum, this way is easy to use and keeps the code clear but i have read that the enum type would have a size equal to the maximum size of one of the embedded structs.
Second choice is using structs with Option<> values.

### References
[Make a langage](https://lunacookies.github.io/lang/)
[Rust analyzer lectures](https://www.youtube.com/channel/UCLd3PQ6J0C-VuNBozsXGUWg/playlists)

Bon pour expliquer rapidement le fonctionnement des choses mais il reste nécéssaire de vérifier les informations fournies.
[ChatGPT](https://chatgpt.com/)

### Explication générée par ChatGPT 
tout ce qui suit n'est pas de moi et n'est parfois pas vérifié, seulement utilisé comme moyen de compréhension rapide.

Je suis basé sur ChatGPT, un modèle d'intelligence artificielle développé par OpenAI, spécifiquement une version améliorée et récente (décembre 2024). Mon entraînement repose sur une vaste collection de textes provenant de sources publiques, y compris des livres, des articles, de la documentation technique et des discussions en ligne, disponibles jusqu'à une certaine date (octobre 2023). Voici quelques détails sur la manière dont les réponses ont été générées :

Modèle Utilisé
Base : GPT-4.5 Turbo ou une version similaire.
Domaine d'application : Optimisé pour comprendre et expliquer des concepts techniques, y compris la programmation, les structures de données, et la conception de compilateurs.
Capacités :
Fournir des explications détaillées basées sur des principes de base de la programmation.
Faire des analyses précises de la mémoire et des performances.
Proposer des exemples clairs et adaptés aux bonnes pratiques.
Sources de Connaissances
Les réponses que je génère sont construites à partir de connaissances générales intégrées, qui proviennent de :

Documentation Rust :
Rust Book.
Rust Reference.
Rust API Documentation.
Communauté Rust :
Discussions sur des forums publics comme Rust Users Forum et des échanges Stack Overflow.
Bases informatiques générales :
Principes de conception des compilateurs (tirés de manuels académiques).
Structures de données et optimisations en mémoire.
Absence de Sources Dynamiques
Je n'ai pas accédé à des sources dynamiques ou en temps réel pour répondre à vos questions dans cette session. Si vous avez des références ou des sources spécifiques que vous aimeriez voir utilisées, je peux vous aider à les analyser et intégrer dans nos échanges.

### Détails des différentes approches

We have two types of expressions:

ArithmeticExpression: Simple operations like 1 + 2.
Requires 8 bytes (e.g., two i32 operands and a small discriminant for the operation type).
VariableExpression: Holds a variable name and its value.
Requires 32 bytes: 24 bytes for a String (on a 64-bit system: 8 bytes for pointer, 8 bytes for length, 8 bytes for capacity) and 8 bytes for an i32 value.

```
enum Expression {
    ArithmeticExpression(i32, i32), // 8 bytes
    VariableExpression(String, i32), // 32 bytes
}
```

Memory Allocation:

The size of the Expression enum is determined by its largest variant, VariableExpression, which is 32 bytes.
This means every instance of the Expression enum occupies 32 bytes, even for ArithmeticExpression.
Waste:

For ArithmeticExpression, which requires only 8 bytes, this results in 24 bytes wasted per instance.
Example Usage: Suppose you parse a program with:

100 ArithmeticExpression instances (only need 100 × 8 = 800 bytes).
10 VariableExpression instances (10×32=320 bytes).
Total memory used: (100+10)×32=3520 bytes

Case 2: Optimized Design with Boxing
```
enum Expression {
    ArithmeticExpression(i32, i32), // 8 bytes
    VariableExpression(Box<Variable>), // Pointer (8 bytes)
}

struct Variable {
    name: String, // 24 bytes
    value: i32,   // 8 bytes
}
```

Memory Allocation:

Expression now uses only 8 bytes for the discriminant and variant pointer, regardless of the variant.
The VariableExpression data is stored on the heap:
8 bytes for the pointer in Expression.
32 bytes for the Variable struct on the heap.
Example Usage:

100 ArithmeticExpression instances (100×8=800 bytes).
10 VariableExpression instances:10×8=80 bytes for enum pointers.
10×32=320 bytes for heap allocations.
Total memory used:800+80+320=1200 bytes.

When Boxing May Not Be Optimal
Boxing introduces an indirection (pointer) and a heap allocation, which can be slower due to:

Cache misses during pointer dereferencing.
Heap management overhead.
If your program mostly uses VariableExpression and rarely uses ArithmeticExpression, boxing might worsen performance and memory usage due to the added indirection. In such a case, embedding the data directly in the enum (Case 1) could be better.

Using structs instead of enums :
Case 1: General-Purpose Expression Struct
Suppose you want to represent multiple expression types (ArithmeticExpression, VariableExpression, etc.) in a single struct using Option fields.

```
struct Expression {
    arithmetic: Option<ArithmeticExpression>,
    variable: Option<VariableExpression>,
}

struct ArithmeticExpression {
    left: i32,
    right: i32,
    operator: char, // '+', '-', etc.
}

struct VariableExpression {
    name: String,
    value: i32,
}

let expr = Expression {
    arithmetic: Some(ArithmeticExpression {
        left: 5,
        right: 10,
        operator: '+',
    }),
    variable: None,
};

let expr = Expression {
    arithmetic: None,
    variable: Some(VariableExpression {
        name: "a".to_string(),
        value: 42,
    }),
};
```

Memory Implications
Memory Allocation:

Option wrapping types like ArithmeticExpression or VariableExpression is efficient because Option::None doesn’t consume extra memory. Rust uses unused discriminant values in the wrapped type.
However, the struct's overall memory size is determined by the sum of its fields.
Drawbacks:

If most fields are unused (None), memory can be wasted unless these fields are rarely used or small.

Case 2: Optimized Struct with Shared Fields
For expressions that share some fields (e.g., an expression always has a type and potentially some metadata), you can use Option only for less common fields.

```
struct Expression {
    expression_type: ExpressionType, // Discriminant for the type
    name: Option<String>,            // Used in VariableExpression
    value: Option<i32>,              // Shared between Variable and Arithmetic
    operator: Option<char>,          // Only used for ArithmeticExpression
}

enum ExpressionType {
    Arithmetic,
    Variable,
}

let expr = Expression {
    expression_type: ExpressionType::Arithmetic,
    name: None,
    value: Some(5),
    operator: Some('+'),
};

let expr = Expression {
    expression_type: ExpressionType::Variable,
    name: Some("a".to_string()),
    value: Some(42),
    operator: None,
};
```

When to Use Options in Structs
Advantages:
Unified Structure: Reduces the need for enums, making it easier to handle different kinds of expressions in a generic way.
Memory Efficiency: Option is memory-efficient when fields are often unused (e.g., most expressions don’t use operator).
Clarity: The None values clearly indicate unused or absent fields.
Disadvantages:
Wasted Memory: If your struct has many Option fields and most of them are unused for a given instance, memory can still be wasted.
Complexity in Validation: You must ensure the correct combination of fields is set for a given expression type (e.g., operator should only be Some for ArithmeticExpression).

Recommendation
Use Option fields when:
Your expressions share many fields, but only a subset is relevant for each type.
You want a single struct to handle multiple cases, avoiding enums or separate structs.
Avoid this approach if:
Most fields will often remain None.
You need tight control over memory or processing performance.
For most compiler-related use cases, enums with boxed variants or disjoint structs are typically better for memory and clarity. Use Option fields sparingly in shared structs to simplify certain designs.