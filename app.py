from flask import Flask, render_template, redirect, request, session
import pandas as pd

app = Flask(__name__, template_folder="template")
app.secret_key = 'your_secret_key'  # Replace with a strong secret key

# Dummy user data for login (replace with database logic)
users = {'user1': 'password1', 'user2': 'password2'}

# Load stats from CSV (ensure your CSV has columns like 'username' and 'stats')
stats_df = pd.read_csv('stats.csv')

@app.route('/')
def index():
    if 'username' in session:
        username = session['username']
        user_stats = stats_df[stats_df['username'] == username]
        return render_template("index.html", username=username, stats=user_stats)
    return redirect('/login')

@app.route('/login', methods=['GET', 'POST'])
def login():
    if request.method == 'POST':
        username = request.form['username']
        #password = request.form['password']
        if username in users: #and users[username] == password:
            session['username'] = username
            return redirect('/')
        else:
            print("Account does not exist")
    return render_template('login.html')

@app.route('/logout')
def logout():
    session.pop('username', None)
    return redirect('/login')

if __name__ == '__main__':
    app.run(debug=True, port=8000)