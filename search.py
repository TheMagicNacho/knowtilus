import json
import logging
import pandas as pd
import sys
import os
import numpy as np
from tabulate import tabulate
from transformers import pipeline
import torch
from sentence_transformers import SentenceTransformer
import re

logging.basicConfig(level=logging.DEBUG)

if len(sys.argv) != 2:
    print('Usage: python search.py <directory>')
    sys.exit(1)
directory = sys.argv[1]


def vectorize(text):
    # REF: https://huggingface.co/sentence-transformers/all-mpnet-base-v2
    sentences = text.lower().replace('\n', '.').replace('!', '.').replace('?', '.').split('.')
    model = SentenceTransformer('sentence-transformers/all-mpnet-base-v2')
    embeddings = model.encode(sentences)
    return embeddings


db = {}

database = os.path.join(directory, 'knowtilus.db')
with open(database, 'r') as file:
    string = file.read()
    db = json.loads(string)
    file.close()
    logging.info("Database loaded")
    # logging.debug(db)

matrix = pd.DataFrame()

# set the matrix colomuns as filename, summary, frequency, vectorization, keywords, score
matrix['filename'] = db.keys()
matrix['summary'] = [db[filename]['summary'] for filename in db.keys()]
matrix['frequency'] = [db[filename]['frequency'] for filename in db.keys()]
matrix['vectorization'] = [db[filename]['vectorization'] for filename in db.keys()]
matrix['keywords'] = [db[filename]['keywords'] for filename in db.keys()]
# matrix['title'] = [db[filename]['title'] for filename in db.keys()]
matrix['score'] = 0
# TODO: Make each score a column the create a weighted average of the scores

 # vector, keywords, summary, frequency, file_name
score_weights = [95, 93, 61, 83, 52]

def search(search_string):
    user_input = search_string.lower().split()
    user_input.append(search_string.lower())

    for search_input in user_input:
   
        search_vector = torch.tensor(vectorize(search_input))

        for index, row in matrix.iterrows():
            summary_score = 0
            keywords_score = 0
            freq_score = 0
            vectorization_score = 1
            filename_score = 0

            if re.search(search_input, row['filename'], re.IGNORECASE):
                filename_score = 1


            summary = row['summary']
            if re.search(search_input, summary, re.IGNORECASE):
                summary_score = 1

            frequency = row['frequency']
            freq_score = frequency[search_input] if search_input in frequency else 0
      
            keywords = row['keywords']
            for keyword in keywords:
                if re.search(search_input, keyword, re.IGNORECASE):
                    keywords_score += 1

            vectorization = row['vectorization']
            for vector in vectorization:
                vector = torch.tensor(vector)
                cosine = torch.nn.functional.cosine_similarity(vector, search_vector)
                if cosine > 0.53:
                    vectorization_score += 1

            
            score_array = [vectorization_score, keywords_score, summary_score, freq_score, filename_score]

            final_score = np.average(a =score_array, weights= score_weights)
            # matrix.at[index, 'score'] += vectorization_score
            # matrix.at[index, 'score'] += summary_score
            # matrix.at[index, 'score'] += keywords_score
            # matrix.at[index, 'score'] += freq_score

            matrix.at[index, 'score'] = final_score

def main():
    search_string = ""
    search_string = input("Search: ")
    search(search_string)
    print("KNOWTILUS SEARCH RESULTS")
    res = matrix.sort_values(by='score', ascending=False).head(10)
    print(tabulate(res[['score', 'filename', 'keywords']], headers='keys', tablefmt='psql', showindex='never'))


    sumarize = input("Do you want to see the summary of the top result? (y/n): ")
    if sumarize == 'y':
        top_result = matrix.sort_values(by='score', ascending=False).iloc[0]
        print("Summary of the top result:")
        print(top_result['summary'])



while True:
    main()


