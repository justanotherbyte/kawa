import kawa


def test_plaintext():
    response = kawa.request("GET", "http://localhost:8080")
    assert response.body == "hello world"

def test_json():
    import json

    payload = {
        "username": "justanotherbyte",
        "password": "http"
    }
    response = kawa.request(
        "POST",
        "http://127.0.0.1:8080/data",
        headers={"Content-Type": "application/json"},
        body=json.dumps(payload)
    )
    assert json.loads(response.body) == payload

def test_parse_http_url():
    url1 = kawa.Url("http://localhost:8080")
    url2 = kawa.Url("https://localhost:8080/")
    url3 = kawa.Url("http://localhost:8080/hello?param=1")

    assert url1.scheme == "http"
    assert url1.path == "/"
    assert url2.scheme == "https"
    assert url2.path == "/"
    assert url3.path == "/hello?param=1"
