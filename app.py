from flask import Flask, render_template, redirect, request, session, flash
import pandas as pd
import numpy as np
import json
import subprocess
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import accuracy_score, confusion_matrix, classification_report
from sklearn.preprocessing import StandardScaler

# loads model and data
data = pd.read_csv("R6_Data.csv")  # read training data
X = data.drop(["username", "cheater"], axis=1)  # features
y = data['cheater']  # labels
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.25, random_state=42)

model = LogisticRegression()
model.fit(X_train, y_train)

ss_train = StandardScaler()
X_train = ss_train.fit_transform(X_train)

ss_test = StandardScaler()
X_test = ss_test.fit_transform(X_test)

# prediction function
def predict_user(user):
    user = np.array(user).reshape(1, -1)
    prediction = model.predict_proba(user)
    return prediction

# function to get user data from the Rust app
def get_user_data(username):
    p = subprocess.run(["cargo", "run", "--", "--username", username], capture_output=True)
    try:
        user = json.loads(json.loads(p.stdout))
    except json.JSONDecodeError:
        return None  # return None if user not found
    data = [user["hsp"], user["wrp"], user["wins"], user["losses"], user["matches"], user["kd"], user["kills"], user["deaths"], user["kpm"]]
    return data

app = Flask(__name__, template_folder="template")
app.secret_key = 'your_secret_key'

@app.route('/')
def index(): 
    if 'username' in session:
        username = session['username']
        user_data = data[data['username'] == username].iloc[0]  # Get user stats from CSV
        other_players_stats = data[data['username'] != username]
        page = request.args.get('page', 1, type=int)
        per_page = 9
        start = (page - 1) * per_page
        end = start + per_page
        page_stats = other_players_stats.iloc[start:end]
        
        # Predict if the user is a cheater
        prediction = predict_user([user_data[["hsp", "wrp", "wins", "losses", "matches", "kd", "kills", "deaths", "kpm"]].values.tolist()])[0]
        prob_cheater = prediction[1] * 100  # probability of being a cheater
        
        flash(f"Probability that user {username} is a cheater: {prob_cheater:.2f}%")
        return render_template("index.html", username=username, stats=user_data, page=page, total_pages=len(other_players_stats) // per_page + (1 if len(other_players_stats) % per_page != 0 else 0))
    return redirect('/login')

@app.route('/login', methods=['GET', 'POST'])
def login():
    if request.method == 'POST':  # user inputs username
        username = request.form['username']
        
        # checks if username exists in the CSV file
        if username in data['username'].values:
            session['username'] = username  # stores the username in the session
            return redirect('/')  # redirects to the home page
        else:
            return render_template('login.html', error="Account does not exist")
    return render_template('login.html')

@app.route('/logout')
def logout(): 
    session.pop('username', None)
    return redirect('/login')

if __name__ == '__main__':
    app.run(debug=True, port=8000)