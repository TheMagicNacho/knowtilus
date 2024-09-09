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
import matplotlib.pyplot as plt

from core import Lex

logging.basicConfig(level=logging.DEBUG)

if len(sys.argv) != 2:
    print('Usage: python search.py <directory>')
    sys.exit(1)
directory = sys.argv[1]

# Load the model and scope it higher to avoid loading it every time
vector_model = SentenceTransformer('sentence-transformers/all-mpnet-base-v2')

def vectorize(text):
    # REF: https://huggingface.co/sentence-transformers/all-mpnet-base-v2
    sentences = text.lower().replace('\n', '.').replace('!', '.').replace('?', '.').split('.')
    embeddings = vector_model.encode(sentences)
    return embeddings

# Sigmoid function which takes a value and a max number then normalizes the value between 0 and 1
def sigmoid(x, max):
    return 1 / (1 + np.exp(-x/max))

db = {}

database = os.path.join(directory, 'knowtilus.db')
with open(database, 'r') as file:
    string = file.read()
    db = json.loads(string)
    file.close()
    logging.info("Database loaded")
    # logging.debug(db)

 # vector, keywords, summary, frequency, file_name
score_weights = [30, 20, 5, 35, 10]

def search(search_string):
    matrix = pd.DataFrame()

    # set the matrix colomuns as filename, summary, frequency, vectorization, keywords, score
    matrix['filename'] = db.keys()
    matrix['summary'] = [db[filename]['summary'] for filename in db.keys()]
    matrix['frequency'] = [db[filename]['frequency'] for filename in db.keys()]
    matrix['vectorization'] = [db[filename]['vectorization'] for filename in db.keys()]
    matrix['keywords'] = [db[filename]['keywords'] for filename in db.keys()]
    matrix['score'] = 0.0


    # user_input = search_string.lower().split()
    # user_input.append(search_string.lower())
    user_input = Lex(search_string)


    for search_input in user_input.get_tokens():
        print("Search Input: " + search_input)

        search_vector = torch.tensor(vectorize(search_input))
        for index, row in matrix.iterrows():
            summary_score = 0
            keywords_score = 0
            freq_score = 0
            vectorization_score = 0
            filename_score = 0

            if re.search(search_input, row['filename'], re.IGNORECASE):
                filename_score += 1


            summary = row['summary']
            if re.search(search_input, summary, re.IGNORECASE):
                summary_score += 1

            frequency = row['frequency']
            freq_score += frequency[search_input] if search_input in frequency else 0

            keywords = row['keywords']
            for keyword in keywords:
                if re.search(search_input, keyword, re.IGNORECASE):
                    keywords_score += 1

            vectorization = row['vectorization']
            vector = torch.tensor(vectorization)
            cosine = torch.nn.functional.cosine_similarity(vector, search_vector)
            if cosine > 0.38:
                vectorization_score += 1


            score_array = [vectorization_score, keywords_score, summary_score, freq_score, filename_score]

            final_score = np.average(a =score_array, weights= score_weights)
            matrix.at[index, 'score'] += float(final_score)

    # Normalize the score using the sigmoid function
    matrix = matrix.sort_values(by='score', ascending=False)
    max = matrix['score'].max()
    matrix['score'] = sigmoid(matrix['score'], max)

    matrix['delta'] = matrix['score'].max() - matrix['score']
    return matrix

def main():
    search_string = ""
    search_string = input("Search: ")
    res = search(search_string)

    # # UNCOMMENT TO PLOT THE RESULTS
    # plt.bar(res['filename'], res['score'])
    # plt.xlabel('File Name')
    # plt.ylabel('Score')
    # plt.xticks(rotation=90)
    # plt.title('KNOWTILUS SEARCH RESULTS: ' + search_string)
    # plt.show()


    print("KNOWTILUS SEARCH RESULTS: ", search_string)
    print(tabulate(res.head(20)[['filename', 'score', 'delta']], headers='keys', tablefmt='psql', showindex='never'))



 
while True:
    main()


