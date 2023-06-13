import kawa
import requests
import timeit
import json

def benchmark_time_for_x_requests():
    NUMBER = (10 ** 4) # 10k

    data = {
        "username": "justanotherbyte",
        "password": "http"
    }

    def kawa_request():
        response = kawa.request("GET", "127.0.0.1", 8080, "/", body=json.dumps(data))
        print(response)

    def requests_request():
        response = requests.request("GET", "http://127.0.0.1:8080/", json=data)
        print(response)

    kawa_time = timeit.timeit(kawa_request, number=NUMBER)
    requests_time = timeit.timeit(requests_request, number=NUMBER)

    print(f"Benchmark for: {NUMBER:,} requests:")

    print("Kawa:", kawa_time)
    print("Requests: ", requests_time)

    print("Kawa is {}x faster".format(round(requests_time / kawa_time)))

benchmark_time_for_x_requests()