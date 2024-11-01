import matplotlib.pyplot as plt
import pandas as pd
import numpy as np

from sklearn.model_selection import train_test_split
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import accuracy_score, confusion_matrix, classification_report
from sklearn.preprocessing import StandardScaler

def predict_user(user):
    user = np.array(user[1:]).reshape(1, -1)
    prediction = model.predict_proba(user)
    return prediction

data = pd.read_csv("R6_Data.csv")

print(data.head())  # Inspect the first few rows

X = data.drop(["username","cheater"], axis=1)  # Features
y = data['cheater']               # Labels

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.25, random_state=42)

model = LogisticRegression()
model.fit(X_train, y_train)

ss_train = StandardScaler()
X_train = ss_train.fit_transform(X_train)

ss_test = StandardScaler()
X_test = ss_test.fit_transform(X_test)

user = [0.518,0.506,1125,957,2221,1.19,9604,8047,4.32]
predictions = model.predict(X_test)

cm = confusion_matrix(y_test, predictions)

TN, FP, FN, TP = confusion_matrix(y_test, predictions).ravel()

print('True Positive(TP)  = ', TP)
print('False Positive(FP) = ', FP)
print('True Negative(TN)  = ', TN)
print('False Negative(FN) = ', FN)

accuracy =  (TP + TN) / (TP + FP + TN + FN)

print('Accuracy of the binary classifier = {:0.3f}'.format(accuracy))


# user = call_scraper(username)
user = ["polo1.",0.518,0.506,1125,957,2221,1.19,9604,8047,4.32]
prediction = predict_user(user)
print(prediction)
# user is a single row of data, including username but not the "cheater" field
