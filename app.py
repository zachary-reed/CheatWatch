from flask import Flask, render_template, redirect, request, session
import pandas as pd

app = Flask(__name__, template_folder="template")
app.secret_key = 'your_secret_key'  # secret key

stats_df = pd.read_csv('R6_Data.csv') # reads data from r6_data

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
        if username in stats_df['username'].values:
            session['username'] = username
            return redirect('/')
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