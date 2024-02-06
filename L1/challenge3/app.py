from flask import Flask, make_response, render_template, request, send_file
from apscheduler.schedulers.background import BackgroundScheduler
from datetime import datetime, timedelta 
import uuid
import os
import logging
import re

app = Flask(__name__)

# Configure logging
formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
handler = logging.StreamHandler()
handler.setFormatter(formatter)
app.logger.addHandler(handler)
app.logger.setLevel(logging.DEBUG)  # Set the logging level to DEBUG for more detailed logs
app.config['PATTERN_FILENAME'] = re.compile(r'^[a-zA-Z0-9/\-]+(?:\.(txt|jpg|jpeg|png|pdf))$')
app.config['PATTERN_EXT'] = re.compile(r'^(?:\.(txt|jpg|jpeg|png|pdf))$')
app.config['UPLOAD_FOLDER'] = '/app/uploads'
app.config['MAX_CONTENT_LENGHT'] = 1024 * 1024 * 8 * 10 # 10MB

@app.route('/', methods=['GET', 'POST'])
def upload_file():
    app.logger.info('User access to main page')
    if request.method == 'POST':
        uploaded_file = request.files['file']
        if uploaded_file.filename:
            uuid_filename = str(uuid.uuid4())
            _, file_extension = os.path.splitext(uploaded_file.filename)

            if not app.config['PATTERN_EXT'].match(file_extension):
                return render_template('index.html', status='The extension is not within .txt, .jpg, .jpeg, .png or .pdf'), 400
            
            if len(uploaded_file.read()) > app.config['MAX_CONTENT_LENGHT']:
                return render_template('index.html', status='File size exceeds the maximum allowed limit (10MB)'), 400
            
            filename = os.path.join(app.config['UPLOAD_FOLDER'], uuid_filename + file_extension)
            uploaded_file.seek(0)  # Reset the file cursor.
            uploaded_file.save(filename)
            app.logger.info('File uploaded with name : %s', uuid_filename + file_extension)
            return render_template('index.html', status='File uploaded successfully with name : ' + uuid_filename + file_extension)
        else:
            return render_template('index.html', status='No file given')
    return render_template('index.html')

@app.route('/file', methods=['POST'])
def download_file():
    uuid_filename = request.form['name']
    app.logger.info('attempt to recover file : %s', uuid_filename)
    if uuid_filename and app.config['PATTERN_FILENAME'].match(uuid_filename):
        filename = os.path.join(app.config['UPLOAD_FOLDER'], uuid_filename)
        print(filename)
        if os.path.exists(filename):
            return send_file(filename, as_attachment=True);
        else:
            app.logger.info('File not found')
            return render_template('index.html', status='File not found'), 404
    else:
        app.logger.info('Invalid request')
        return render_template('index.html', status='Invalid request'), 400

def delete_old_files():
    now = datetime.now()
    cutoff_time = now - timedelta(minutes=1)
    for filename in os.listdir(app.config['UPLOAD_FOLDER']):
        file_path = os.path.join(app.config['UPLOAD_FOLDER'], filename)
        if os.path.isfile(file_path):
            file_time = datetime.fromtimestamp(os.path.getmtime(file_path))
            if file_time < cutoff_time:
                os.remove(file_path)

if __name__ == '__main__':
    app.run(debug=False, host='0.0.0.0', port=8000)
    # Initialize the scheduler
    scheduler = BackgroundScheduler()
    scheduler.add_job(delete_old_files, 'interval', minutes=1)  # Run every minute
    scheduler.start()