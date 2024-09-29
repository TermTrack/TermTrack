# A very simple Flask Hello World app for you to get started with...

from flask import Flask
import sqlite3
import json

app = Flask(__name__)


def get_cursor():
    con = sqlite3.connect("./scores.db")
    cur = con.cursor()
    return cur, con


@app.route("/log_result/<id>/<name>/<time>")
def log_result(id, name, time):
    cur, con = get_cursor()
    cur.execute("insert into scores values (?,?,?)", (id, name, time))
    con.commit()
    return "success"


@app.route("/get_result/<id>")
def get(id):
    cur, con = get_cursor()
    l = []
    for row in cur.execute(
        "select name, time from scores where id = ? order by time", (id,)
    ):
        l.append({"name": str(row[0]), "time": float(row[1])})
    res = json.dumps(l)
    return res


app.run(host="0.0.0.0", port=8000)
