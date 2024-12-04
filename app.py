from flask import Flask, render_template, redirect, request, session, flash
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import json
import subprocess
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import accuracy_score, confusion_matrix, classification_report
from sklearn.preprocessing import StandardScaler





def predict_user(user):
    #user = np.array(user[1:]).reshape(1, -1)
    user = np.array(user).reshape(1,-1)
    prediction = model.predict_proba(user)
    return prediction

def get_user_data(username):
    p = subprocess.run(["cargo", "run", "--", "--username", username], capture_output=True)
    user = json.loads(json.loads(p.stdout))
    data = [user["hsp"], user["wrp"], user["wins"], user["losses"], user["matches"], user["kd"], user["kills"], user["deaths"], user["kpm"]]
    return data





p = subprocess.run(["cargo", "build"]) # Compile Rust code
data = pd.read_csv("R6_Data.csv")      # read training data

#print(data.head())  # Inspect the first few rows

X = data.drop(["username","cheater"], axis=1)  # Features
y = data['cheater']               # Labels

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.25, random_state=42)

model = LogisticRegression()
model.fit(X_train, y_train)

ss_train = StandardScaler()
X_train = ss_train.fit_transform(X_train)

ss_test = StandardScaler()
X_test = ss_test.fit_transform(X_test)



predictions = model.predict(X_test)

cm = confusion_matrix(y_test, predictions)

TN, FP, FN, TP = confusion_matrix(y_test, predictions).ravel()

print('True Positive(TP)  = ', TP)
print('False Positive(FP) = ', FP)
print('True Negative(TN)  = ', TN)
print('False Negative(FN) = ', FN)

accuracy =  (TP + TN) / (TP + FP + TN + FN)

print('Accuracy of the binary classifier = {:0.3f}'.format(accuracy))


app = Flask(__name__, template_folder="template")
app.secret_key = 'your_secret_key'  # secret key


@app.route('/')
def index(): # this is the index page
    if 'username' in session:
        username = session['username']
        other_players_stats = stats_df[stats_df['username'] != username]
        page = request.args.get('page', 1, type=int)
        per_page = 9
        start = (page - 1) * per_page
        end = start + per_page
        page_stats = other_players_stats.iloc[start:end]
        return render_template("index.html", username=username, stats=page_stats, page=page, total_pages=len(other_players_stats) // per_page + (1 if len(other_players_stats) % per_page != 0 else 0))
    return redirect('/login')

@app.route('/login', methods=['GET', 'POST'])
def login(): # this is the login page 
    if request.method == 'POST': # user inputs username
        username = request.form['username']
        """
        if username in stats_df['username'].values:
            session['username'] = username
            return redirect('/')
        """
        user_data = get_user_data(username) # call Rust program with username as parameter, returns data
        prediction = predict_user(user_data)[0] # returns probability that user is a cheater, do whatever with this
        print(prediction)
        msg = str("Probability that user " + username + " is a cheater: " + str(prediction[1]) + "%")
        print(msg)
        flash(msg)
    else: # error for wrong username
        print("Account does not exist")
        return render_template('login.html', error="Account does not exist")
    return render_template('login.html')

@app.route('/logout')
def logout(): # this is the logout button that redirects back to the login page
    session.pop('username', None)
    return redirect('/login')

if __name__ == '__main__':
    app.run(debug=True, port=8000)