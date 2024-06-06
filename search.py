import json
import logging
import pandas as pd
import sys
import os

from transformers import pipeline
import torch
from sentence_transformers import SentenceTransformer

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
matrix['score'] = 0
# TODO: Make each score a column the create a weighted average of the scores

def search(search_string):
    user_input = search_string.lower().split()
    for search_input in user_input:

        search_vector = torch.tensor(vectorize(search_input))

        for index, row in matrix.iterrows():
            summary = row['summary']
            summary_score = 0
            if search_input in summary:
                summary_score = 1
            matrix.at[index, 'score'] += summary_score

            frequency = row['frequency']
            freq_score = frequency[search_input] if search_input in frequency else 0
            matrix.at[index, 'score'] += freq_score

            # calculate the score for the keywords column
            keywords = row['keywords']
            keywords_score = 0
            for keyword in keywords:
                if search_input in keyword:
                    keywords_score = 1
                    break
            matrix.at[index, 'score'] += keywords_score

            vectorization = row['vectorization']
            vectorization_score = 0
            for vector in vectorization:
                vector = torch.tensor(vector)
                cosine = torch.nn.functional.cosine_similarity(vector, search_vector)
                if cosine > 0.45:
                    vectorization_score = 1
                    break
            matrix.at[index, 'score'] += vectorization_score

def main():
    search_string = input("Enter search string: ")
    search(search_string)
    print("SEARCH RESULTS")
    # print(matrix.sort_values(by='score', ascending=False))

    for i in range(5):
        res = matrix.sort_values(by='score', ascending=False).iloc[i]
        p = str(res['score']) + " " + res['filename']
        print(p)

    sumarize = input("Do you want to see the summary of the top result? (y/n): ")
    if sumarize == 'y':
        top_result = matrix.sort_values(by='score', ascending=False).iloc[0]
        print("Summary of the top result:")
        print(top_result['summary'])
    # else:
        # print("Goodbye!")
        # sys.exit(0)


while True:
    main()


