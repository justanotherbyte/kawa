import timeit

import kawa
import requests
import httpx


def benchmark_time_for_x_requests():
    NUMBER = 10 ** 4 # 10k

    def kawa_request():
        response = kawa.request("GET", "http://localhost:8080")
        print(response)

    def requests_request():
        response = requests.request("GET", "http://127.0.0.1:8080")
        print(response)

    def httpx_request():
        response = httpx.request("GET", "http://127.0.0.1:8080")
        print(response)

    kawa_time = timeit.timeit(kawa_request, number=NUMBER)
    requests_time = timeit.timeit(requests_request, number=NUMBER)
    httpx_time = timeit.timeit(httpx_request, number=NUMBER)

    print(f"Benchmark for: {NUMBER:,} requests:")

    print("Kawa:", kawa_time)
    print("Requests: ", requests_time)
    print("HTTPX: ", httpx_time)

    print("Kawa is {}x faster than requests".format(round(requests_time / kawa_time)))
    print("Kawa is {}x faster than httpx".format(round(httpx_time / kawa_time)))

    make_chart(
        name="Time - 10,000 requests",
        requests_time=requests_time,
        kawa_time=kawa_time,
        httpx_time=httpx_time,
        benchmark_type="plaintext"
    )

def benchmark_time_for_x_json_requests():
    NUMBER = 10 ** 4
    import json
    import sys

    with open("tests/client/data.json", "r") as f:
        payload = f.read()

    payload_size = sys.getsizeof(payload)
    payload = json.loads(payload)

    def kawa_request():
        body = json.dumps(payload, indent=0)
        response = kawa.request(
            "POST",
            "http://127.0.0.1:8080/data",
            body=body,
            headers={"Content-Type": "application/json"}
        )
        print(response)

    def requests_request():
        response = requests.request("POST", "http://127.0.0.1:8080/data", json=payload)
        print(response)

    def httpx_request():
        response = httpx.request("GET", "http://127.0.0.1:8080", json=payload)
        print(response)

    kawa_time = timeit.timeit(kawa_request, number=NUMBER)
    requests_time = timeit.timeit(requests_request, number=NUMBER)
    httpx_time = timeit.timeit(httpx_request, number=NUMBER)

    print(f"Benchmark for json: {NUMBER:,} requests - {payload_size} bytes size payload:")

    print("Kawa:", kawa_time)
    print("Requests: ", requests_time)
    print("HTTPX: ", httpx_time)
    
    print("Kawa is {}x faster than requests".format(round(requests_time / kawa_time)))
    print("Kawa is {}x faster than httpx".format(round(httpx_time / kawa_time)))

    make_chart(
        "Time - 10,000 requests",
        requests_time=requests_time,
        kawa_time=kawa_time,
        httpx_time=httpx_time,
        benchmark_type="json"
    )

def make_chart(
    name: str,
    requests_time: float,
    kawa_time: float,
    httpx_time: float,
    benchmark_type: str
):
    import matplotlib.pyplot as plt

    plt.bar(
        ["kawa", "httpx", "requests"],
        [kawa_time, httpx_time, requests_time],
        width=0.25,
        color=["#D04C38", "#6FC276", "#2A7CB6"]
    )
    plt.xlabel("Library")
    plt.ylabel("Time - Seconds")
    plt.title(f"{name} - {benchmark_type}")
    plt.savefig(f".github/images/{benchmark_type}-benchmark.png")

benchmark_time_for_x_json_requests()