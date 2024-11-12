from flask import Flask, render_template, redirect, request, session
import pandas as pd

app = Flask(__name__, template_folder="template")
app.secret_key = 'your_secret_key'  # secret key

# Dummy user data for login for demo; TO DO: May need to remove or edit to read full file
#users = {'AS1XN': 'password1', 'user2': 'password2'}

# TO DO: Load stats from zachs csv
stats_df = pd.read_csv('R6_Data.csv')

@app.route('/')
def index():
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
def login():
    if request.method == 'POST':
        username = request.form['username']
        #password = request.form['password']
        if username in stats_df['username'].values: #and users[username] == password:
            session['username'] = username
            return redirect('/')
        else:
            print("Account does not exist")
            return render_template('login.html', error="Account does not exist")
    return render_template('login.html')
@app.route('/logout')
def logout():
    session.pop('username', None)
    return redirect('/login')

if __name__ == '__main__':
    app.run(debug=True, port=8000)