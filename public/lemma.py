from flask import Flask, request
from flask_cors import CORS, cross_origin
from pymystem3 import Mystem

app = Flask(__name__)
cors = CORS(app) # разрешить CORS для всех
app.config['CORS_HEADERS'] = 'Content-Type'
@app.route('/', methods=['POST', 'GET'])
@cross_origin()
def index():
   if request.method == 'POST':
      text = request.form.get('text')
      m = Mystem()
      lemmas = m.lemmatize(text)
      lemmas.sort()
      out =  ' '.join(lemmas).strip().lower() 
      return out
   else:
      return 'Use POST requests'
if __name__ == '__main__':
    #app.debug = True
    app.run(host='0.0.0.0', port=8000)
