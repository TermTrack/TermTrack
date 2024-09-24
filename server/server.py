from flask import Flask
import json

app = Flask(__name__)


@app.route("/result/<level_id>/<name>/<time>")
def result(level_id, name, time):
    with open("toplist.json") as f:
        topplistor = json.loads(f.read())
        try:
            topplistor[level_id]
        except:
            topplistor[level_id] = []
    topplistor[level_id].append({"name": name, "time": time})
    with open("toplist.json", "w") as f:
        f.write(json.dumps(topplistor))
    x = sorted(topplistor[level_id], key=lambda x: float(x["time"]))[0:10]
    print(x)
    return json.dumps(x)


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=7000)
