# knowtilus::PROTOTYPE
knowtilus (noʊ-TILL-uhs) - noun : A sea creature of exceptional navigational skills specializing in knowledge.

knowtilus is a simple search engine indexer for your desktop.
By leveraging AI and statistical methods, knowtilus can create a database within a directory for quick and easy search in the future.

:runner: **Quick Start**
1. Install miniconda
2. Create a new conda environment
3. Install the required packages
4. Run the app
```bash
python3 crawl.py <path/to/your/dataset>
```

This will create a database in your current working directory with the name `database.json`.

5. Search for a query
```bash
python3 search.py <query>
```




Notes:
- Use tf-idf as a search method. https://www.freecodecamp.org/news/how-to-process-textual-data-using-tf-idf-in-python-cd2bbc0a94a3/