# Create an application reads all the pdf files in the directory the user passes as an argument.

import os
import sys
import PyPDF2
from fpdf import FPDF
import json
import logging
from json import JSONEncoder
from transformers import pipeline
import torch
from sentence_transformers import SentenceTransformer

logging.basicConfig(level=logging.DEBUG)

vector_transformer = SentenceTransformer('sentence-transformers/all-mpnet-base-v2')

summary_transformer = pipeline(
        "summarization",
        'pszemraj/led-base-book-summary',
        device=0 if torch.cuda.is_available() else -1,
    )

class ReportEncoder(JSONEncoder):
    def default(self, o):
        return o.__dict__

class Database:
    def __init__(self):
        self.files_reports = {}

    def add(self, file_report):
        # Allow for file_report to be serialized
        self.files_reports[file_report.filename] = file_report
                # report = json.dumps(file_report, indent=4, cls=ReportEncoder)
        # self.files_reports.append(report)
    
    def load_json(self, json_string):
        try:
            self.files_reports = json.loads(json_string)
        except Exception as e:
            logging.error(e)
            self.files_reports = {}

    
    def serialize(self):
        return json.dumps(self.files_reports, cls=ReportEncoder)


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
        self.title = ""

    def add_title(self, title):
        self.title = title

    def add_keywords(self, keywords):
        self.keywords = keywords

    def add_vectorization(self, vectorization):
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
            'keywords': self.keywords,
            'title': self.title
        }

def load_database(directory):
    db = Database()
    database = os.path.join(directory, 'knowtilus.db')
    if not os.path.exists(database):
        logging.info("Creating New Database")
        return db
    else:
        with open(database, 'r') as file:
            string = file.read()
            db.load_json(string)
            file.close()
            logging.info("Existing database loaded.")
            return db

def read_pdf_files(directory):
    path = os.path.join(directory, 'knowtilus.db')
    db = load_database(directory)
    try:
        for filename in os.listdir(directory):
            if filename.endswith('.pdf'):
                pdf_file = open(directory
                                + '/' + filename, 'rb')
                pdf_reader = PyPDF2.PdfFileReader(pdf_file)
                logging.info("Analyzing New File")
                # document_title = pdf_reader.getDocumentInfo().title

                for page_num in range(pdf_reader.numPages):
                    page = pdf_reader.getPage(page_num)
                    human_page = page_num + 1
                    analysis_key = filename + '-p' + str(human_page)
                    if db.files_reports.get(analysis_key):
                        logging.info("Skipping: " + analysis_key)
                        continue 
                    logging.info("Analyzing: " + analysis_key)
                    extracted_text = page.extract_text()
                    logging.debug("Found Text: " + str(extracted_text))
                    #  TODO: Turn this analysis into a function for easy reuse.
                    fa = FileAnalysis(analysis_key)
                    keywords = keyword_analysis(extracted_text)
                    fa.add_keywords(keywords)
                    status_keywords = "Keywords: " + str(keywords)
                    logging.debug(status_keywords)

                    embeddings = vectorize(extracted_text)
                    fa.add_vectorization(embeddings)
                    status_vector = "Vectors: " + str(embeddings)
                    logging.debug(status_vector)

                    summary = summarizer(extracted_text)
                    fa.add_summary(summary[0]['summary_text'])
                    status_summary = "Summary: " + str(summary[0]['summary_text'])
                    logging.debug(status_summary)
                    
                    freq = frequency_analysis(extracted_text)
                    fa.add_frequency(freq)
                    status_frequency = "Frequency: " + str(freq)
                    logging.debug(status_frequency)

                    # fa.add_title(document_title)
                    db.add(fa)
    except KeyboardInterrupt:
        logging.error("Keyboard Interrupt")
        pdf_file.close()
        write_to_disk(db.serialize(), path)
        sys.exit(1)
    pdf_file.close()
    serialized_db = db.serialize()
    write_to_disk(serialized_db, path)

def write_to_disk(serialized_db, path):
    with open(path, 'w') as f:
        f.write(serialized_db)
        f.close()
        logging.info("Write Success")


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
    text = remove_punctuation(text).lower()

    stopwords = {'a', 'the', 'is', 'of', 'in', 'and', 'on', 'to', 'as', 'at', 'by', 'for', 'an', 'with', 'from', 'that', 'this', 'these', 'those', 'it', 'its', 'are', 'was', 'were', 'be', 'been', 'being', 'have', 'has', 'had', 'do', 'does', 'did', 'but', 'not', 'or', 'if', 'because', 'so', 'such', 'too', 'very', 'can', 'will', 'would', 'should', 'may', 'might', 'must', 'shall', 'than', 'then', 'just', 'also', 'now', 'here', 'there', 'where', 'when', 'how', 'all', 'any', 'both', 'each', 'few', 'more', 'most', 'other', 'some', 'own', 'same', 'than', 'too', 'very', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '−→'}

    words = [word for word in text.split() if word.lower() not in stopwords]
    ngrams = []
    for i in range(len(words)-n+1):
        ngram = tuple(words[i:i+n])
        ngrams.append(ngram)

    ngram_counts = {}
    for ngram in ngrams:
        if ngram not in ngram_counts:
            ngram_counts[ngram] = 0
        ngram_counts[ngram] += 1

    filtered_counts = {ngram: count for ngram, count in ngram_counts.items() if count >= min_count}
    top_ngrams = []
    for ngram, count in sorted(filtered_counts.items(), key=lambda item: item[1], reverse=True):
        top_ngrams.append((ngram, count))
        if len(top_ngrams) == 10:
            break
    top_ngrams.sort(key=lambda item: item[1], reverse=True)
    keywords = []
    for ngram, count in top_ngrams:
        keyword = ' '.join(ngram)
        if len(keyword) > 1:
            keywords.append(keyword)
    return keywords


def vectorize(text):
    # REF: https://huggingface.co/sentence-transformers/all-mpnet-base-v2
    sentences = text.lower().replace('\n', '.').replace('!', '.').replace('?', '.').split('.')

    embeddings = vector_transformer.encode(sentences)
    return embeddings

def remove_punctuation(text):

    punctuation = "!\"#$~%&()*,+,/:;.<=>?@[\\]^-_`{|}~"
    return text.translate(str.maketrans('', '', punctuation))

def summarizer(input_text):
    return summary_transformer(
        input_text,
        min_length=5,
        max_length=100,
        no_repeat_ngram_size=3,
        encoder_no_repeat_ngram_size=3,
        repetition_penalty=3.5,
        num_beams=4,
        early_stopping=True,
    )

def convert_txt_to_pdf(directory):
    for filename in os.listdir(directory):
        if filename.endswith('.txt') and not filename.endswith('.pdf'):
            with open(directory + '/' + filename, 'r') as file:
                text = file.read()
                file.close()
                pdf = FPDF()
                pdf.add_page()
                pdf.set_font("Courier", size=11)
                pdf.multi_cell(0, 10, text)
                pdf.output(directory + '/' + filename + '.pdf')
                os.remove(directory
                            + '/' + filename)

try:
    if len(sys.argv) != 2:
        print('Usage: python main.py <directory>')
        sys.exit(1)
    directory = sys.argv[1]
    convert_txt_to_pdf(directory)
    read_pdf_files(directory)
   
except Exception as e:
    logging.error(e)
    sys.exit(1)


