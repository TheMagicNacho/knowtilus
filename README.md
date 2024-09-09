# knowtilus::PROTOTYPE
knowtilus (noʊ-TILL-uhs) - noun : A sea creature of exceptional navigational skills specializing in knowledge.
`as a prototype, this is unstable and prone to change. I also plan to re-write in rust`

knowtilus is a simple search engine indexer for your desktop.
By leveraging AI and statistical methods, knowtilus can create a database within a directory for quick and easy search in the future.

:runner: **Quick Start**
1. Install miniconda
2. Create a new conda environment
3. Install the required packages
4. Run the app
```bash
python3 crawl.py <path/to/your/corpus>
```

This will create a database in your current working directory with the name `database.json`.

5. Search for a query
```bash
python3 search.py <path/to/your/corpus>
```

🔍 ** Searching **
When searching, use the lowest root word possible. For example, if you are searching for "running", use "run" instead.
This is because features are matched against fuzzy matches of the query.

This is an imperfect knowledge engine, searching for long questions will create a lot of noise `mo input mo problems`. To combat this, try to keep queries short and to the point. In the future, I plan to implement a more robust qurey augmentation.

Knowtilus is great at finding concepts within a body of knowledge. For example: "Cell Growth" or "Attomic Wells".



Notes:
- Use tf-idf as a search method. https://www.freecodecamp.org/news/how-to-process-textual-data-using-tf-idf-in-python-cd2bbc0a94a3/


# knowtilus::TODO
- [ ] Implement a more robust query augmentation
- [ ] Add image classification on crawl


# Lexer
The lexer is a class. "Lex".
Lex contains
- tokens : Words that are found in the corpus and split by spaces. Tokens are lammented to their root word.
- senteces : a list of sentences that are found in the corpus. Helpful for generating summaries and context around a word.
- text : the raw text of the corpus as a string. This is stripped of punctionation.
# Database Features

## Vector
An array of floats that represent the word in a high dimensional space. This is used for similarity matching.

## Keywords
Generated using n-grams and analyzed using the text from lex.

## Filename
Filenames provide a wealth of information about the document. This is used to generate a summary of the document.

## Summary
The summary is AI generated. And if the user searches for a word, and that word is found in the summary a score is provided. 
Testing has proven that AI generated summaries do not work well. Weight scores from a summary should be low.

