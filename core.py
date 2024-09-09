
class Lex:
    stopwords = {"i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", 
                "your", "yours", "yourself", "yourselves", "he", "him", "his", "himself",
                "she", "her", "hers", "herself", "it", "its", "itself", "they", "them", "their",
                "theirs", "themselves", "what", "which", "who", "whom", "this", "that", "these",
                "those", "am", "is", "are", "was", "were", "be", "been", "being", "have", "has",
                "had", "having", "do", "does", "did", "doing", "a", "an", "the", "and", "but",
                "if", "or", "because", "as", "until", "while", "of", "at", "by", "for", "with",
                "about", "against", "between", "into", "through", "during", "before", "after",
                "above", "below", "to", "from", "up", "down", "in", "out", "on", "off", "over",
                "under", "again", "further", "then", "once", "here", "there", "when", "where",
                "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some",
                "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very",
                "s", "t", "can", "will", "just", "don", "should", "now", '1', '2', '3', '4', '5',
                '6', '7', '8', '9', '0', '−→', "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
                "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"}

    strict_punctuation = "!\"#$~%&()'*,+,/:;.<=>?@[\\]^-_`’{|}~•"
    sentence_punctuation = "\"#$~%&()'*+/:;<=>@[\\]^-_`{|}~•’"

    text = ""
    sentences = []
    tokens = []

    def __init__(self, text): 

        lower_text = str(text).lower()
        stoppless_text = self.process_stopwords(lower_text)

        sentences = self.strip_punctuation(stoppless_text, self.sentence_punctuation)
        self.sentences = str(sentences).replace('\n', '.').replace('!', '.').replace('?', '.').replace(',', '.').split('.')

        self.text = self.strip_punctuation(stoppless_text, self.strict_punctuation)

        temp_token = []
        tokens = self.text.split()
        for token in tokens:
            # TODO : Lemmatize the token. Python is crashing because of a SQL error.
            # TODO : Only tokenize real words. Check if the token is in a dictionary.
            temp_token.append( token )
        self.tokens = temp_token

    def process_stopwords(self, text):
        return ' '.join([word for word in text.split() if word.lower() not in Lex.stopwords])
    
    def strip_punctuation(self, text, punctuation=strict_punctuation):
        return text.translate(str.maketrans('', '', punctuation))

    def get_text(self):
        return self.text
    
    def get_tokens(self):
        return self.tokens

    def get_sentences_aray(self):
        return self.sentences

    def get_sentences(self):
        return ' '.join(self.sentences)

    #  TODO: Function that checks if the token is in a dictionary

    # TODO: Function which laments words in a text