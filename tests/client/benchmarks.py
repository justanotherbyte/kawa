import timeit
import time
import asyncio

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
    print("Now timing httpx") # i need this to tell the difference between the async and sync times
    httpx_time = timeit.timeit(httpx_request, number=NUMBER)

    httpx_async_client = httpx.AsyncClient()

    async def httpx_async_time() -> float:
        start = time.time()
        for _ in range(NUMBER):
            response = await httpx_async_client.request("GET", "http://127.0.0.1:8080")
            print(f"{response} async")
        end = time.time()

        await httpx_async_client.aclose()

        return end - start

    print("Event loop being retrieved")
    loop = asyncio.new_event_loop()
    print("Now timing async httpx")
    httpx_async_result = loop.run_until_complete(httpx_async_time())

    print(f"Benchmark for: {NUMBER:,} requests:")

    print("Kawa:", kawa_time)
    print("Requests: ", requests_time)
    print("HTTPX: ", httpx_time)
    print("HTTPX async:", httpx_async_result)

    print("Kawa is {}x faster than requests".format(round(requests_time / kawa_time)))
    print("Kawa is {}x faster than httpx".format(round(httpx_time / kawa_time)))
    print("Kawa is {}x faster than httpx async".format(round(httpx_async_result / kawa_time)))

    make_chart(
        name="Time - 10,000 requests",
        requests_time=requests_time,
        kawa_time=kawa_time,
        httpx_time=httpx_time,
        httpx_async_time=httpx_async_result,
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

    httpx_async_client = httpx.AsyncClient()

    async def httpx_async_time() -> float:
        start = time.time()
        for _ in range(NUMBER):
            response = await httpx_async_client.request("GET", "http://127.0.0.1:8080", json=payload)
            print(f"{response} async")
        end = time.time()

        await httpx_async_client.aclose()

        return end - start

    kawa_time = timeit.timeit(kawa_request, number=NUMBER)
    requests_time = timeit.timeit(requests_request, number=NUMBER)
    print("Now timing httpx")
    httpx_time = timeit.timeit(httpx_request, number=NUMBER)

    print("Event loop being retrieved")
    loop = asyncio.new_event_loop()
    print("Now timing async httpx")
    httpx_async_result = loop.run_until_complete(httpx_async_time())

    print(f"Benchmark for json: {NUMBER:,} requests - {payload_size} bytes size payload:")

    print("Kawa:", kawa_time)
    print("Requests: ", requests_time)
    print("HTTPX: ", httpx_time)
    print("HTTPX async: ", httpx_async_result)
    
    print("Kawa is {}x faster than requests".format(round(requests_time / kawa_time)))
    print("Kawa is {}x faster than httpx".format(round(httpx_time / kawa_time)))
    print("Kawa is {}x faster than httpx async".format(round(httpx_async_result / kawa_time)))

    make_chart(
        "Time - 10,000 requests",
        requests_time=requests_time,
        kawa_time=kawa_time,
        httpx_time=httpx_time,
        httpx_async_time=httpx_async_result,
        benchmark_type="json"
    )

def make_chart(
    name: str,
    requests_time: float,
    kawa_time: float,
    httpx_time: float,
    httpx_async_time: float,
    benchmark_type: str
):
    import matplotlib.pyplot as plt

    plt.bar(
        ["kawa", "httpx", "httpx async", "requests"],
        [kawa_time, httpx_time, httpx_async_time, requests_time],
        width=0.25,
        color=["#D04C38", "#6FC276", "#F7963F", "#2A7CB6"]
    )
    plt.xlabel("Library")
    plt.ylabel("Time - Seconds")
    plt.title(f"{name} - {benchmark_type}")
    plt.savefig(f".github/images/{benchmark_type}-benchmark.png")

benchmark_time_for_x_json_requests()