# Module Outline

## Steps to Build

1. Call command `build-outline --path ./book`
2. Find source directory
   1. dynamic?
   2. hardcoded? `<path>/src`
3. Filter files
   1. Only chapter files `/ch\d\d-.+.md/`
   2. Not `ch00`
4. For each file
   1. Extract lines that are headings
   2. Write to a new file in `dest`
      1. Name derived from main heading (1 #)
      2. For each Heading
         1. Write Heading
         2. One blank line
