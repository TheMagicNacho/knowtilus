# Create an application reads all the pdf files in the directory the user passes as an argument.

import os
import sys
import PyPDF2
import json

from transformers import pipeline
import torch
from sentence_transformers import SentenceTransformer


class Database:
    def __init__(self):
        self.files_reports = {}

    def add(self, file_report):
        self.files_reports[file_report.filename] = file_report
    
    def serialize(self):
        #  TODO: Turn this into a hashmap instead of an array.
        return {
            'files_reports': [file_report.serialize() for file_report in self.files_reports.values()]
        }
    
    def dump_to_disk(self):
        data = json.dumps(self.serialize())
        # TODO: have the databasse save to the directory of the pdf files.
        with open('database.json', 'w') as f:
            f.write(data)
            f.close()

class FileAnalysis:
    def __init__(self, filename):
        # filename is the name of the file associated with this analysis.
        self.filename = filename
        # The sumary of the text using a hugging face model.
        self.summary = ""
        # Simple frequency analysis of the text. Includes the count of each word in the text.
        self.frequency = {}
        # Vectorization as numpy array of the text for vector searching.
        self.vectorization = []
        # Array of strings. Keywords are found using an n-gram analysis.
        self.keywords = []

    def add_keywords(self, keywords):
        self.keywords = keywords

    def add_vectorization(self, vectorization):
        # serialize from numpy array to list
        self.vectorization = vectorization.tolist()

    def add_summary(self, summary):
        if self.summary:
            self.summary = self.summary + '::' + summary
        else:
            self.summary = summary

    def add_frequency(self, freq_analysis):
        self.frequency = freq_analysis
    
    def dump_json(self):
        d = json.dumps(self.serialize())
        with open(self.filename + '-report.json', 'w') as f:
            f.write(d)
            f.close()

    def serialize(self):
        return {
            'filename': self.filename,
            'summary': self.summary,
            'frequency': self.frequency,
            'vectorization': self.vectorization,
            'keywords': self.keywords
        }

def read_pdf_files(directory):
    db = Database()
    for filename in os.listdir(directory):
        if filename.endswith('.pdf'):
            pdf_file = open(directory
                            + '/' + filename, 'rb')
            pdf_reader = PyPDF2.PdfFileReader(pdf_file)
            # TODO conduct an analysis of the text in the pdf file AND by each page.

            for page_num in range(pdf_reader.numPages):
                page = pdf_reader.getPage(page_num)
                extracted_text = page.extract_text()
                print("text: ", extracted_text)
                #  TODO: Turn this analysis into a function for easy reuse.
                fa = FileAnalysis(filename + '-p' + str(page_num))
                keywords = keyword_analysis(extracted_text)
                fa.add_keywords(keywords)

                embedings = vectorize(extracted_text)
                fa.add_vectorization(embedings)

                summary = summarizer(extracted_text)
                fa.add_summary(summary[0]['summary_text'])

                freq = frequency_analysis(extracted_text)
                fa.add_frequency(freq)
                print(fa.serialize())
                db.add(fa)
            pdf_file.close()
    db.dump_to_disk()


def frequency_analysis(text):
    words = remove_punctuation(text).lower().split()
    freq = {}
    for word in words:
        if word in freq:
            freq[word] += 1
        else:
            freq[word] = 1
    return freq

def keyword_analysis(text, n=1, min_count=2):
    #  Ngram analysis of the text to find the most common keywords.
    text = remove_punctuation(text).lower()

    # dictionary of stopwords for commonly used articles and prepositions
    stopwords = {'a', 'the', 'is', 'of', 'in', 'and', 'on', 'to', 'as', 'at', 'by', 'for', 'an', 'with', 'from', 'that', 'this', 'these', 'those', 'it', 'its', 'are', 'was', 'were', 'be', 'been', 'being', 'have', 'has', 'had', 'do', 'does', 'did', 'but', 'not', 'or', 'if', 'because', 'so', 'such', 'too', 'very', 'can', 'will', 'would', 'should', 'may', 'might', 'must', 'shall', 'than', 'then', 'just', 'also', 'now', 'here', 'there', 'where', 'when', 'how', 'all', 'any', 'both', 'each', 'few', 'more', 'most', 'other', 'some', 'own', 'same', 'than', 'too', 'very', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '−→'}

    # Split text into words, ignoring stopwords
    words = [word for word in text.split() if word.lower() not in stopwords]

    # Generate n-grams of words
    ngrams = []
    for i in range(len(words)-n+1):
        ngram = tuple(words[i:i+n])
        ngrams.append(ngram)

    # Count n-gram frequencies manually
    ngram_counts = {}
    for ngram in ngrams:
        if ngram not in ngram_counts:
            ngram_counts[ngram] = 0
        ngram_counts[ngram] += 1

    # Filter by minimum count manually
    filtered_counts = {ngram: count for ngram, count in ngram_counts.items() if count >= min_count}

    # Sort by frequency (descending) and get top 10 manually (similar to previous example)
    top_ngrams = []
    for ngram, count in sorted(filtered_counts.items(), key=lambda item: item[1], reverse=True):
        top_ngrams.append((ngram, count))
        if len(top_ngrams) == 10:
            break
    top_ngrams.sort(key=lambda item: item[1], reverse=True)
    #    generate an array of the top ngram keywords
    keywords = []
    for ngram, count in top_ngrams:
        keyword = ' '.join(ngram)
        if len(keyword) > 1:
            keywords.append(keyword)
    return keywords


def vectorize(text):
    # REF: https://huggingface.co/sentence-transformers/all-mpnet-base-v2
    sentences = text.lower().replace('\n', '.').replace('!', '.').replace('?', '.').split('.')
    model = SentenceTransformer('sentence-transformers/all-mpnet-base-v2')
    embeddings = model.encode(sentences)
    return embeddings

def remove_punctuation(text):
  """
  Removes all punctuation characters from a string.

  Args:
      text: The string to remove punctuation from.

  Returns:
      A new string without any punctuation characters.
  """
  punctuation = "!\"#$~%&()*,+,/:;.<=>?@[\\]^-_`{|}~"
  no_punct_text = text.translate(str.maketrans('', '', punctuation))
  return no_punct_text

def summarizer(input_text):
    summarizer = pipeline(
        "summarization",
        'pszemraj/led-base-book-summary',
        device=0 if torch.cuda.is_available() else -1,
    )

    result = summarizer(
        input_text,
        min_length=16,
        max_length=256,
        no_repeat_ngram_size=3,
        encoder_no_repeat_ngram_size=3,
        repetition_penalty=3.5,
        num_beams=4,
        early_stopping=True,
    )

    return result

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: python main.py <directory>')
        sys.exit(1)
    directory = sys.argv[1]
    read_pdf_files(directory)