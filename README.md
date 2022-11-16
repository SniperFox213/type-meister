# type-meister
New approach to maintain shared types in a complex monorepos


### ToDo

1. Rewrite parser. We need to get rid of Parser struct, and move back to parse_tokens function. We also need not to skip Whitespace characters - we need to properly parse them. And this (not skipping whitespaces) will require a looot of code rewriting.