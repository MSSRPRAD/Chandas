from flask import Flask, request, jsonify, render_template
from chandas import identify_matra_rs, identify_vrtta_rs, identify_anushtup_rs
app = Flask(__name__)

# Define a route for the home page
@app.route('/')
def home():
    return render_template('./index.html')

# Define a route for handling GET and POST requests to /api
@app.route('/api', methods=['GET', 'POST'])
def api():
    if request.method == 'GET':
        return jsonify({"message": "No Data to show yet"})
    elif request.method == 'POST':
        data = request.get_json()
        print('recieved:'+data['inputData'])
        if data is not None:
            resp_matra = identify_matra_rs(data['inputData'])
            resp_anust = identify_anushtup_rs(data['inputData'])
            resp_vrtta = identify_vrtta_rs(data['inputData'])
            return jsonify({"result": "Success!", "matra": resp_matra, "vrtta": resp_vrtta, "anushtup": resp_anust})
        else:
            return jsonify({"message": "Invalid JSON data"}), 400

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
