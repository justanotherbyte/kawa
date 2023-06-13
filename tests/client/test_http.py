import kawa


def test_plaintext():
    response = kawa.request("GET", "127.0.0.1", 8080, "/")
    assert response.body == "hello world"

def test_json():
    import json

    payload = {
        "username": "justanotherbyte",
        "password": "http"
    }
    response = kawa.request(
        "POST",
        "127.0.0.1",
        8080,
        "/data",
        headers={"Content-Type": "application/json"},
        body=json.dumps(payload)
    )
    assert json.loads(response.body) == payload